use rsa_utils::io::{
    encrypt_file,
    decrypt_file,
    write_key_pair_csv,
};
use std::path::PathBuf;
use rsa_rs::keys::keypair::KeyPair;

#[cfg(test)]
fn test_encrypt_decrypt_file() {
    let file_path_str = "enc_file.txt";
    let key_path_str = "key_path.txt";

    let key_path_1 = PathBuf::from(key_path_str);
    let key_path_2 = PathBuf::from(key_path_str);
    let key_path_3 = PathBuf::from(file_path_str);
    
    let file_path_1 = PathBuf::from(file_path_str);
    let file_path_2 = PathBuf::from(file_path_str);
    let file_path_3 = PathBuf::from(file_path_str); 
    let file_path_4 = PathBuf::from(key_path_str);
    let file_path_5 = PathBuf::from(file_path_str);
    
    let key_pair = KeyPair::generate_key_pair(65537);
    write_key_pair_csv(key_path_1, &key_pair);
    
    let content = "Heres a message to encrypt/decrypt";
    std::fs::write(file_path_1,content).expect("Error writing to file");
    
    encrypt_file(file_path_2, key_path_2);
    let encrypted_content = std::fs::read_to_string(file_path_3).expect("Error reading from encrypted file");
    let encrypted_content_str = encrypted_content.as_str();
    assert_ne!(content, encrypted_content_str);

    decrypt_file(file_path_4, key_path_3);
    let decrypted_content =  std::fs::read_to_string(file_path_5).expect("Error reading from encrypted file");
    let decrypted_content_str = decrypted_content.as_str();
    assert_eq!(content, decrypted_content_str);

}
