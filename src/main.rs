
use lib::crypto_primitives::safe_veor;
use lib::{crypto_primitives::safe_vmull, vector_mul};
use lib::write_and_compare_kat_file::write_and_compare_kat_file;
use std::process::Command;
use lib::finite_field::{add, mul};



fn main() {
   /*  let mut x = [0u8 ; 200];
    let y = [1u8 ; 200];
    let z = [9u8 ; 200];

    safe_veor(  &mut x, &y, &z, 200);

    // should be 3
    println!("{:?}", x);  */

    //let yeehaw = vector_mul!(y, z, 10);
    // println!("From vector mul {}", yeehaw);

    write_and_compare_kat_file(); 
}