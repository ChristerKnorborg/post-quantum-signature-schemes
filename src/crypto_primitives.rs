use crate::genkat::bindings;


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
    output: &mut [u32],
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
