mod sample;
mod finite_field;
mod utils;


use crate::sample as samp;
use crate::finite_field as ff;
use crate::utils as util;


fn main() {


    /* 
    let inv_of_2 = ff::inv(0x2);
    println!("The inverse of 2 is: {}", inv_of_2);
    

    let four_mult_nine = ff::mul(0x4, 0x9);
    println!("4 * 9 = {}", four_mult_nine);

    */

    // Input matrix A
    let mut a = vec![
        vec![0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x4], 
        vec![0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xf], 
        vec![0x8, 0x9, 0xf, 0xe, 0x3, 0x4, 0x5, 0x6],
        vec![0x8, 0x9, 0x5, 0x6, 0xe, 0xa, 0xb, 0x1],  
    ];
    let mut a_clone = a.clone();

    // Input vector y (hash/tag)
    let y = vec![0x1, 0x2, 0x3, 0x4];

    println!("Input Matrix: ");
    util::print_matrix(a.clone());

    println!("Input Vector (hash): ");
    println!("{:?}", y);


    let x: Vec<u8> = match samp::sample_solution(a, y.clone()) {
        Ok(x) => x, // If Ok, destructure the tuple into `a` and `x`
        Err(e) => {
            println!("Error: {}", e);
            return; // Exit the function early in case of error.
        }
    };


    println!("Vector x: ");
    println!("{:?}", x);


    //first iter goes to row, second iter goes over all entries in a row.
    //a[1][1]*x[1] +  a[1][2]*x[2] .....
    let Ax_eq_y: Vec<u8> = a_clone.iter().map(|row| {
        row.iter().zip(x.iter()).map(|(a_row_idx, x_idx)| ff::mul(*a_row_idx, *x_idx))
        .fold(0, |acc, x| ff::add(acc, x))
    }).collect();

    println!("Solution Ax = y");
    println!("Ax: {:?}" , Ax_eq_y);
    println!("y: {:?}" , y);

}