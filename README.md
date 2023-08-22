# rsa_utils

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
