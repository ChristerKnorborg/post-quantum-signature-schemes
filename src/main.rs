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
    let mut entropy_input: Vec<u8> = vec![0u8; 1000]; // Example entropy input
    let personalization_string: Vec<u8> = vec![0u8; 1000]; // Example, adjust as necessary
    let nbytes: u64 = entropy_input.len() as u64;

    println!("Entropy input: {:?}", entropy_input);

    unsafe {
        bindings::randombytes_init(
            entropy_input.as_mut_ptr(),
            personalization_string.as_ptr(), // Even if empty, this is now a valid pointer
            256,
        );

        bindings::randombytes(entropy_input.as_mut_ptr(), nbytes);
    }

    println!("Entropy input: {:?}", entropy_input);
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
