mod sample;
mod finite_field;


use crate::sample as samp;
use crate::finite_field as ff;


fn main() {


    /* 
    let inv_of_2 = ff::inv(0x2);
    println!("The inverse of 2 is: {}", inv_of_2);
    

    let four_mult_nine = ff::mul(0x4, 0x9);
    println!("4 * 9 = {}", four_mult_nine);

    */

    // Input matrix A
    let a = vec![
        vec![0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x4, 0x2], 
        vec![0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xf, 0xe], 
        vec![0x8, 0x9, 0xf, 0xe, 0x3, 0x4, 0x5, 0x6, 0x2],
        vec![0x8, 0x9, 0x5, 0x6, 0xe, 0xa, 0xb, 0x1, 0x3],  
    ];

    // Input vector y (tag)
    let y = vec![0x1, 0x2, 0x3, 0x4];

    println!("Input Matrix: ");
    samp::print_matrix(a.clone());

    println!("Input Vector (tag): ");
    println!("{:?}", y);


    let x: Vec<u8> = match samp::sample_solution(a, y.clone()) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };


    

    println!("Vector x: ");
    println!("{:?}", x);
    










}