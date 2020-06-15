use s3::bucket::Bucket;
use s3::credentials::Credentials;
use s3::serde_types::ListBucketResult;
use crate::decrypt;
use std::fs;
use std::str;
use std::fs::File;
use std::io::Write;
use rusoto_core::Region;
use rusoto_s3::{CopyObjectRequest, S3Client,  S3};
use tokio::runtime::Runtime;
type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

const BUCKET_NAME: &str = "YOUR BUCKET NAME";
pub const STATUS_CODE: u32 = 200;
pub const STATUS_CODE_DEL: u32 = 204;
pub const FILE_NAME_UUID: &str  = "710f5eca-010c-472e-9260-afc70dfd7a60";


struct TestS3Client {
    region: Region,
    s3: S3Client,
    bucket_name: String,
    // This flag signifies whether this bucket was already deleted as part of a test
    bucket_deleted: bool,
}

impl TestS3Client {
    // construct S3 testing client
    fn new(bucket_name: String) -> TestS3Client {
        let region = Region::UsEast2;
        TestS3Client {
            region: region.to_owned(),
            s3: S3Client::new(region),
            bucket_name: bucket_name.to_owned(),
            bucket_deleted: false,
        }
    }
}

// inititializes logging
fn init_logging() {
    let _ = env_logger::try_init();
}

pub async  fn aws_s3_func() -> Result<()> {
    println!("1");

    init_logging();
// -------------------- AWS S3 PROFILE INFO---------------------

    let bucket_name = "YOUR BUCKET NAME";
    let region = "us-east-2".parse().unwrap();
    let access_key = "YOUR ACCESS KEY";
    let secret_key = "YOUR SECRET KEY";
// -------------------- CREDENTIALS ---------------------
    
    let credentials = Credentials::new(access_key, secret_key, None); //new(access_key: &str, secret_key: &str, token: Option<&str>) -> Credentials
    println!("2");
// -------------------- BUCKET ---------------------

    let bucket = Bucket::new(bucket_name, region, credentials);
    println!("3");
// -------------------- LIST OBJECTS IN HOLDING---------------------
    list_objects_in_holding(bucket.clone());
    println!("4");
// -------------------- COPY FILE FROM HOLDING TO BACKUP ---------------------
    let test_client = TestS3Client::new(bucket_name.to_string());
    println!("5");

    let copy_file_from_holding_to_backup_result = copy_file_from_holding_to_backup(&test_client.s3);
    
    Runtime::new()
        .expect("Failed to create Tokio runtime")
        .block_on(copy_file_from_holding_to_backup_result);

// -------------------- DOWNLOAD ENCRYPTED FILE ---------------------
println!("6");

let encrypt_file_downloaded = download_from_holding_to_local(bucket.clone());
match encrypt_file_downloaded {
    Ok(downloaded) =>{
        println!("Success! - encrypt file download");
// -------------------- DECRYPT THE FILE ---------------------
        let decrypted = decrypt::decrypt_function(FILE_NAME_UUID);

// -------------------- ONCE DECRYPTED, DELETE THE FILE FROM /HOLDING  ---------------------
            match decrypted {
                Ok(decrypted) => {
                    println!("Success! - Decrypt");
                            let holding_deleted = delete_file_in_s3_holding(bucket.clone());
                            
                            match holding_deleted {
                                Ok(holding_deleted) => {
                                    println!("Success! - Delete the file in the holding folder");
// -------------------- DELETE DOWNLOADED ENCRYPTED FILE ---------------------
                                    let local_deleted = delete_file_in_local();
                                    println!("{:?} - Delete the file in the local downloaded folder", local_deleted);
                                }
                                Err(_err) => println!("! Failed to delete the file in the holding folder. Error msg: {:?}", _err)
                            }
                }
                Err(_err) => println!("!Failed to decrypt the file. Error msg: {:?}", _err)
            }   // match decrypted
    }

    Err(_err) => println!("!Failed to download encrypted file from aws s3 /holding")

    } // match encrypt_file_downloaded


    Ok(())
}

pub fn list_objects_in_holding(bucket: Bucket){
    let results: Vec<(ListBucketResult, u32)> = bucket.list("holding/", None).unwrap();

    for (ListBucketResult, code) in results {
        assert_eq!(STATUS_CODE, code);
        println!("num of objects in the bucket: {:?}", ListBucketResult.contents.len());
        println!("bucket name: {:?}", ListBucketResult.name);
        println!("objects in holding folder: {:?}", ListBucketResult.contents);
        let uuid = &ListBucketResult.contents[1].key;
       // println!("uuid: {:?}", uuid);
        //FILE_NAME_UUID = uuid;
    }
}

pub fn write_file_in_local(body: Vec<u8>) -> Result<()> {

    let filepath_to_download_in_local = format!("C:/Users/M7/Desktop/decrypt_file/downloaded_encrypted_files/{}", FILE_NAME_UUID);
    let mut file_created = File::create(filepath_to_download_in_local).expect("create file failed");
    file_created.write_all(&body).expect("write encrypted file in local /decrypt_file/downloaded_encrypted_files failed");
    
    Ok(())
}

pub fn download_from_holding_to_local(bucket: Bucket) -> Result<()> {

    let filepath_in_holding = format!("/holding/{}", FILE_NAME_UUID);
    // get file from s3 /holding
    let (file_body, code) = bucket.get(&filepath_in_holding).unwrap();
    assert_eq!(STATUS_CODE, code);

    // DOWNLOAD FILE : write a file in the original format - Vec<u8>
    let file_downloaded = write_file_in_local(file_body.clone());
    
    /* CONVERT FILE CONTENT INTO STRING - HUMAN READABLE FROM Vec<u8> TYPE just to check : cat command
    let s = match str::from_utf8(&file_body.clone()) {
        Ok(body) => {
            println!("file body  : {:?}", body);
        }
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };*/

    Ok(())
}

async fn copy_file_from_holding_to_backup(client: &S3Client) -> Result<()>{
    
    let req = CopyObjectRequest {
        bucket: BUCKET_NAME.to_string(),
        key: format!("backup/{}", FILE_NAME_UUID),    // to
        copy_source: rusoto_s3::util::encode_key(format!("{}/holding/{}", BUCKET_NAME, FILE_NAME_UUID)), // from
        content_type: Some("application/json".to_owned()),
        ..Default::default()
    };

    println!("copying file {:?} from holding to backup", FILE_NAME_UUID);

    let result = client
        .copy_object(req)
        .await
        .expect("Couldn't copy the file");


    Ok(())
}

// delete file in /holding in aws s3
pub fn delete_file_in_s3_holding(bucket: Bucket) -> Result<()> {
    let filepath_to_delete_in_holding = format!("/holding/{}", FILE_NAME_UUID);
    let (_, code) = bucket.delete(&filepath_to_delete_in_holding).unwrap();

    assert_eq!(STATUS_CODE_DEL, code);    

    Ok(())
}

// delete file in local
pub fn delete_file_in_local() ->  std::io::Result<()> {
    fs::remove_file(format!("C:/Users/M7/Desktop/decrypt_file/downloaded_encrypted_files/{}", FILE_NAME_UUID))?;
    
    Ok(())
}



















/*----- THIS WILL LIST FILES AND UPLOAD LOCAL FILES TO THE BUCKET USING COMMAND LINE,
** IT WORKS BUT THE REASON WE USE ABOVE CODE IS TO MAKE SURE THE FILE IS UPLOADED USING S3 COMMAND.

use std::process::Command;

pub fn aws_s3(){
       // list_files_in_holding();
       once_decrypted_mv_file_to_backup();
        list_files_in_holding();
}

pub fn list_files_in_holding(){
    println!("list_files_in_holding\n\n");

    // command line: aws s3 ls s3://BUCKETNAME/holding/
    Command::new("aws")
    .arg("s3") // separates the command by space
    .arg("ls")
    .arg("s3://BUCKETNAME/holding/")
    .spawn() // runs the command
    .expect("aws s3 failed to list files in the holding folder"); // error msg
}

pub fn once_decrypted_mv_file_to_backup(){
    println!("once_decrypted_mv_file_to_backup\n");

    // command line: aws s3 mv s3://BUCKETNAME/holding/encrypted_file s3://BUCKETNAME/backup/
    Command::new("aws")
    .arg("s3") // separates the command by space
    .arg("mv")
    .arg("s3://BUCKETNAME/holding/encrypted_file")
    .arg("s3://BUCKETNAME/backup/")
    .spawn() // runs the command
    .expect("aws s3 failed to move a decrypted file to the backup folder"); // error msg
}
*/