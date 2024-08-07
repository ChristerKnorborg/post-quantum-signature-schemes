use crate::crypto_primitives::safe_random_bytes_init;
use std::fs::File;
use std::io::{self, Read};
use std::io::{Result, Write};
use std::path::Path;

pub fn test_random(k: u8, o: u8) -> Vec<u8> {
    let num_elems: u16 = (k * o) as u16;

    let test_vec = vec![1; num_elems as usize];
    test_vec
}

pub fn print_matrix(mat: Vec<Vec<u8>>) {
    mat.iter().for_each(|f| {
        println!("{:?}", f);
    })
}

// Method to set the random number generator seed for debugging and testing purposes
pub fn set_seed_for_test(mut entropy_input: Vec<u8>) {
    let personalization_string: Vec<u8> = vec![0u8; 47]; // Example, adjust as necessary

    safe_random_bytes_init(
        &mut entropy_input,
        &personalization_string, // Even if empty, this is now a valid pointer
        256,
    );
}

pub fn write_u32_array_to_file_int(filename: &str, data: &[u32]) -> Result<()> {
    let mut file = File::create(filename)?;

    for value in data.iter() {
        write!(file, "{}", value)?;
    }

    Ok(())
}

pub fn write_u32_array_to_file_byte(filename: &str, data: &[u32]) -> Result<()> {
    let mut file = File::create(filename)?;

    for value in data.iter() {
        write!(file, "{:08X}", value)?;
    }

    Ok(())
}

pub fn write_u8_array_to_file_int(filename: &str, data: &[u8]) -> Result<()> {
    let mut file = File::create(filename)?;

    for byte in data.iter() {
        write!(file, "{:}", byte)?;
    }

    Ok(())
}

pub fn write_u8_array_to_file_byte(filename: &str, data: &[u8]) -> Result<()> {
    let mut file = File::create(filename)?;

    for byte in data.iter() {
        write!(file, "{:02X}", byte)?;
    }

    Ok(())
}

// Convert a hex string to a byte vector of nibbles by parsing each pair of hex digits
// into a u8 and collecting them into a single Vec<u8>.
pub fn hex_string_to_bytes(hex_str: &str) -> Vec<u8> {
    let uneven: bool = hex_str.len() % 2 != 0;

    // Iteration will be 1 less if uneven (due to integer division)
    let iterations = hex_str.len() / 2;

    let mut res: Vec<u8> = hex_str
        .as_bytes()
        .chunks(2)
        .take(iterations) // Iterations depends on uneven
        .map(|chunk| {
            let hex_digit = std::str::from_utf8(chunk).unwrap();
            u8::from_str_radix(hex_digit, 16).unwrap()
        })
        .collect();

    if uneven {
        // Process last character by making it the higher nibble
        let last_char = &hex_str[hex_str.len() - 1..];
        let last_byte = u8::from_str_radix(last_char, 16).unwrap() << 4;
        res.push(last_byte);
    }

    res
}

// Convert a byte vector of nibbles to a hex string by formatting each byte as a pair of
// hex digits and concatenating them into a single String.
pub fn bytes_to_hex_string(bytes: &[u8], uneven: bool) -> String {
    let mut hex_str = String::new();
    let len = bytes.len();

    for (i, byte) in bytes.iter().enumerate() {
        if uneven && i == len - 1 {
            // For the last byte and if uneven is true, format only the first 4 bits
            hex_str.push_str(&format!("{:01X}", byte >> 4));
        } else {
            // Convert the whole byte to two hex characters
            hex_str.push_str(&format!("{:02X}", byte));
        }
    }
    hex_str
}

fn read_file_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn compare_hex_files<P: AsRef<Path>>(file1_path: P, file2_path: P) -> io::Result<bool> {
    let file1_contents = read_file_to_string(file1_path)?;
    let file2_contents = read_file_to_string(file2_path)?;

    Ok(file1_contents == file2_contents)
}

// test echoleon_form
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hex_to_bytes_and_back() {
        // Test with 0 count from GENKAT MAYO 1
        let count_0_message = "D81C4D8D734FCBFBEADE3D3F8A039FAA2A2C9957E835AD55B22E75BF57BB556AC8";
        let count_0_sk = "7C9935A0B07694AA0C6D10E4DB6B1ADD2FD81A25CCB14803";

        // Convert to bytes and back to hex
        let message_bytes = hex_string_to_bytes(count_0_message);
        let sk_bytes = hex_string_to_bytes(count_0_sk);
        let hex_str_back_message = bytes_to_hex_string(&message_bytes, false);
        let hex_str_back_sk = bytes_to_hex_string(&sk_bytes, false);

        // Assert that the hex strings are the same before and after conversion
        assert_eq!(count_0_message, hex_str_back_message);
        assert_eq!(count_0_sk, hex_str_back_sk);
    }

    #[test]
    fn test_hex_to_bytes_and_back_uneven_start() {
        // Test with 0 count from GENKAT MAYO 1
        let count_0_message = "D81C4D8D734FCBFBEADE3D3F8A039FAA2A2C9957E835AD55B22E75BF57BB556AC8F"; // uneven number of characters (Added a F at the end to make it uneven)
        let count_0_sk = "7C9935A0B07694AA0C6D10E4DB6B1ADD2FD81A25CCB14803";

        // Convert to bytes and back to hex
        let message_bytes = hex_string_to_bytes(count_0_message);
        let sk_bytes = hex_string_to_bytes(count_0_sk);
        let hex_str_back_message = bytes_to_hex_string(&message_bytes, true);
        let hex_str_back_sk = bytes_to_hex_string(&sk_bytes, false);

        // Assert that the hex strings are the same before and after conversion
        assert_eq!(count_0_message, hex_str_back_message);
        assert_eq!(count_0_sk, hex_str_back_sk);
    }
}
