// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use crate::signature::{AuthenticatorTrait, VerifyParams};
use crate::utils::{make_transaction, make_zklogin_tx};
use crate::{
    base_types::SuiAddress,
    crypto::{get_key_pair_from_rng, DefaultHash, SignatureScheme, SuiKeyPair},
    signature::GenericSignature,
    zk_login_authenticator::{AddressParams, PublicInputs, ZkLoginAuthenticator, ZkLoginProof},
    zk_login_util::DEFAULT_JWK_BYTES,
};
use fastcrypto::encoding::{Base64, Encoding};
use fastcrypto::hash::HashFunction;
use fastcrypto::traits::ToFromBytes;
use fastcrypto_zkp::bn254::zk_login::{
    parse_jwks, AuxInputs, OAuthProvider, OAuthProviderContent, SupportedKeyClaim,
};
use rand::{rngs::StdRng, SeedableRng};
use shared_crypto::intent::{Intent, IntentMessage};

#[test]
fn zklogin_authenticator_scenarios() {
    use im::hashmap::HashMap as ImHashMap;
    let (user_address, tx, authenticator) = make_zklogin_tx();

    let intent_msg = IntentMessage::new(
        Intent::sui_transaction(),
        tx.into_data().transaction_data().clone(),
    );

    let parsed: ImHashMap<(String, String), OAuthProviderContent> =
        parse_jwks(DEFAULT_JWK_BYTES, OAuthProvider::Google).unwrap().into_iter().collect();

    // Construct the required info required to verify a zk login authenticator
    // in authority server (i.e. epoch and default JWK).
    let aux_verify_data = VerifyParams::new(parsed.clone());

    let x = authenticator
    .verify_authenticator(&intent_msg, user_address, Some(0), &aux_verify_data);
    println!("x: {:?}", x);
    // Verify passes.
    // this is failing till we use a new nonce
    assert!(x.is_ok());

    let parsed: ImHashMap<(String, String), OAuthProviderContent> = parsed
        .into_iter()
        .enumerate()
        .map(|(i, ((_, _), v))| ((format!("nosuchkey_{}", i), "".to_string()), v))
        .collect();

    // correct kid can no longer be found
    let aux_verify_data = VerifyParams::new(parsed);

    // Verify fails.
    assert!(authenticator
        .verify_authenticator(&intent_msg, user_address, Some(9999), &aux_verify_data)
        .is_err());
}

// #[test]
// fn test_serde_zk_login_signature() {
//     let user_key: SuiKeyPair =
//         SuiKeyPair::Ed25519(get_key_pair_from_rng(&mut StdRng::from_seed([0; 32])).1);

//     let proof = ZkLoginProof::from_json("{\"pi_a\":[\"14773032106069856668972396672350797076055861237383576052493955423994116090912\",\"10256481106886434602492891208282170223983625600287748851653074125391248309014\",\"1\"],\"pi_b\":[[\"11279307737533893899063186106984511177705318223976679158125470783510400431153\",\"10750728891610181825635823254337402092259745577964933136615591108170079085462\"],[\"5103373414836661910221794877118021615720837981629628049907178132389261290989\",\"14681020929085169399218842877266984710651553119645243809601357658254293671214\"],[\"1\",\"0\"]],\"pi_c\":[\"9289872310426503554963895690721473724352074264768900407255162398689856598144\",\"11987870476480102771637928245246115838352121746691800889675428205478757122604\",\"1\"],\"protocol\":\"groth16\"}").unwrap();
//     let aux_inputs = AuxInputs::from_json("{\"claims\": [{\"name\": \"iss\",\"value_base64\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"index_mod_4\": 1},{\"name\": \"aud\",\"value_base64\": \"CJhdWQiOiI1NzU1MTkyMDQyMzctbXNvcDllcDQ1dTJ1bzk4aGFwcW1uZ3Y4ZDg0cWRjOGsuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLC\",\"index_mod_4\": 1}], \"header_base64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImM5YWZkYTM2ODJlYmYwOWViMzA1NWMxYzRiZDM5Yjc1MWZiZjgxOTUiLCJ0eXAiOiJKV1QifQ\",\"addr_seed\": \"15604334753912523265015800787270404628529489918817818174033741053550755333691\",\"eph_public_key\": [\"17932473587154777519561053972421347139\", \"134696963602902907403122104327765350261\"],\"max_epoch\": 10000,\"key_claim_name\": \"sub\",\"modulus\": \"24501106890748714737552440981790137484213218436093327306276573863830528169633224698737117584784274166505493525052788880030500250025091662388617070057693555892212025614452197230081503494967494355047321073341455279175776092624566907541405624967595499832566905567072654796017464431878680118805774542185299632150122052530877100261682728356139724202453050155758294697161107805717430444408191365063957162605112787073991150691398970840390185880092832325216009234084152827135531285878617366639283552856146367480314853517993661640450694829038343380576312039548353544096265483699391507882147093626719041048048921352351403884619\"}").unwrap().init().unwrap();
//     let public_inputs = PublicInputs::from_json(
//         "[\"6049184272607241856912886413680599526372437331989542437266935645748489874658\"]",
//     )
//     .unwrap();

//     let mut hasher = DefaultHash::default();
//     hasher.update([SignatureScheme::ZkLoginAuthenticator.flag()]);
//     let address_params = AddressParams::new(
//         OAuthProvider::Google.get_config().0.to_owned(),
//         SupportedKeyClaim::Sub.to_string(),
//     );
//     hasher.update(bcs::to_bytes(&address_params).unwrap());
//     // hasher.update(big_int_str_to_bytes(aux_inputs.get_address_seed()));
//     let user_address = SuiAddress::from_bytes(hasher.finalize().digest).unwrap();

//     // Sign the user transaction with the user's ephemeral key.
//     let tx = make_transaction(user_address, &user_key, Intent::sui_transaction());
//     let s = match tx.inner().tx_signatures.first().unwrap() {
//         GenericSignature::Signature(s) => s,
//         _ => panic!("Expected a signature"),
//     };

//     // Construct the authenticator with all user submitted components.
//     let authenticator = GenericSignature::ZkLoginAuthenticator(ZkLoginAuthenticator::new(
//         proof,
//         public_inputs,
//         aux_inputs,
//         s.clone(),
//     ));

//     let serialized = authenticator.as_ref();
//     let deserialized = GenericSignature::from_bytes(serialized).unwrap();
//     assert_eq!(deserialized, authenticator);

//     let user_address: SuiAddress = (&sig).try_into().unwrap();
//     assert_eq!(
//         user_address,
//         SuiAddress::from_str("0xd5cb3a8f365cf9e0486df1675538fa7b95875e1ef3ebbe1d392e163f0aa12a11")
//             .unwrap()
//     );
// }
