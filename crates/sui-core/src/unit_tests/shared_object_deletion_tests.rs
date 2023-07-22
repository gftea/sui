use std::sync::Arc;

use sui_types::{
    base_types::{ObjectID, ObjectRef, SequenceNumber, SuiAddress},
    crypto::{get_key_pair, AccountKeyPair},
    effects::TransactionEffects,
    object::Object,
    programmable_transaction_builder::ProgrammableTransactionBuilder,
    transaction::{
        CertifiedTransaction, ProgrammableTransaction, Transaction, TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
    },
};

use crate::authority::authority_test_utils::execute_sequenced_certificate_to_effects;
use crate::authority::authority_tests::certify_shared_obj_transaction_no_execution;
use crate::{
    authority::{
        authority_tests::{build_programmable_transaction, execute_programmable_transaction},
        move_integration_tests::build_and_publish_test_package,
        test_authority_builder::TestAuthorityBuilder,
        AuthorityState,
    },
    move_call,
};
use move_core_types::ident_str;
use sui_protocol_config::ProtocolConfig;
use sui_types::effects::TransactionEffectsV1;
use sui_types::error::{ExecutionError, SuiError};
use sui_types::transaction::{ObjectArg, VerifiedCertificate};

pub struct TestRunner {
    pub sender: SuiAddress,
    pub sender_key: AccountKeyPair,
    pub gas_object_ids: Vec<ObjectID>,
    pub authority_state: Arc<AuthorityState>,
    pub package: ObjectRef,
}

impl TestRunner {
    pub async fn new(base_package_name: &str) -> Self {
        telemetry_subscribers::init_for_testing();
        let (sender, sender_key): (_, AccountKeyPair) = get_key_pair();

        let mut protocol_config = ProtocolConfig::get_for_max_version();
        protocol_config.set_shared_object_deletion_for_testing();
        let authority_state = TestAuthorityBuilder::new()
            .with_protocol_config(protocol_config)
            .build()
            .await;

        let mut gas_object_ids = vec![];
        for _ in 0..5 {
            let gas_object_id = ObjectID::random();
            let gas_object = Object::with_id_owner_for_testing(gas_object_id, sender);
            authority_state.insert_genesis_object(gas_object).await;
            gas_object_ids.push(gas_object_id);
        }

        let package = build_and_publish_test_package(
            &authority_state,
            &sender,
            &sender_key,
            &gas_object_ids[0],
            base_package_name,
            false,
        )
        .await;

        Self {
            sender,
            sender_key,
            gas_object_ids,
            authority_state,
            package,
        }
    }

    pub async fn create_shared_object(&mut self) -> TransactionEffectsV1 {
        let TransactionEffects::V1(effects) = self
            .execute_owned_transaction({
                let mut builder = ProgrammableTransactionBuilder::new();
                move_call! {
                    builder,
                    (self.package.0)::o2::create()
                };
                builder.finish()
            })
            .await;
        effects
    }

    pub async fn execute_owned_transaction(
        &mut self,
        pt: ProgrammableTransaction,
    ) -> TransactionEffects {
        let effects = execute_programmable_transaction(
            &self.authority_state,
            &self.gas_object_ids[0],
            &self.sender,
            &self.sender_key,
            pt,
            TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        )
        .await
        .unwrap();

        effects
    }

    pub async fn delete_shared_obj_tx(
        &mut self,
        shared_obj_id: ObjectID,
        initial_shared_version: SequenceNumber,
    ) -> Transaction {
        let mut delete_object_transaction_builder = ProgrammableTransactionBuilder::new();
        let arg = delete_object_transaction_builder
            .obj(ObjectArg::SharedObject {
                id: shared_obj_id,
                initial_shared_version,
                mutable: true,
            })
            .unwrap();
        move_call! {
            delete_object_transaction_builder,
            (self.package.0)::o2::consume_o2(arg)
        };
        let delete_obj_tx = delete_object_transaction_builder.finish();
        self.create_signed_transaction_from_pt(delete_obj_tx, 0)
            .await
    }

    pub async fn mutate_shared_obj_tx(
        &mut self,
        shared_obj_id: ObjectID,
        initial_shared_version: SequenceNumber,
    ) -> Transaction {
        let mut delete_object_transaction_builder = ProgrammableTransactionBuilder::new();
        let arg = delete_object_transaction_builder
            .obj(ObjectArg::SharedObject {
                id: shared_obj_id,
                initial_shared_version,
                mutable: true,
            })
            .unwrap();
        move_call! {
            delete_object_transaction_builder,
            (self.package.0)::o2::mutate_o2(arg)
        };
        let delete_obj_tx = delete_object_transaction_builder.finish();
        self.create_signed_transaction_from_pt(delete_obj_tx, 0)
            .await
    }

    pub async fn create_signed_transaction_from_pt(
        &mut self,
        pt: ProgrammableTransaction,
        account_id: usize,
    ) -> Transaction {
        build_programmable_transaction(
            &self.authority_state,
            &self.gas_object_ids[account_id],
            &self.sender,
            &self.sender_key,
            pt,
            TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        )
        .await
        .unwrap()
    }

    pub async fn certify_shared_obj_transaction(
        &mut self,
        tx: Transaction,
    ) -> Result<VerifiedCertificate, SuiError> {
        certify_shared_obj_transaction_no_execution(&self.authority_state, tx).await
    }

    pub async fn execute_sequenced_certificate_to_effects(
        &mut self,
        certificate: VerifiedCertificate,
    ) -> Result<
        (
            CertifiedTransaction,
            TransactionEffects,
            Option<ExecutionError>,
        ),
        SuiError,
    > {
        execute_sequenced_certificate_to_effects(&self.authority_state, certificate).await
    }
}

#[tokio::test]
async fn test_delete_shared_object() {
    let mut runner = TestRunner::new("shared_object_deletion").await;
    let effects = runner.create_shared_object().await;

    assert_eq!(effects.created.len(), 1);

    let shared_obj = effects.created[0].0;
    let shared_obj_id = shared_obj.0;
    let initial_shared_version = shared_obj.1;
    let delete_obj_tx = runner
        .delete_shared_obj_tx(shared_obj_id, initial_shared_version)
        .await;

    let cert = runner
        .certify_shared_obj_transaction(delete_obj_tx)
        .await
        .unwrap();

    let (_executed, TransactionEffects::V1(effects), error) = runner
        .execute_sequenced_certificate_to_effects(cert)
        .await
        .unwrap();

    assert!(error.is_none());

    assert_eq!(effects.deleted.len(), 1);
    assert_eq!(effects.shared_objects.len(), 1);

    // assert the shared object was deleted
    let deleted_obj_id = effects.deleted[0].0;
    let shared_obj_id = effects.shared_objects[0].0;
    assert_eq!(deleted_obj_id, shared_obj_id);

    // assert the version of the deleted shared object was incremented
    let deleted_obj_ver = effects.deleted[0].1;
    let shared_obj_ver = effects.shared_objects[0].1;
    assert!(shared_obj_ver > deleted_obj_ver);

    // assert the rest of the effects are as expected
    assert!(effects.status.is_ok());
    assert!(effects.created.is_empty());
    assert!(effects.unwrapped_then_deleted.is_empty());
    assert!(effects.wrapped.is_empty());
}

#[tokio::test]
async fn test_mutate_after_delete() {
    let mut runner = TestRunner::new("shared_object_deletion").await;
    let effects = runner.create_shared_object().await;

    assert_eq!(effects.created.len(), 1);

    let shared_obj = effects.created[0].0;
    let shared_obj_id = shared_obj.0;
    let initial_shared_version = shared_obj.1;
    let delete_obj_tx = runner
        .delete_shared_obj_tx(shared_obj_id, initial_shared_version)
        .await;

    let cert = runner
        .certify_shared_obj_transaction(delete_obj_tx)
        .await
        .unwrap();

    let (_executed, TransactionEffects::V1(effects), error) = runner
        .execute_sequenced_certificate_to_effects(cert)
        .await
        .unwrap();
}
