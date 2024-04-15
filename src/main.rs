
use lib::crypto_primitives::{safe_veor, safe_vmull_flat};
use lib::matrix_mul;
use lib::{crypto_primitives::safe_vmull, vector_mul};
use lib::write_and_compare_kat_file::write_and_compare_kat_file;
use std::process::Command;
use lib::finite_field::{add, mul};



fn main() {
   /* let mut x = [0u8 ; 200];
    let y = [1u8 ; 200];
    let z = [9u8 ; 200];

    safe_vmull_flat(  &mut x, &y, &z, 20, 10, 20);

    // should be 3
    println!("{:?}", x); 

    let matrix_y = [[2u8 ; 10]; 20];
    let matrix_z = [[9u8 ; 20]; 10];

    let yeehaw = matrix_mul!(matrix_y, 20, 10, matrix_z, 20);
    for i in 0..20 {
        println!("{:?}", yeehaw[i]);
    } */

   write_and_compare_kat_file(); 
}