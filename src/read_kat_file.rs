use std::fs::File;
use std::io::prelude::*;

use crate::utils::{bytes_to_hex_string, hex_string_to_bytes};
use crate::mayo_functionality as mf;
pub fn read_kat() -> () {
    let mut file = File::open("./src/genKAT/Results MAYO/PQCsignKAT_24_MAYO_1.txt").unwrap();

    let mut contents = String::new(); 
    file.read_to_string(&mut contents);

    let mut ctr = 0;
    let mut seed_bytes: Vec<u8> = Vec::new();
    let mut msg_bytes: Vec<u8> = Vec::new();
    let mut pk_bytes: Vec<u8> = Vec::new();
    let mut sk_bytes: Vec<u8> = Vec::new();
    let mut sm_bytes: Vec<u8> = Vec::new();

    for line in contents.lines() {
        println!("{}", line);


        if line.starts_with("count") {

            let res: &str = line.strip_prefix("count = ").unwrap();
            let comp = res.parse::<i32>().unwrap();
            if ctr != 0 || ctr != comp {


                let res_sm = mf::api_sign(msg_bytes.clone(), sk_bytes.clone());
                println!("msg_bytes len: {:?}", msg_bytes.len());
                println!("sk_bytes len: {:?}", sk_bytes.len());
                println!("msg_bytes: {:?}", msg_bytes);
                println!("sk_bytes: {:?}", sk_bytes);

                println!("back to hex: {:?}", bytes_to_hex_string(&msg_bytes, false));
                println!("back to hex: {:?}", bytes_to_hex_string(&sk_bytes, false));
            
                assert_eq!(res_sm, sm_bytes);
            }
            ctr += 1;
        }

        if line.starts_with("seed") {
            let res: &str = line.strip_prefix("seed = ").unwrap();
            seed_bytes = hex_string_to_bytes(res);
        }
        else if line.starts_with("msg") {
            let res: &str = line.strip_prefix("msg = ").unwrap();
            msg_bytes = hex_string_to_bytes(res); 
        }
        else if line.starts_with("pk") {
            let res: &str = line.strip_prefix("pk = ").unwrap();
            pk_bytes = hex_string_to_bytes(res); 
        }
        else if line.starts_with("sk") {
            let res: &str = line.strip_prefix("sk = ").unwrap();
            sk_bytes = hex_string_to_bytes(res); 
        }
        else if line.starts_with("sm ") {
            let res: &str = line.strip_prefix("sm = ").unwrap();
            sm_bytes = hex_string_to_bytes(res); 
        }
    }
}












