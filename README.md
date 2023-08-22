# rsa_utils

[![crates.io][crate-image]][crate-link]
[![Documentation][doc-image]][doc-link]
[![dependency status][deps-image]][deps-link]

A Rust library containing utility functions for the purpose of encrypting / decrypting files and reading / writing keyfiles. 

## Example
``` rust
use rsa_utils::io::{
    encrypt_file,
    decrypt_file,
    write_key_pair_csv,
};
use std::path::PathBuf;
use rsa_rs::keys::keypair::KeyPair;

let key_path = PathBuf::from("key_path.txt");
let file_path = PathBuf::from("file_path.txt");
let key_pair = KeyPair::generate_key_pair(65537);

write_key_pair_csv(key_path, &key_pair);
encrypt_file(file_path, key_path);
decrypt_file(file_path, key_path);

```

[//]: # (badges)

[crate-image]: https://buildstats.info/crate/rsa_utils
[crate-link]: https://crates.io/crates/rsa_utils
[doc-image]: https://docs.rs/rsa_utils/badge.svg
[doc-link]: https://docs.rs/rsa_utils
[deps-image]: https://deps.rs/repo/github/goatm0n/rsa_utils/status.svg
[deps-link]: https://deps.rs/repo/github/goatm0n/rsa_utils
