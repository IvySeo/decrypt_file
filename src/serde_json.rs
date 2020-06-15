#[macro_use]
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Result, Value};



#[derive(Serialize, Deserialize, Debug)]
struct M7Small {
     date : String, // date
     time  : String, // time without time zone,
     heading: String,
     lat : f32,  // type Real in postgres
     lathemi: String,
     lon : f32,
     lonhemi: String,
     truckid: i32,
     mph: String,
     gun1 : i32,
     gun2 : i32,
     gun3 : i32,
     gun4 : i32,
     gun5 : i32,
     gun6 : i32,
     gun7 : i32,
     gun8 : i32,
     yellowmil : String,
     whitemil  : String,
     beads  : String,
     project  : String,
     company  : String,
     region : String,
}


 pub fn convert_from_str(m7data: &str) -> Result<()> {

    let m7: Vec<M7Small> = serde_json::from_str(m7data)?;
    //println!("\n\nm7 : {:#?}\n\n", m7);

    //println!("truckid: {}, date: {}", m7[0].date, m7[0].truckid);

    Ok(())
}