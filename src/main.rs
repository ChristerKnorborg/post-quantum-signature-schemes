use std::clone;

use constants::{SALT_BYTES, SIG_BYTES};

use lib::read_kat_file::write_kat_file;
use utils::write_to_file_byte;
use utils::write_to_file_int;

use crate::{
    constants::M,
    finite_field::{add, mul},
    utils::bytes_to_hex_string,
};

use lib::bitsliced_functionality;
use lib::constants;
use lib::crypto_primitives;
use lib::finite_field;
use lib::mayo_functionality;
use lib::read_kat_file;
use lib::sample;
use lib::utils;

fn main() {
    // println!("file name: {} \n", constants::COMPARE_FILE_NAME);
    // println!("O: {} \n", constants::O);
    // println!("K: {} \n", constants::K);
    // println!("N: {} \n", constants::N);
    // println!("M: {} \n", constants::M);
    // read_kat_file::read_kat();



    write_kat_file();
}
