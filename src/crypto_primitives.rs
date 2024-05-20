use crate::arm_neon_intrinsic::arm_intrinsic::{encode_bit_sliced_array_mayo12, mul_add_bitsliced_m_vec, mul_add_bitsliced_m_vec_mayo1, mul_add_bitsliced_m_vec_mayo3, mul_add_bitsliced_m_vec_mayo5};
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



pub fn safe_mul_add_bitsliced_m_vec(input: &[u32], input_start: i32, nibble: u8, acc: &mut [u32], acc_start: i32){
    unsafe { mul_add_bitsliced_m_vec(input.as_ptr(), input_start, nibble, acc.as_mut_ptr(), acc_start) }
}

pub fn safe_mul_add_bitsliced_m_vec_mayo1(input: &[u32], input_start: i32, input_offset: i32, nibble1: u8, nibble2: u8, acc: &mut [u32], acc_start: i32){
    unsafe { mul_add_bitsliced_m_vec_mayo1(input.as_ptr(), input_start, input_offset, nibble1, nibble2, acc.as_mut_ptr(), acc_start) }
}

pub fn safe_mul_add_bitsliced_m_vec_mayo3(input: &[u32], input_start: i32, nibble: u8, acc: &mut [u32], acc_start: i32){
    unsafe { mul_add_bitsliced_m_vec_mayo3(input.as_ptr(), input_start, nibble, acc.as_mut_ptr(), acc_start) }
}

pub fn safe_mul_add_bitsliced_m_vec_mayo5(input: &[u32], input_start: i32, nibble: u8, acc: &mut [u32], acc_start: i32){
    unsafe { mul_add_bitsliced_m_vec_mayo5(input.as_ptr(), input_start, nibble, acc.as_mut_ptr(), acc_start) }
}


pub fn safe_encode_bit_sliced_array_mayo12(input: &mut [u8], output: &mut [u8], matrices: i32){
    unsafe { encode_bit_sliced_array_mayo12(input.as_mut_ptr() , output.as_mut_ptr() , matrices) }
}

