use std::clone;

use constants::{SALT_BYTES, SIG_BYTES};
use genKAT::bindings;
use utils::write_to_file_int;
use utils::write_to_file_byte;

use crate::{
    bitsliced_functionality::{decode_bit_sliced_matrices, encode_bit_sliced_matrices}, constants::M, finite_field::{add, mul}, utils::bytes_to_hex_string
};

mod bitsliced_functionality;
mod constants;
mod crypto_primitives;
mod finite_field;
mod mayo_functionality;
mod read_kat_file;
mod sample;
mod utils;
mod genKAT {
    pub mod bindings;
}

fn main() {
    let mut entropy_input: Vec<u8> = (0..=47).collect();
    let personalization_string: Vec<u8> = vec![0u8; 47]; // Example, adjust as necessary
    let nbytes: u64 = entropy_input.len() as u64;

    crypto_primitives::safe_randombytes_init(
        &mut entropy_input,
        &personalization_string, // Even if empty, this is now a valid pointer
        256,
    );

    let mut bing: Vec<u8>;
    let mut message: Vec<u8> = vec![0u8; 33];

    crypto_primitives::safe_randomBytes(&mut entropy_input, nbytes);
    //SAFE_RANDOMBYTES SUSPISIOUS
    crypto_primitives::safe_randomBytes(&mut message, 33 as u64);

    crypto_primitives::safe_randombytes_init(
        &mut entropy_input,
        &personalization_string, // Even if empty, this is now a valid pointer
        256,
    );

    let (cpk, csk) = mayo_functionality::compact_key_gen(entropy_input);


    let signature = mayo_functionality::api_sign(message.clone(), csk.clone());

    let flattened: Vec<u8> = signature.clone().into_iter().collect();
    let array: Box<[u8]> = flattened.into_boxed_slice();
    let array_ref: &[u8] = &*array;
    let _ = write_to_file_byte("sig", array_ref);

    let ver = mayo_functionality::api_sign_open(signature, cpk);

    println!("{:?}", ver);

}
