use std::fs::File;
use std::io::{Write, Result};


pub fn test_random(k: u8, o: u8) -> Vec<u8> {
    let num_elems: u16 = (k * o) as u16;

    let test_vec = vec![1; num_elems as usize];
    return test_vec;
}

pub fn print_matrix(mat: Vec<Vec<u8>>) -> () {
    mat.iter().for_each(|f| {
        println!("{:?}", f);
    })
}



// Helper function to transpose a matrix (as described in the MAYO paper)
pub fn transpose_matrix(matrix: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    // Create a new transposed matrix with the dimensions swapped
    let mut transposed = vec![vec![0u8; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = matrix[i][j]; // Swap elements
        }
    }
    return transposed;
}

// Helper function to transpose a matrix (as described in the MAYO paper)
pub fn transpose_vector(vector: &Vec<u8>) -> Vec<Vec<u8>> {
    let rows = vector.len();

    // Create a new transposed matrix with the dimensions swapped
    let mut transposed = vec![vec![0u8; rows]; 1];

    for i in 0..rows {
        transposed[0][i] = vector[i]; // Swap elements
    }
    return transposed;
}


pub fn write_to_file(filename: &str, data: &[u8]) -> Result<()> {
    let mut file = File::create(filename)?;
    
    writeln!(file, "\nbitsliced_P: ")?;
    for byte in data.iter() {
        write!(file, "{:}", byte)?;
    }
    
    Ok(())
}


// Convert a hex string to a byte vector by parsing each pair of hex digits
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

    return res;
}

// Convert a byte vector to a hex string by formatting each byte as a pair of
// hex digits and concatenating them into a single String.
pub fn bytes_to_hex_string(bytes: &Vec<u8>, uneven: bool) -> String {
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
    return hex_str;
}







// test echoleon_form
#[cfg(test)]
mod tests {

    use super::*;
    use crate::mayo_functionality as mf;
    use crate::utils as ut;
    use std::vec;

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
