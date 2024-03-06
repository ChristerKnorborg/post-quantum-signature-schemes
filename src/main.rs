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



    
    println!("file name: {} \n", constants::COMPARE_FILE_NAME);
    println!("O: {} \n", constants::O);
    println!("K: {} \n", constants::K);
    println!("N: {} \n", constants::N);
    println!("M: {} \n", constants::M);
    read_kat_file::read_kat();

}
