use crate::bitsliced_functionality::{decode_bit_sliced_matrices, encode_bit_sliced_matrices};

mod sample;
mod finite_field;
mod utils;
mod constants;
mod bitsliced_functionality;
mod MAYO_functionality;





fn main() {

    let (pk, sk) = MAYO_functionality::compact_key_gen();
    
    println!("Secret key: {:?}", sk);
    println!("Public key: {:?}", pk);

    println!("Secret key length: {:?}", sk.len());
    println!("Public key length: {:?}", pk.len());
    

}