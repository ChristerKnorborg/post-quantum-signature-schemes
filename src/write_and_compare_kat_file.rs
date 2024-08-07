use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use crate::constants::{COMPARE_FILE_NAME, CPK_BYTES, P3_BYTES, PK_SEED_BYTES, SIG_BYTES, VERSION};
use crate::crypto_primitives::{safe_random_bytes, safe_random_bytes_init};
use crate::mayo_functionality::{api_sign, api_sign_open, compact_key_gen};
use crate::utils::bytes_to_hex_string;
use std::fs::OpenOptions;
use std::io::Write;

pub fn write_and_compare_kat_file() {
    let mut seeds = vec![vec![0u8; 48]; 100];
    let mut messages = vec![Vec::new(); 100];
    let mut entropy_input: Vec<u8> = (0..=47).collect();
    let personalization_string: Vec<u8> = vec![0u8; 48]; // Example, adjust as necessary
    safe_random_bytes_init(&mut entropy_input, &personalization_string, 256);
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
        let mut seed = vec![0u8; 48];
        safe_random_bytes(&mut seed, nbytes);
        seeds[count] = seed;

        let mlen = 33 * (count + 1);
        let mut msg = vec![0u8; mlen];

        safe_random_bytes(&mut msg, mlen as u64);
        messages[count] = msg;
    }

    for count in 0..100 {
        // Print progress
        print!("\rProcessing interation {} / 100", count + 1);
        io::stdout().flush().unwrap();

        let cur_seed = &mut seeds[count];
        safe_random_bytes_init(cur_seed, &personalization_string, 256);

        let mlen = 33 * (count + 1);
        let smlen = mlen + SIG_BYTES;

        let (cpk, csk) = compact_key_gen();

        let cpk_seed = cpk.seed;
        let cpk_p3_u8 = cpk.p3;

        let mut p3_u8 = [0u8; P3_BYTES];
        for (i, &num) in cpk_p3_u8.iter().enumerate() {
            let byte_slice = num.to_le_bytes(); // Convert each u32 to 4 u8s. Use to_be_bytes for big endian.
            let start_index = i * 4;
            p3_u8[start_index..start_index + 4].copy_from_slice(&byte_slice);
        }

        let mut cpk_array = [0u8; CPK_BYTES];
        cpk_array[..PK_SEED_BYTES].copy_from_slice(&cpk_seed);
        cpk_array[PK_SEED_BYTES..].copy_from_slice(&p3_u8);

        let signature = api_sign(messages[count].clone(), csk);
        let (ver_cor, _) = api_sign_open(signature.clone(), cpk);

        let seed_hex = bytes_to_hex_string(&seeds[count], false);
        let msg_hex = bytes_to_hex_string(&messages[count], false);

        let cpk_hex = bytes_to_hex_string(cpk_array.as_ref(), false);
        let csk_hex = bytes_to_hex_string(csk.as_ref(), false);
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

    let correct_file_produced = compare_files("output.txt", COMPARE_FILE_NAME);

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

pub fn compare_files<P: AsRef<Path>>(file1_path: P, file2_path: P) -> bool {
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

    println!(""); // Newline for readability
    if is_different {
        println!("^^^^^^ INCORRECT VALUES PRODUCED!. CHECK DIFFERENCES ABOVE ^^^^^^");
        false
    } else {
        println!("CORRECT VALUES PRODUCED!");
        true
    }
}
