
use lib::{crypto_primitives::safe_vmull, vector_mul};
use lib::write_and_compare_kat_file::write_and_compare_kat_file;
use std::process::Command;
use lib::finite_field::{add, mul};



fn main() {
   /* let mut x: u8 = 0;
    let y = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8];
    let z = [9u8 ; 10];

    safe_vmull(  &mut x, &y, &z, 10);

    // should be 3
    println!("{:?}", x); 

    let yeehaw = vector_mul!(y, z, 10);
    println!("From vector mul {}", yeehaw); */

    write_and_compare_kat_file(); 
}