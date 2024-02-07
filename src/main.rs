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

    // Input vector y (hash/tag)
    let y = vec![0x1, 0x2, 0x3, 0x4];

    println!("Input Matrix: ");
    util::print_matrix(a.clone());

    println!("Input Vector (tag): ");
    println!("{:?}", y);


    let mut x: Vec<u8> = match samp::sample_solution(a, y.clone()) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };


    

    println!("Vector x: ");
    println!("{:?}", x);
    let a_temp = vec![
        vec![0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x4], 
        vec![0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xf], 
        vec![0x8, 0x9, 0xf, 0xe, 0x3, 0x4, 0x5, 0x6],
        vec![0x8, 0x9, 0x5, 0x6, 0xe, 0xa, 0xb, 0x1],  
    ];

    //first iter goes to row, second iter goes over all entries in a row.
    //a[1][1]*x[1] +  a[1][2]*x[2] .....
    let mult_x: Vec<u8> = a_temp.iter().map(|row| {
        row.iter().zip(x.iter()).map(|(a_row_idx, r_idx)| ff::mul(*a_row_idx, *r_idx))
        .fold(0, |acc, x| ff::add(acc, x))
    }).collect();

    println!("Solution!");
    println!("{:?}" , mult_x);




}