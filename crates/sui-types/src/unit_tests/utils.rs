// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::crypto::{Signer, SuiKeyPair};
use crate::multisig::{MultiSig, MultiSigPublicKey};
use crate::programmable_transaction_builder::ProgrammableTransactionBuilder;
use crate::transaction::{SenderSignedData, TEST_ONLY_GAS_UNIT_FOR_TRANSFER};
use crate::SuiAddress;
use crate::{
    base_types::{dbg_addr, ExecutionDigests, ObjectID},
    committee::Committee,
    crypto::{
        get_key_pair, get_key_pair_from_rng, AccountKeyPair, AuthorityKeyPair,
        AuthorityPublicKeyBytes, DefaultHash, Signature, SignatureScheme,
    },
    gas::GasCostSummary,
    messages_checkpoint::{
        CertifiedCheckpointSummary, CheckpointContents, CheckpointSummary, SignedCheckpointSummary,
    },
    object::Object,
    signature::GenericSignature,
    transaction::{Transaction, TransactionData},
    zk_login_authenticator::ZkLoginAuthenticator,
    zk_login_util::AddressParams,
};
use fastcrypto::ed25519::Ed25519KeyPair;
use fastcrypto::encoding::{Base64, Encoding};
use fastcrypto::hash::HashFunction;
use fastcrypto::traits::KeyPair as KeypairTraits;
use fastcrypto::traits::ToFromBytes;
use fastcrypto_zkp::bn254::zk_login::{
    AuxInputs, OAuthProvider, PublicInputs, SupportedKeyClaim, ZkLoginProof,
};
use rand::rngs::StdRng;
use rand::SeedableRng;
use shared_crypto::intent::{Intent, IntentMessage};
use std::collections::BTreeMap;

pub fn make_committee_key<R>(rand: &mut R) -> (Vec<AuthorityKeyPair>, Committee)
where
    R: rand::CryptoRng + rand::RngCore,
{
    make_committee_key_num(4, rand)
}

pub fn make_committee_key_num<R>(num: usize, rand: &mut R) -> (Vec<AuthorityKeyPair>, Committee)
where
    R: rand::CryptoRng + rand::RngCore,
{
    let mut authorities: BTreeMap<AuthorityPublicKeyBytes, u64> = BTreeMap::new();
    let mut keys = Vec::new();

    for _ in 0..num {
        let (_, inner_authority_key): (_, AuthorityKeyPair) = get_key_pair_from_rng(rand);
        authorities.insert(
            /* address */ AuthorityPublicKeyBytes::from(inner_authority_key.public()),
            /* voting right */ 1,
        );
        keys.push(inner_authority_key);
    }

    let committee = Committee::new_for_testing_with_normalized_voting_power(0, authorities);
    (keys, committee)
}

// Creates a fake sender-signed transaction for testing. This transaction will
// not actually work.
pub fn create_fake_transaction() -> Transaction {
    let (sender, sender_key): (_, AccountKeyPair) = get_key_pair();
    let recipient = dbg_addr(2);
    let object_id = ObjectID::random();
    let object = Object::immutable_with_id_for_testing(object_id);
    let pt = {
        let mut builder = ProgrammableTransactionBuilder::new();
        builder.transfer_sui(recipient, None);
        builder.finish()
    };
    let data = TransactionData::new_programmable(
        sender,
        vec![object.compute_object_reference()],
        pt,
        TEST_ONLY_GAS_UNIT_FOR_TRANSFER, // gas price is 1
        1,
    );
    to_sender_signed_transaction(data, &sender_key)
}

fn make_transaction_data(sender: SuiAddress) -> TransactionData {
    let object = Object::immutable_with_id_for_testing(ObjectID::random_from_rng(
        &mut StdRng::from_seed([0; 32]),
    ));
    let pt = {
        let mut builder = ProgrammableTransactionBuilder::new();
        builder.transfer_sui(dbg_addr(2), None);
        builder.finish()
    };
    TransactionData::new_programmable(
        sender,
        vec![object.compute_object_reference()],
        pt,
        TEST_ONLY_GAS_UNIT_FOR_TRANSFER, // gas price is 1
        1,
    )
}

/// Make a user signed transaction with the given sender and its keypair. This
/// is not verified or signed by authority.
pub fn make_transaction(sender: SuiAddress, kp: &SuiKeyPair, intent: Intent) -> Transaction {
    let data = make_transaction_data(sender);
    Transaction::from_data_and_signer(data, intent, vec![kp])
}

// This is used to sign transaction with signer using default Intent.
pub fn to_sender_signed_transaction(
    data: TransactionData,
    signer: &dyn Signer<Signature>,
) -> Transaction {
    to_sender_signed_transaction_with_multi_signers(data, vec![signer])
}

pub fn to_sender_signed_transaction_with_multi_signers(
    data: TransactionData,
    signers: Vec<&dyn Signer<Signature>>,
) -> Transaction {
    Transaction::from_data_and_signer(data, Intent::sui_transaction(), signers)
}

pub fn mock_certified_checkpoint<'a>(
    keys: impl Iterator<Item = &'a AuthorityKeyPair>,
    committee: Committee,
    seq_num: u64,
) -> CertifiedCheckpointSummary {
    let contents = CheckpointContents::new_with_causally_ordered_transactions(
        [ExecutionDigests::random()].into_iter(),
    );

    let summary = CheckpointSummary::new(
        committee.epoch,
        seq_num,
        0,
        &contents,
        None,
        GasCostSummary::default(),
        None,
        0,
    );

    let sign_infos: Vec<_> = keys
        .map(|k| {
            let name = k.public().into();

            SignedCheckpointSummary::sign(committee.epoch, &summary, k, name)
        })
        .collect();

    CertifiedCheckpointSummary::new(summary, sign_infos, &committee).expect("Cert is OK")
}

mod zk_login {
    use super::*;

    fn get_proof() -> ZkLoginProof {
        thread_local! {
            static PROOF: ZkLoginProof = ZkLoginProof::from_json("{\"pi_a\":[\"21079899190337156604543197959052999786745784780153100922098887555507822163222\",\"4490261504756339299022091724663793329121338007571218596828748539529998991610\",\"1\"],\"pi_b\":[[\"9379167206161123715528853149920855132656754699464636503784643891913740439869\",\"15902897771112804794883785114808675393618430194414793328415185511364403970347\"],[\"16152736996630746506267683507223054358516992879195296708243566008238438281201\",\"15230917601041350929970534508991793588662911174494137634522926575255163535339\"],[\"1\",\"0\"]],\"pi_c\":[\"8242734018052567627683363270753907648903210541694662698981939667442011573249\",\"1775496841914332445297048246214170486364407018954976081505164205395286250461\",\"1\"],\"protocol\":\"groth16\"}").unwrap();
        }
        PROOF.with(|p| p.clone())
    }

    fn get_aux_inputs() -> AuxInputs {
        thread_local! {
            static AUX_INPUTS: AuxInputs = AuxInputs::from_json("{\"claims\": [{\"name\": \"iss\",\"value_base64\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"index_mod_4\": 1},{\"name\": \"aud\",\"value_base64\": \"CJhdWQiOiI1NzU1MTkyMDQyMzctbXNvcDllcDQ1dTJ1bzk4aGFwcW1uZ3Y4ZDg0cWRjOGsuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLC\",\"index_mod_4\": 1}], \"header_base64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImM5YWZkYTM2ODJlYmYwOWViMzA1NWMxYzRiZDM5Yjc1MWZiZjgxOTUiLCJ0eXAiOiJKV1QifQ\",\"addr_seed\": \"15604334753912523265015800787270404628529489918817818174033741053550755333691\",\"max_epoch\": 10000,\"key_claim_name\": \"sub\",\"modulus\": \"24501106890748714737552440981790137484213218436093327306276573863830528169633224698737117584784274166505493525052788880030500250025091662388617070057693555892212025614452197230081503494967494355047321073341455279175776092624566907541405624967595499832566905567072654796017464431878680118805774542185299632150122052530877100261682728356139724202453050155758294697161107805717430444408191365063957162605112787073991150691398970840390185880092832325216009234084152827135531285878617366639283552856146367480314853517993661640450694829038343380576312039548353544096265483699391507882147093626719041048048921352351403884619\"}").unwrap().init().unwrap();
        }

        AUX_INPUTS.with(|a| a.clone())
    }

    fn get_public_inputs() -> PublicInputs {
        thread_local! {
            static PUBLIC_INPUTS: PublicInputs = PublicInputs::from_json(
                "[\"6049184272607241856912886413680599526372437331989542437266935645748489874658\"]",
            )
            .unwrap();
        }
        PUBLIC_INPUTS.with(|p| p.clone())
    }

    pub fn get_zklogin_user_address() -> SuiAddress {
        thread_local! {
            static USER_ADDRESS: SuiAddress = {
                // Derive user address manually: Blake2b_256 hash of [zklogin_flag || address seed in bytes || bcs bytes of AddressParams])
                let mut hasher = DefaultHash::default();
                hasher.update([SignatureScheme::ZkLoginAuthenticator.flag()]);
                let address_params = AddressParams::new(
                    OAuthProvider::Google.get_config().0.to_owned(),
                    SupportedKeyClaim::Sub.to_string(),
                );
                hasher.update(bcs::to_bytes(&address_params).unwrap());
                // hasher.update(big_int_str_to_bytes(get_aux_inputs().get_address_seed()));
                SuiAddress::from_bytes(hasher.finalize().digest).unwrap()
            };
        }
        USER_ADDRESS.with(|a| *a)
    }

    fn get_zklogin_user_key() -> SuiKeyPair {
        SuiKeyPair::Ed25519(
            Ed25519KeyPair::from_bytes(
                &Base64::decode("a3R0jvXpEziZLHsbX1DogdyGm8AK87HScEK+JJHwaV8=").unwrap(),
            )
            .unwrap(),
        )
        // SuiKeyPair::Ed25519(Ed25519KeyPair::generate(&mut StdRng::from_seed([0; 32])))
    }

    pub fn make_zklogin_tx() -> (SuiAddress, Transaction, GenericSignature) {
        let data = make_transaction_data(get_zklogin_user_address());

        sign_zklogin_tx(data)
    }

    pub fn sign_zklogin_tx(data: TransactionData) -> (SuiAddress, Transaction, GenericSignature) {
        // Sign the user transaction with the user's ephemeral key.
        //let tx = make_transaction(user_address, &user_key, Intent::sui_transaction());

        let tx = Transaction::from_data_and_signer(
            data,
            Intent::sui_transaction(),
            vec![&get_zklogin_user_key()],
        );

        let s = match tx.inner().tx_signatures.first().unwrap() {
            GenericSignature::Signature(s) => s,
            _ => panic!("Expected a signature"),
        };

        // Construct the authenticator with all user submitted components.
        let authenticator = GenericSignature::ZkLoginAuthenticator(ZkLoginAuthenticator::new(
            get_proof(),
            get_public_inputs(),
            get_aux_inputs(),
            s.clone(),
        ));

        let tx = Transaction::new(SenderSignedData::new(
            tx.transaction_data().clone(),
            Intent::sui_transaction(),
            vec![authenticator.clone()],
        ));

        (get_zklogin_user_address(), tx, authenticator)
    }
}

pub fn keys() -> Vec<SuiKeyPair> {
    let mut seed = StdRng::from_seed([0; 32]);
    let kp1: SuiKeyPair = SuiKeyPair::Ed25519(get_key_pair_from_rng(&mut seed).1);
    let kp2: SuiKeyPair = SuiKeyPair::Secp256k1(get_key_pair_from_rng(&mut seed).1);
    let kp3: SuiKeyPair = SuiKeyPair::Secp256r1(get_key_pair_from_rng(&mut seed).1);
    vec![kp1, kp2, kp3]
}

pub fn make_upgraded_multisig_tx() -> Transaction {
    let keys = keys();
    let pk1 = &keys[0].public();
    let pk2 = &keys[1].public();
    let pk3 = &keys[2].public();

    let multisig_pk = MultiSigPublicKey::new(
        vec![pk1.clone(), pk2.clone(), pk3.clone()],
        vec![1, 1, 1],
        2,
    )
    .unwrap();
    let addr = SuiAddress::from(&multisig_pk);
    let tx = make_transaction(addr, &keys[0], Intent::sui_transaction());

    let msg = IntentMessage::new(Intent::sui_transaction(), tx.transaction_data().clone());
    let sig1 = Signature::new_secure(&msg, &keys[0]);
    let sig2 = Signature::new_secure(&msg, &keys[1]);

    // Any 2 of 3 signatures verifies ok.
    let multi_sig1 = MultiSig::combine(vec![sig1, sig2], multisig_pk).unwrap();
    Transaction::new(SenderSignedData::new(
        tx.transaction_data().clone(),
        Intent::sui_transaction(),
        vec![GenericSignature::MultiSig(multi_sig1)],
    ))
}

pub use zk_login::*;
