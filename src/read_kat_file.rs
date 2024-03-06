use std::fs::File;
use std::io::prelude::*;

use crate::utils::{bytes_to_hex_string, hex_string_to_bytes};
use crate::mayo_functionality::{self as mf, api_sign_open};
use crate::crypto_primitives::{safe_randombytes_init, safe_randomBytes};
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

    let mut entropy_input: Vec<u8> = (0..=47).collect();
    let personalization_string: Vec<u8> = vec![0u8; 47]; // Example, adjust as necessary
    let nbytes: u64 = entropy_input.len() as u64;


    // Init the randombytes like NIST correctly
    safe_randombytes_init(
        &mut entropy_input,
        &personalization_string,
        256,
    );
    safe_randomBytes(&mut entropy_input, nbytes);

    for line in contents.lines() {


        if line.starts_with("count") {

            let res: &str = line.strip_prefix("count = ").unwrap();
            let comp = res.parse::<i32>().unwrap();
            if ctr != 0 || ctr != comp {

                safe_randombytes_init(
                    &mut seed_bytes,
                    &personalization_string,
                    256,
                );
                println!("Current round: {}", comp);
                let (cpk, csk) = mf::compact_key_gen(seed_bytes.clone());
                
                assert_eq!(csk, sk_bytes);
                assert_eq!(cpk, pk_bytes);


                let res_sm = mf::api_sign(msg_bytes.clone(), csk);
                let (ver_cor, _) = mf::api_sign_open(res_sm.clone(), cpk);

                assert!(ver_cor);
            
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











