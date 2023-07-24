// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use fastcrypto_zkp::bn254::zk_login::OAuthProvider;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Used in tests or anywhere that fetching up to date JWKs is not possible.
pub const DEFAULT_JWK_BYTES: &[u8] = r#"{
    "keys": [
        {
          "kty": "RSA",
          "e": "AQAB",
          "alg": "RS256",
          "kid": "c9afda3682ebf09eb3055c1c4bd39b751fbf8195",
          "use": "sig",
          "n": "whYOFK2Ocbbpb_zVypi9SeKiNUqKQH0zTKN1-6fpCTu6ZalGI82s7XK3tan4dJt90ptUPKD2zvxqTzFNfx4HHHsrYCf2-FMLn1VTJfQazA2BvJqAwcpW1bqRUEty8tS_Yv4hRvWfQPcc2Gc3-_fQOOW57zVy-rNoJc744kb30NjQxdGp03J2S3GLQu7oKtSDDPooQHD38PEMNnITf0pj-KgDPjymkMGoJlO3aKppsjfbt_AH6GGdRghYRLOUwQU-h-ofWHR3lbYiKtXPn5dN24kiHy61e3VAQ9_YAZlwXC_99GGtw_NpghFAuM4P1JDn0DppJldy3PGFC0GfBCZASw"
        },
        {
          "alg": "RS256",
          "use": "sig",
          "n": "1qrQCTst3RF04aMC9Ye_kGbsE0sftL4FOtB_WrzBDOFdrfVwLfflQuPX5kJ-0iYv9r2mjD5YIDy8b-iJKwevb69ISeoOrmL3tj6MStJesbbRRLVyFIm_6L7alHhZVyqHQtMKX7IaNndrfebnLReGntuNk76XCFxBBnRaIzAWnzr3WN4UPBt84A0KF74pei17dlqHZJ2HB2CsYbE9Ort8m7Vf6hwxYzFtCvMCnZil0fCtk2OQ73l6egcvYO65DkAJibFsC9xAgZaF-9GYRlSjMPd0SMQ8yU9i3W7beT00Xw6C0FYA9JAYaGaOvbT87l_6ZkAksOMuvIPD_jNVfTCPLQ==",
          "e": "AQAB",
          "kty": "RSA",
          "kid": "6083dd5981673f661fde9dae646b6f0380a0145c"
        }
      ]
  }"#.as_bytes();

/// A whitelist of client_ids (i.e. the value of "aud" in cliams) for each provider
pub static DEFAULT_WHITELIST: Lazy<HashMap<&str, Vec<&str>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        OAuthProvider::Google.get_config().0,
        vec!["946731352276-pk5glcg8cqo38ndb39h7j093fpsphusu.apps.googleusercontent.com"],
    );
    map.insert(
        OAuthProvider::Twitch.get_config().0,
        vec!["d31icql6l8xzpa7ef31ztxyss46ock"],
    );
    map
});

/// Parameters for generating an address.
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressParams {
    iss: String,
    key_claim_name: String,
}

impl AddressParams {
    pub fn new(iss: String, key_claim_name: String) -> Self {
        Self {
            iss,
            key_claim_name,
        }
    }
}
