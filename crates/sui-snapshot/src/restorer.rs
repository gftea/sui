// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::reader::StateSnapshotReaderV1;
use anyhow::Result;
use fastcrypto::hash::MultisetHash;
use futures::future::AbortHandle;
use mysten_metrics::spawn_monitored_task;
use std::path::PathBuf;
use std::sync::Arc;
use sui_archival::reader::{load_summaries_upto, ArchiveReaderBalancer};
use sui_config::node::SnapshotRestoreConfig;
use sui_core::authority::authority_store_tables::AuthorityPerpetualTables;
use sui_core::authority::epoch_start_configuration::EpochStartConfiguration;
use sui_storage::hard_link;
use sui_storage::object_store::{ObjectStoreConfig, ObjectStoreType};
use sui_types::accumulator::Accumulator;
use sui_types::messages_checkpoint::{CheckpointCommitment, ECMHLiveObjectSetDigest};
use sui_types::storage::{ReadStore, WriteStore};
use sui_types::sui_system_state::epoch_start_sui_system_state::EpochStartSystemStateTrait;
use sui_types::sui_system_state::{get_sui_system_state, SuiSystemStateTrait};
use tracing::{info, warn};
use typed_store::rocks::default_db_options;

pub struct SnapshotRestorer {
    config: SnapshotRestoreConfig,
    snapshot_reader: Arc<StateSnapshotReaderV1>,
    archive_balancer: Arc<ArchiveReaderBalancer>,
    parent_db_path: PathBuf,
}

impl SnapshotRestorer {
    pub async fn new(
        config: SnapshotRestoreConfig,
        archive_balancer: Arc<ArchiveReaderBalancer>,
        parent_db_path: PathBuf,
    ) -> Result<Self> {
        let snapshot_reader = StateSnapshotReaderV1::new(
            config.epoch,
            &config.remote_store_config,
            &ObjectStoreConfig {
                object_store: Some(ObjectStoreType::File),
                directory: Some(config.local_store_dir.clone()),
                ..Default::default()
            },
            usize::MAX,
            config.download_concurrency,
        )
        .await?;

        Ok(Self {
            config,
            snapshot_reader: Arc::new(snapshot_reader),
            archive_balancer,
            parent_db_path,
        })
    }

    pub async fn run<S>(&self, store: S) -> Result<()>
    where
        S: WriteStore + Clone + Send + Sync,
        <S as ReadStore>::Error: std::error::Error + Send + Sync + 'static,
    {
        let epoch = self.config.epoch;
        let archive_latest = self
            .archive_balancer
            .get_archive_watermark()
            .await
            .expect("Failed to get archive watermark")
            .expect("Archive watermark not set");
        let archive_reader = self
            .archive_balancer
            .pick_one_random(0..archive_latest)
            .await
            .expect("Balancer failed to pick archive reader");

        let manifest = archive_reader.acquire_manifest_guard().await;
        let concurrency = archive_reader.concurrency();
        let remote_object_store = archive_reader.remote_object_store();

        // let store_ref = Arc::new(store);

        // let checkpoint_sync_handle = spawn_monitored_task!(async move {

        // });
        load_summaries_upto(
            epoch.saturating_add(1),
            store.clone(),
            manifest,
            concurrency,
            remote_object_store,
        )
        .await
        .expect("Failed to load summaries");

        let (sha3_digests, accumulator) = self.snapshot_reader.get_checksums()?;
        let (abort_handle, abort_registration) = AbortHandle::new_pair();

        let perpetual_options = default_db_options().optimize_db_for_write_throughput(4);
        let perpetual_staging_tables = Arc::new(AuthorityPerpetualTables::open(
            &self.parent_db_path.join("store_staging"),
            Some(perpetual_options.options),
        ));

        // let read_handle = spawn_monitored_task!(async move {

        // });
        self.snapshot_reader
            .read(&perpetual_staging_tables, sha3_digests, abort_registration)
            .await
            .map_err(|e| anyhow::anyhow!("{:?}", e.to_string()))
            .unwrap_or_else(|e| panic!("Failed to read snapshot from remote store: {:?}", e));

        // Upon successful exit of this task, we should have all checkpoint summaries
        // from genesis to the restore epoch, as well as the end of epoch checkpoint
        // mapping populated.
        // checkpoint_sync_handle.await?;

        if self.config.disable_verify {
            warn!("Skipping formal snapshot verification! This is not recommended for production/non-emergency use.");
        } else {
            let checkpoint_summary = store
                .get_epoch_last_checkpoint(epoch)?
                .expect("Expected last checkpoint for epoch to exist after summary sync");
            let commitments = &checkpoint_summary
                .end_of_epoch_data
                .as_ref()
                .unwrap_or_else(|| {
                    panic!("Expected end of epoch checkpoint summary to be returned")
                })
                .epoch_commitments;
            if commitments.is_empty() {
                panic!(
                    "Formal snapshot verification not supported for epoch {} on this node",
                    epoch
                );
            }

            let root_state_digest = ECMHLiveObjectSetDigest::from(accumulator.digest());
            let CheckpointCommitment::ECMHLiveObjectSetDigest(commitment_digest) = &commitments[0];
            if root_state_digest != commitment_digest.clone() {
                abort_handle.abort();
                panic!(
                    "Formal snapshot root state digest ({:?}) does not match root state digest commitment ({:?}) for epoch {}",
                    root_state_digest,
                    commitment_digest,
                    epoch,
                );
            }
            info!("Formal snapshot verification passed for epoch {}", epoch);
        }
        // read_handle.await?;

        self.setup_db_state(accumulator, perpetual_staging_tables, store)
            .await
    }

    // This function should be called once state accumulator based hash verification
    // is complete and live object set state is downloaded to local store
    async fn setup_db_state<S>(
        &self,
        accumulator: Accumulator,
        perpetual_db: Arc<AuthorityPerpetualTables>,
        store: S,
    ) -> Result<()>
    where
        S: WriteStore + Clone + Send,
        <S as ReadStore>::Error: std::error::Error + Send + Sync + 'static,
    {
        let epoch = self.config.epoch;
        let last_checkpoint = store.get_epoch_last_checkpoint(epoch)?.expect(&format!(
            "Expected last checkpoint for epoch {} to exist after summary sync",
            epoch,
        ));

        // Note that we do not update the highest verified checkpoint here, incase
        // we ran ahead during summary sync.
        store.update_highest_pruned_checkpoint(&last_checkpoint)?;
        store.update_highest_synced_checkpoint(&last_checkpoint)?;
        store.update_highest_executed_checkpoint(&last_checkpoint)?;

        let system_state_object = get_sui_system_state(&perpetual_db)?;
        let new_epoch_start_state = system_state_object.into_epoch_start_state();
        let next_epoch_committee = new_epoch_start_state.get_sui_committee();
        let epoch_start_configuration =
            EpochStartConfiguration::new(new_epoch_start_state, *last_checkpoint.digest());
        perpetual_db
            .set_epoch_start_configuration(&epoch_start_configuration)
            .await?;
        // TODO we should consolidate the perpetual store and checkpoint store watermarks,
        // as these should always agree
        perpetual_db.set_highest_pruned_checkpoint_without_wb(last_checkpoint.sequence_number)?;
        perpetual_db.insert_root_state_hash(epoch, last_checkpoint.sequence_number, accumulator)?;
        store.insert_committee(next_epoch_committee)?;

        // Hardlink staging tables to perpetual store path so that on restart, the
        // data will be available in the expected authority store directory.
        Ok(hard_link(
            self.parent_db_path.join("store_staging"),
            self.parent_db_path.join("store"),
        )?)
    }
}
