use std::fs::File;
use std::io::prelude::*;

use crate::mayo_functionality as mf;

pub fn read_kat() -> () {
    let mut file = File::open("./src/genKAT/Results MAYO/PQCsignKAT_24_MAYO_1.txt").unwrap();

    let mut contents = String::new(); 
    file.read_to_string(&mut contents);

    let mut ctr = 0;
    let mut seed_bytes: Vec<u8> = Vec::with_capacity(500);
    let mut msg_bytes: Vec<u8> = Vec::with_capacity(500);
    let mut pk_bytes: Vec<u8> = Vec::with_capacity(500);
    let mut sk_bytes: Vec<u8> = Vec::with_capacity(500);
    let mut sm_bytes: Vec<u8> = Vec::with_capacity(500);

    for line in contents.lines() {

        if line.starts_with("count") {

            let res: &str = line.strip_prefix("count = ").unwrap();
            let comp = res.parse::<i32>().unwrap();
            if ctr != 0 || ctr != comp {
                let res_sm = mf::api_sign(msg_bytes.clone(), sk_bytes.clone());

                assert_eq!(res_sm, sm_bytes);
            }
            ctr += 1;
        }

        if line.starts_with("seed") {
            let res: &str = line.strip_prefix("seed = ").unwrap();
            seed_bytes = res.as_bytes().to_vec();
        }
        else if line.starts_with("msg") {
            let res: &str = line.strip_prefix("msg = ").unwrap();
             msg_bytes = res.as_bytes().to_vec();
        }
        else if line.starts_with("pk") {
            let res: &str = line.strip_prefix("pk = ").unwrap();
            sk_bytes = res.as_bytes().to_vec();
        }
        else if line.starts_with("sk") {
            let res: &str = line.strip_prefix("sk = ").unwrap();
            sk_bytes = res.as_bytes().to_vec();
        }
        else if line.starts_with("sm ") {
            let res: &str = line.strip_prefix("sm = ").unwrap();
            sm_bytes = res.as_bytes().to_vec();
        }
        
    }

}