use constants::{SALT_BYTES, SIG_BYTES};

use crate::bitsliced_functionality::{decode_bit_sliced_matrices, encode_bit_sliced_matrices};

mod sample;
mod finite_field;
mod utils;
mod constants;
mod bitsliced_functionality;
mod mayo_functionality;





fn main() {

    let (cpk, csk) = mayo_functionality::compact_key_gen();
    
    // println!("Compact Secret key: {:?}", csk);
    // println!("Compact Public key: {:?}", cpk);

    // println!("Compact Secret key length: {:?}", csk.len());
    // println!("Compact Public key length: {:?}", cpk.len());
    


    let esk = mayo_functionality::expand_sk(csk);
    let epk = mayo_functionality::expand_pk(cpk);


    // println!("Expanded Secret key: {:?}", esk);
    // println!("Expanded Public key: {:?}", epk);

    // println!("Expanded Secret key length: {:?}", esk.len());
    // println!("Expanded Public key length: {:?}", epk.len());
    

    let message = vec![1, 2, 3, 4, 5, 6, 7, 8];
    
    let sig = vec![12u8; SIG_BYTES];

    let verify: bool = mayo_functionality::verify(epk, sig, &message);


    //let sig = mayo_functionality::sign(esk, &message);





}