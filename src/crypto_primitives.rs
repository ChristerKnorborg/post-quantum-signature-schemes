use crate::genkat::bindings;
use crate::assembly::arm_instructions;
use aes::cipher::{KeyIvInit, StreamCipher};
use byteorder::{ByteOrder, LittleEndian};
use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::Shake256;





/* RANDOMNESS AND EXTENDED OUTPUT FUNCTION USE NIST CALLS 
    - SAME AS MAYO VERSION BY THE AUTHORS 
*/


pub fn safe_random_bytes_init(
    entropy_input: &mut [u8],
    personalization_string: &[u8],
    security_strength: i32,
) {
    unsafe {
        bindings::randombytes_init(
            entropy_input.as_mut_ptr(),
            personalization_string.as_ptr(),
            security_strength,
        );
    }
}

pub fn safe_random_bytes(random_arrays: &mut [u8], nbytes: u64) {
    unsafe {
        bindings::randombytes(random_arrays.as_mut_ptr(), nbytes);
    }
}

pub fn safe_aes_128_ctr(
    output: &mut [u8],
    output_byte_len: u64,
    input: &[u8],
    input_byte_len: u64,
) {
    unsafe {
        bindings::AES_128_CTR(
            output.as_mut_ptr(),
            output_byte_len,
            input.as_ptr(),
            input_byte_len,
        );
    }
}

pub fn safe_shake256(output: &mut [u8], output_byte_len: u64, input: &[u8], input_byte_len: u64) {
    unsafe {
        bindings::shake256(
            output.as_mut_ptr(),
            output_byte_len,
            input.as_ptr(),
            input_byte_len,
        );
    }
}










pub fn aes_128_ctr_seed_expansion(pk_seed: [u8; 16], output_length: usize) -> Vec<u8> {
    type Aes128Ctr64LE = ctr::Ctr64LE<aes::Aes128>; // Define the type of the cipher (AES-128-CTR in little-endian mode)

    let key = pk_seed; // 16 bytes key
    let iv: [u8; 16] = [0u8; 16]; // 16 bytes IV

    let mut cipher = Aes128Ctr64LE::new(&key.into(), &iv.into());

    let mut output = Vec::with_capacity(output_length);

    let mut ctr: u128 = 0u128; // 128-bit counter (0 initial value) to encrypt

    while output.len() < output_length {
        let mut buf = [0u8; 16]; // 16 bytes buffer to store the counter
        LittleEndian::write_u128(&mut buf, ctr); // Write the counter to the buffer (array of bytes)
        cipher.apply_keystream(&mut buf); // Encrypt the counter with the key and IV
        output.extend_from_slice(&buf); // Append the encrypted counter to the output vector

        ctr += 1;
    }

    // Truncate the output to the desired length (if not multiple of 16 bytes)
    output.truncate(output_length);

    return output;
}






// Function to hash a bytestring with SHAKE256 to a specified output length
pub fn shake256(bytestring: &Vec<u8>, output_length: usize) -> Vec<u8> {
    let mut hasher = Shake256::default();

    hasher.update(&bytestring);

    let mut output = vec![0; output_length]; // Allocate space for the output
    let mut reader = hasher.finalize_xof(); // Get the reader for the output
    reader.read(&mut output); // Read the output into the allocated space

    return output;
}


pub fn safe_asm(res_last_row: &mut [u8], p1_last_row: &[u8], final_o_vec: &[u8], row_length: i32) {
    unsafe { arm_instructions::vmull_values(res_last_row.as_mut_ptr(), p1_last_row.as_ptr(), final_o_vec.as_ptr(), row_length); }
}








#[cfg(test)]
mod tests {
    use crate::utils::print_matrix;

    use super::*;

    #[test]
    fn test_shake256() {
        let input = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
        let output_length = 32;
        let result = shake256(&input, output_length);
        assert_eq!(result.len(), output_length);
        println!("{:?}", result);
    }

    #[test]
    fn test_aes_128_ctr_seed_expansion() {
        let input = [0x00; 16];
        let output_length = 32;
        let result = aes_128_ctr_seed_expansion(input, output_length);
        assert_eq!(result.len(), output_length);
        println!("{:?}", result);
    }
}