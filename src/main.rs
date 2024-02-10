mod sample;
mod finite_field;
mod utils;
mod constants;
mod bitsliced_functionality;
mod MAYO_functionality;



use MAYO_functionality::aes_128_ctr_seed_expansion;

use crate::bitsliced_functionality as bf;


fn main() {

    let pk_seed: [u8; 16] = [0; 16];

    let expanded_seed = aes_128_ctr_seed_expansion(pk_seed, 50);


    println!("{:?}", expanded_seed);


}