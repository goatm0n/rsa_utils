use std::path::PathBuf;
use rsa_rs::{
    keys::keypair::KeyPair,
    encryption::{
        encrypt::encrypt_string,
        decrypt::decrypt_string,
    },
};

pub fn get_full_path(path: &PathBuf) -> PathBuf {
    let mut full_path = PathBuf::new();
    let path_str = path.to_str().unwrap();
    if path_str.chars().nth(0).unwrap() != 'C' {
        let cwd = std::env::current_dir().unwrap();
        full_path.push(cwd);
    }
    full_path.push(path);
    return full_path;
}

pub fn write_key_pair_csv(path: PathBuf, key_pair: &KeyPair) {
    let e = key_pair.public_key().public_exponent().to_string();
    let d = key_pair.private_key().private_exponent().to_string();
    let n = key_pair.public_key().modulus().to_string();
    let mut contents = String::new();
    contents += "e,d,n";
    contents += "\n";
    contents += e.as_str();
    contents += ",";
    contents += d.as_str();
    contents += ",";
    contents += n.as_str();
    std::fs::write(path, contents).expect("error writing key_pair to file");
}

pub fn write_vec_u128(path: PathBuf, data: Vec<u128>) {
    let mut contents = String::new();
    for num in data {
        let num_string = num.to_string();
        contents += num_string.as_str();
        contents += "\t";
    }
    std::fs::write(path, contents).expect("error writing vector to file");
}


pub fn parse_key_file(path: PathBuf) -> KeyPair {
    let content = std::fs::read_to_string(&path).expect("could not read key file");
    if content.len() == 0 {panic!("empty key-file: cannot parse keys")}
    let mut line = 0;
    let mut var = 0;
    let mut e = String::new();
    let mut d = String::new();
    let mut n = String::new();
    for c in content.chars() {
        if c == '\n' {
            line += 1;
            continue;
        }
        if line == 0 {
            continue;
        }
        if line == 1 && c == ',' {
            var += 1;
            continue;
        }
        if line == 1 && c != ',' {
            match var {
                0 => e.push(c),
                1 => d.push(c),
                2 => n.push(c),
                _ => panic!("invalid key-file format")
            }
        }
    }
    let e_int:u128 = e.parse::<u128>().unwrap(); 
    let d_int:u128 = d.parse().unwrap();
    let n_int:u128 = n.parse().unwrap(); 
    return KeyPair::from(e_int, d_int, n_int);
}


pub fn read_vec_u128(path: &PathBuf) -> Vec<u128> {
    let file_content = std::fs::read_to_string(path).expect("could not read file");
    if file_content.len() == 0 {panic!("empty-file: nothing to decrypt")}
    let mut num_string = String::new();
    let mut num_vec:Vec<u128> = Vec::new();
    for c in file_content.chars() {
        if c!= '\t' {
            num_string.push(c);
        } else {
            let num:u128 = num_string.parse().unwrap();
            num_vec.push(num);
            num_string.clear();
        }
    }
    return num_vec;
}

pub fn encrypt_file(file_path: PathBuf, key_path: PathBuf) {
    let file_content = std::fs::read_to_string(&file_path).expect("could not read file");
    if file_content.len() == 0 {panic!("empty-file: nothing to encrypt")}
    let key_pair = KeyPair::generate_key_pair(65537);
    write_key_pair_csv(key_path, &key_pair);
    let public_key = key_pair.public_key();
    let enc_vec = encrypt_string(&file_content, public_key);
    write_vec_u128(file_path, enc_vec); 
}

pub fn decrypt_file(file_path: PathBuf, key_path: PathBuf) {
    let encrypted_utf8 = read_vec_u128(&file_path);
    let key_pair = parse_key_file(key_path);
    let decrypted_string = decrypt_string(&encrypted_utf8, key_pair.private_key());
    std::fs::write(file_path, decrypted_string).expect("error writing decrypted_string to file");

}



























