/* --------------------- HOW IT WORKS ---------------------------
copy the encrypted file from amazon s3 BUCKETNAME folder /holding to folder /backup 
-> download the encrypted file from /holding folder to local folder
-> decrypt the encrypted file which we just downloaded 
-> once it's decrypted, delete the encrypted file in /holding folder in aws s3. file still exists in /backup folder but removed from /holding folder once it's decrypted
*/
extern crate libsodium_sys as ffi;
extern crate std;
extern crate libc;
extern crate s3;
use futures::executor::block_on;

#[cfg(not(feature = "std"))]

mod open_file;
mod decrypt;
mod serde_json;
mod aws_s3;

pub const SEALBYTES: usize = ffi::crypto_box_SEALBYTES as usize;


    pub fn main(){
        let ran = aws_s3::aws_s3_func();
        block_on(ran);
        println!("Ran the program");
    }



    