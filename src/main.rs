use std::clone;

use constants::{SALT_BYTES, SIG_BYTES};
use genKAT::bindings;
use utils::set_seed_for_test;
use utils::write_to_file_int;
use utils::write_to_file_byte;
use mayo_functionality as mf;

use crate::{
    bitsliced_functionality::{decode_bit_sliced_matrices, encode_bit_sliced_matrices}, constants::M, finite_field::{add, mul}, utils::bytes_to_hex_string, utils::hex_string_to_bytes
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
    // let seed_hex = "061550234D158C5EC95595FE04EF7A25767F2E24CC2BC479D09D86DC9ABCFDE7056A8C266F9EF97ED08541DBD2E1FFA1";
    // let seed_bytes = hex_string_to_bytes(seed_hex);

    // let message_hex = "D81C4D8D734FCBFBEADE3D3F8A039FAA2A2C9957E835AD55B22E75BF57BB556AC8";
    // let message_bytes = hex_string_to_bytes(message_hex);

    // set_seed_for_test(seed_bytes.clone());

    // let (cpk, csk) = mf::compact_key_gen(seed_bytes);

    // let sig_mes = mf::api_sign(message_bytes, csk);

    // let resso = mf::api_sign_open(sig_mes, cpk);





    println!("file name: {} \n", constants::COMPARE_FILE_NAME);
    println!("O: {} \n", constants::O);
    println!("K: {} \n", constants::K);
    println!("N: {} \n", constants::N);
    println!("M: {} \n", constants::M);
    read_kat_file::read_kat();

}
