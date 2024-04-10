use lib::crypto_primitives::safe_asm;
use lib::finite_field;
use lib::write_and_compare_kat_file::write_and_compare_kat_file;
use std::process::Command;




fn main() {
    /*let mut x = [2u8 ; 58];
    let y = [2u8 ; 58];
    let z = [9u8 ; 58];


    println!("2 times 9 in field: {}", finite_field::mul(2, 9));
    
    safe_asm(&mut x, &y, &z, 58);

    println!("{:?}", x); */

    write_and_compare_kat_file();
}