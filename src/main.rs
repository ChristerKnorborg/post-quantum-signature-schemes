use std::clone;

use constants::{SALT_BYTES, SIG_BYTES};
use genKAT::bindings;

use crate::{
    bitsliced_functionality::{decode_bit_sliced_matrices, encode_bit_sliced_matrices},
    utils::bytes_to_hex_string,
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

    println!("Message: {:?}", bytes_to_hex_string(&message, false));

    crypto_primitives::safe_randombytes_init(
        &mut entropy_input,
        &personalization_string, // Even if empty, this is now a valid pointer
        256,
    );

    let (cpk, csk) = mayo_functionality::compact_key_gen(entropy_input);

    let esk = mayo_functionality::expand_sk(&csk);

    let epk = mayo_functionality::expand_pk(cpk);

    //TODO(i think this is correct)) Need to do this, because they call it in sign to get counter correct
    let esk1 = mayo_functionality::expand_sk(&csk);

    mayo_functionality::sign(&csk, &message);

    println!("expanded PK {:?}", bytes_to_hex_string(&epk, false));

    //read_kat_file::read_kat();

    // let (cpk, csk) = mayo_functionality::compact_key_gen();

    // println!("Compact Secret key: {:?}", csk);
    // println!("Compact Public key: {:?}", cpk);

    // println!("Compact Secret key length: {:?}", csk.len());
    // println!("Compact Public key length: {:?}", cpk.len());

    // let esk = mayo_functionality::expand_sk(csk);
    // let epk = mayo_functionality::expand_pk(cpk);

    // println!("Expanded Secret key: {:?}", esk);
    // println!("Expanded Public key: {:?}", epk);

    // println!("Expanded Secret key length: {:?}", esk.len());
    // println!("Expanded Public key length: {:?}", epk.len());

    // let message = vec![1, 2, 3, 4, 5, 6, 7, 8];

    // let sig = vec![12u8; SIG_BYTES];

    //let verify: bool = mayo_functionality::verify(epk, sig, &message);

    //let sig = mayo_functionality::sign(esk, &message);
}
