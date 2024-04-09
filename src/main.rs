use lib::asm_instructions::safe_asm;
use lib::write_and_compare_kat_file::write_and_compare_kat_file;
use std::process::Command;


// use crate::asm_instructions;
    // write_and_compare_kat_file();


fn main() {
    let mut x = [0u8 ; 58];
    let y = [1u8 ; 58];
    let z = [1u8 ; 58];
    safe_asm(&mut x, &y, &z);

    println!("{:?}", x)
}