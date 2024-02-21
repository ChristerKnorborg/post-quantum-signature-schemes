use std::env;
use std::fs;

pub fn read_kat() -> () {
    let file_path = "genKAT/Results MAYO/PQCsignKAT_24_MAYO_1.rsp";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    for line in contents.lines() {

        if line.starts_with("seed") {
            let res: &str = line.strip_prefix("seed = ").unwrap();
            let res_bytes: Vec<u8> = res.as_bytes().to_vec();
        }
        else if line.starts_with("msg") {
            let res: &str = line.strip_prefix("msg = ").unwrap();
            let res_bytes: Vec<u8> = res.as_bytes().to_vec();
        }
        else if line.starts_with("pk") {
            let res: &str = line.strip_prefix("pk = ").unwrap();
            let res_bytes: Vec<u8> = res.as_bytes().to_vec();
        }
        else if line.starts_with("sk") {
            let res: &str = line.strip_prefix("sk = ").unwrap();
            let res_bytes: Vec<u8> = res.as_bytes().to_vec();
        }
        else if line.starts_with("sm") {
            let res: &str = line.strip_prefix("sm = ").unwrap();
            let res_bytes: Vec<u8> = res.as_bytes().to_vec();
        }
        
    }

}