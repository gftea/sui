// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module sui::hmac_tests {
    use sui::hmac;
    use sui::digest;
    
    #[test]
    fun test_hmac_sha3_256() {
        let key = b"my key!";
        let msg = b"hello world!";
        // The next was calculated using python
        // hmac.new(key, msg, digestmod=hashlib.sha3_256).digest()
        let expected_output_bytes = x"f6d6ae02f426eb9664e89e3c6d86c60e6103ce22b916819219c26e34e8d236dc";
        let output = hmac::hmac_sha3_256(&key, &msg);
        let outout_bytes = digest::sha3_256_digest_to_bytes(&output);
        assert!(outout_bytes == expected_output_bytes, 0);
    }
}
