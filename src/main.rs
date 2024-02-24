use constants::{SALT_BYTES, SIG_BYTES};
use genKAT::bindings;

use crate::bitsliced_functionality::{decode_bit_sliced_matrices, encode_bit_sliced_matrices};

mod bitsliced_functionality;
mod constants;
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

    println!("Entropy input: {:?}", entropy_input);

    mayo_functionality::safe_randombytes_init(
        &mut entropy_input,
        &personalization_string, // Even if empty, this is now a valid pointer
        256,
    );

    let mut bing: Vec<u8>;
    bing = entropy_input.clone();

    mayo_functionality::safe_randomBytes(&mut entropy_input, nbytes);

    println!(
        "Entropy input: {:?}",
        utils::bytes_to_hex_string(&entropy_input, false)
    );

    mayo_functionality::safe_randomBytes(&mut bing, nbytes);

    println!(
        "Entropy input: {:?}",
        utils::bytes_to_hex_string(&bing, false)
    );

    mayo_functionality::safe_randombytes_init(
        &mut entropy_input,
        &personalization_string, // Even if empty, this is now a valid pointer
        256,
    );

    mayo_functionality::compact_key_gen(entropy_input);

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
