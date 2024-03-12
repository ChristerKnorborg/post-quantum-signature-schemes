use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Read};
use std::path::Path;

use std::fs::OpenOptions;
use std::io::Write;
use crate::constants::{COMPARE_FILE_NAME, SIG_BYTES, VERSION};
use crate::utils::{bytes_to_hex_string, compare_hex_files, hex_string_to_bytes};
use crate::mayo_functionality::{self as mf, api_sign, api_sign_open, compact_key_gen};
use crate::crypto_primitives::{safe_randombytes_init, safe_randomBytes};
/* pub fn read_kat() -> () {
    let mut file = File::open(COMPARE_FILE_NAME).unwrap();

    let mut contents = String::new(); 
    _ = file.read_to_string(&mut contents);

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
                
                assert_eq!(res_sm, sm_bytes);
                assert!(ver_cor);
            
                
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
} */


pub fn write_kat_file() {
    let mut seeds = vec![vec![0u8; 48]; 100];
    let mut messages = vec![Vec::new(); 100];
    let mut entropy_input: Vec<u8> = (0..=47).collect();
    let mut personalization_string: Vec<u8> = vec![0u8; 48]; // Example, adjust as necessary
    safe_randombytes_init(&mut entropy_input, &mut personalization_string, 256);
    let nbytes: u64 = entropy_input.len() as u64; // seed fixed to 48 bytes


    let file_path = "output.txt";
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true) // Remove the file if it already exists
        .open(file_path)
        .unwrap();


    // Header
    writeln!(file, "# {}", VERSION).unwrap();
    writeln!(file, "").unwrap();


    // Create all seeds and messages
    for count in 0..100 {
        //fprintf(fp_req, "count = %d\n", i);
        let mut seed = vec![0u8 ; 48];
        safe_randomBytes(&mut seed, nbytes);
        seeds[count] = seed;

        let mlen = 33 * (count + 1);
        let mut msg = vec![0u8 ; mlen];

        safe_randomBytes(&mut msg, mlen as u64);
        messages[count] = msg;
    }
    


    for count in 0..100 { 

    println!("count = {}", count);

    let cur_seed = &mut seeds[count];
    safe_randombytes_init(cur_seed, &mut personalization_string, 256);



    let mlen = 33 * (count + 1);
    let smlen = mlen + SIG_BYTES;



    let (cpk, csk) = compact_key_gen(cur_seed.clone());
    let signature = api_sign(messages[count].clone(), csk.clone());
    let (ver_cor, _) = api_sign_open(signature.clone(), cpk.clone());



    let seed_hex = bytes_to_hex_string(&seeds[count], false);
    let msg_hex = bytes_to_hex_string(&messages[count], false);
    let cpk_hex = bytes_to_hex_string(&cpk.to_vec(), false);
    let csk_hex = bytes_to_hex_string(&csk.to_vec(), false);
    let sm_hex = bytes_to_hex_string(&signature, false);
    

    

    // Write formatted data to file
    writeln!(file, "count = {}", count).unwrap();
    writeln!(file, "seed = {}", seed_hex).unwrap();
    writeln!(file, "mlen = {}", mlen).unwrap();
    writeln!(file, "msg = {}", msg_hex).unwrap();
    writeln!(file, "pk = {}", cpk_hex).unwrap();
    writeln!(file, "sk = {}", csk_hex).unwrap();
    writeln!(file, "smlen = {}", smlen).unwrap();
    writeln!(file, "sm = {}", sm_hex).unwrap();
    writeln!(file, "").unwrap(); 

    assert!(ver_cor);
    
}


let correct_file_produced =  compare_files("output.txt", COMPARE_FILE_NAME);

if correct_file_produced {
    // Delete the file if the test passed
    std::fs::remove_file("output.txt").unwrap();
} 

    
}











fn read_file_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn compare_files<P: AsRef<Path>>(file1_path: P, file2_path: P) -> bool{
    let file1_contents = read_file_to_string(file1_path).unwrap();
    let file2_contents = read_file_to_string(file2_path).unwrap();

    // Split the contents into lines for comparison
    let file1_lines: Vec<&str> = file1_contents.lines().collect();
    let file2_lines: Vec<&str> = file2_contents.lines().collect();


    let mut is_different = false;



    let min_len = std::cmp::min(file1_lines.len(), file2_lines.len());
    // Check if one file has more lines than the other
    if file1_lines.len() != file2_lines.len() {
        println!(
            "Files differ in length: file1 has {} lines, file2 has {} lines",
            file1_lines.len(),
            file2_lines.len()
        );
        is_different = true;
    }


    // Compare files line by line
    for i in 0..min_len {
        if file1_lines[i] != file2_lines[i] {
            println!("Line {} differs", i + 1); // Line numbers are 1-indexed for readability
            is_different = true;
        }
    }

    if is_different {
        println!("^^^^^^ INCORRECT VALUES PRODUCED!. CHECK DIFFERENCES ABOVE ^^^^^^");
        return false;
    } else {
        println!("CORRECT VALUES PROCUCED!");
        return true;
    }
}








