extern crate libsodium_sys as ffi;
extern crate std;
extern crate libc;
#[cfg(not(feature = "std"))]
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Read;

pub fn open_encrypted_file(FILE_NAME_UUID: &str) -> Vec<u8> {
    // specify the filename and its path to open and read   -- just for now, so just focus on encrypt and decrypt the file - later work on recieve the file I guess?
    let filename = format!("C:/Users/M7/Desktop/decrypt_file/downloaded_encrypted_files/{}", FILE_NAME_UUID);
    let file = File::open(filename.clone()).unwrap();
    let reader = BufReader::new(file);

    let mut file_content = Vec::new();
    let mut file = File::open(&filename.clone()).expect("Unable to open file");
    file.read_to_end(&mut file_content).expect("Unable to read");

    file_content
}

pub fn open_public_key() -> String {
    let filename = "C:/Users/M7/Desktop/encrypt_file/public_key/pk";

    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut publickey = "public key".to_string();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let pk = line.unwrap(); // Ignore errors.

        publickey = pk.replace("\"", "");
       // println!("public key @open_public_key(): {:?}", publickey);

    }

    return publickey;
}

pub fn open_secret_key() -> String {
    // specify the filename and its path to open and read   -- just for now, so just focus on encrypt and decrypt the file - later work on recieve the file I guess?
    let filename = "C:/Users/M7/Desktop/encrypt_file/secret_key/sk";

    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut secretkey = "secret key".to_string();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let mut sk = line.unwrap(); // Ignore errors.

        secretkey = sk.replace("\"", "");
      //  println!("secret key  : {:?}", secretkey);
    }

    return secretkey;
}