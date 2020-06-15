// ============================================= DECRYPT  =============================================
extern crate libsodium_sys as ffi;
extern crate std;
extern crate libc;
#[cfg(not(feature = "std"))]
use libc::c_ulonglong;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305 as box_;
/// Number of additional bytes in a ciphertext compared to the corresponding
/// plaintext.
pub const SEALBYTES: usize = ffi::crypto_box_SEALBYTES as usize;
use std::str;

use crate::serde_json;
use crate::open_file;

#[cfg(not(feature = "std"))]
use std::fs::File;
use std::io::{BufWriter, Write};
use uuid::Uuid; 
use anyhow::{Result, anyhow};
use base64;
use sodiumoxide::crypto::{
    box_::{curve25519xsalsa20poly1305::PublicKey, curve25519xsalsa20poly1305::SecretKey}
    
};

pub fn decrypt_function(FILE_NAME_UUID: &str) -> Result<()>{

    // -------------------- OPEN FILES  ---------------------
    
            let encrypted_file = open_file::open_encrypted_file(FILE_NAME_UUID);
            let passed_public_key = open_file::open_public_key();
            let passed_secret_key = open_file::open_secret_key();
    
    // -------------------- DECODE ENCODED PUBLIC KEY AND SECRET KEY ---------------------
            let pk_decoded = base64::decode(passed_public_key)?;
            //println!("decoded pk: {:?}", pk_decoded);
    
            let public_key = PublicKey::from_slice(&pk_decoded)
            .ok_or_else(|| anyhow!("unable to create public key object"))?;
            //println!("pk from slice: {:?}", public_key);
    
            let sk_decoded = base64::decode(passed_secret_key)?;
            //println!("decoded sk: {:?}", sk_decoded);
    
            let secret_key = SecretKey::from_slice(&sk_decoded)
            .ok_or_else(|| anyhow!("unable to create public key object"))?;
            //println!("sk from slice: {:?}", secret_key);
    
    // ------------------- DECRYPT  ---------------------------------
    
    
            //println!("public key: {:?}\n secret key: {:?}", public_key, secret_key);
            let opened = decrypt(encrypted_file, &public_key, &secret_key, FILE_NAME_UUID); //into_bytes(): String -> Vec
            println!("{:?} - decrypted", opened);
           // if opened == Err(()){
            //    return Err(error)
            //}
    
            Ok(())
        }
    
 pub fn decrypt(c: Vec<u8>,
                &box_::PublicKey(ref pk): &box_::PublicKey,
                &box_::SecretKey(ref sk): &box_::SecretKey,
                FILE_NAME_UUID: &str
            ) -> Result<bool, ()> {


        println!("decrypt start");
        //println!("c: {:?}\nc.len(): {:?}\nSEALBYTES: {:?}", c, c.len(), SEALBYTES);
        
        if c.len() < SEALBYTES {
            println!("error!");
            return Err(());
        }

        let mut m = vec![0u8; c.len() - SEALBYTES];

        let ret = unsafe {
            ffi::crypto_box_seal_open(
                // decrypted, ciphertext, CIPHERTEXT_LEN, recipient_pk, recipient_sk
                                    m.as_mut_ptr(),
                                    c.as_ptr(), 
                                    c.len() as c_ulonglong,
                                    pk.as_ptr() as *const u8, 
                                    sk.as_ptr() as *const u8)
        };

        //println!("ret: {:?}", ret);

        let m_to_str = str::from_utf8(&m).unwrap();
        //println!("m_to_str: {:?}", m_to_str);

        let write_file_result = write_file(m.clone(), FILE_NAME_UUID);
        println!("{:?} - write the decrypted file in local", write_file_result);

        let result = serde_json::convert_from_str(m_to_str);
        println!("{:?} - convert from str result", result);

        if ret == 0 {
            Ok(true)
        } else {
            Err(())
        }
    }


pub fn write_file(encrypted:  Vec<u8>, FILE_NAME_UUID: &str) -> std::io::Result<()>{
        
    let mut file_created = File::create(format!("C:/Users/M7/Desktop/decrypt_file/decrypted_files/{}", FILE_NAME_UUID)).expect("create file failed");

    //  write file
    file_created.write_all(&encrypted)?;

    //println!("encrypted txt file generated.");

    Ok(())

}