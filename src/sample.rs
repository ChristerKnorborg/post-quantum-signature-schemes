use std::vec;

use crate::finite_field::{self as ff, sub};
use rand::rngs::StdRng as rng;
use rand::{Rng, rngs::StdRng, SeedableRng};


// Function to perform the echelon form algorithm on matrix B.
pub fn echelon_form(mut b: Vec<Vec<u8>>, k: usize, o: usize) -> Vec<Vec<u8>> {

    let rows = b.len(); 
    let cols = b[0].len(); 
    let mut pivot_row = 0;
    let mut pivot_column = 0;

    while pivot_row < rows && pivot_column < k*o + 1{
        // Find the pivot
        let possible_pivots: Vec<usize> = (pivot_row..rows)
            .filter(|&i| b[i][pivot_column] != 0) // remember to dereferrence i to avoid ownership
            .collect(); // Transforms all the candidate pivots to an iterator.

        // If there are no possible pivots in this column, move to the next
        let next_pivot_row = if let Some(min_pivot) = possible_pivots.iter().min() {
            *min_pivot
        } else {
            // No pivot in this column, move to the next
            pivot_column += 1;
            continue;
        };

        // Swap rows
        b.swap(pivot_row, next_pivot_row);

        // Make the leading entry a "1" by multiplying the row by the inverse of the pivot
        let inv_idx = ff::inv(b[pivot_row][pivot_column]);
        for j in pivot_column..cols {
            b[pivot_row][j] = ff::mul(inv_idx, b[pivot_row][j]);
        }

        // Eliminate entries below the pivot
        for i in pivot_row + 1..rows { // From next pivot row to m - 1
            let factor = b[i][pivot_column];
            for j in pivot_column..cols {

                let finite_mult = ff::mul(factor, b[pivot_row][j]);
                let res = ff::sub(b[i][j], finite_mult);
                b[i][j] = res // b[i][j] - (factor * b[pivot_row][j]);
            }
        }

        pivot_row += 1;
        pivot_column += 1;
    }

    b
}

pub fn sample_rand(k: u8, o: u8) -> Vec<u8> {
    
    let num_elems: u16 = (k * o) as u16;

    let mut rand_core = rng::from_entropy();

    // REMEMBER TO MAKE CRYPTOGRAPHICALLY SECURE
    let vals: Vec<u8> = (0..num_elems).map(|_| rand_core.gen_range(0..15) as u8).collect();

    return vals;
}


pub fn sample_solution(mut a: Vec<Vec<u8>>, mut y: Vec<u8>) -> Result<Vec<u8>, &'static str> {
    let rows = a.len();
    let cols = a[0].len();

    let k = 2;
    let o = 4;

    let r: Vec<u8> = sample_rand(k, o);
    let mut x: Vec<u8> = r.clone();


    // Perform y = y - Ar in single iteration over y and A
    let mut y: Vec<u8> = a.iter().zip(y.iter()).map(|(row, &y_val)| {

        let ar_val: u8 = row.iter().zip(r.iter())
            .map(|(a_row_idx, r_idx)| ff::mul(*a_row_idx, *r_idx))
            .sum(); // Compute the dot product of the current row of A and r
        ff::sub(y_val, ar_val) // Perform subtraction y - Ar
    }).collect(); // Collect new vector of size m


    // Append the first element of y to the first row of A, the second element of y to the second row of A etc.
    for (row, &y_val) in a.iter_mut().zip(y.iter()) {
        row.push(y_val);
    }

    // Put (A | y) in echelon form with leading 1's.
    a = echelon_form(a, 2, 4);


    if a[rows].iter().all(|&i| {i == 0} ) {
        return Err("The matrix A does not have full rank.");
    }
 
    // Back-substitution
    // Create affine transformation known from Oil and Vinegar
    for r in (0..cols).rev() {
        // Let c be the index of first non-zero element of A[r,:]
        let c = a[r].iter().position(|&i| i != 0).unwrap();
        x[c] = ff::add(x[c], y[r]);

        // Calc x_c = x_c + y[r]
        let temp_mult = a.iter().map(|row| {
            ff::mul(y[r], row[c])
        }).collect::<Vec<u8>>();
        

        // Calc y = y - y[r] * A[:,c]
        y = y.iter().zip(temp_mult.iter()).map(|(y_idx, temp_mult_idx)| {
            ff::sub(*y_idx, *temp_mult_idx)
        }).collect();
    }
    Ok(x)
}



pub fn print_matrix(mat: Vec<Vec<u8>>) -> () {
    mat.iter().for_each(|f| {
        println!("{:?}", f);
})
}

// test echoleon_form
#[cfg(test)]
mod tests {
    use std::vec;
    use super::*;

    #[test]
    fn test_echelon_form_simple() {


        /* 
        Must satisfy the following conditions:
            k < n − o
            k*o ≥ m
        where: 
            m is the number of multivariate quadratic polynomials,
            n is number of variables in the multivariate polynomials, 
            k is the whipping parameter,
            o is the dimension of the oil space.
        */

        // n = 4
        // m = 2
        let k = 1; // Whipping parameter
        let o = 2; // Oil space dimension

        // Matrix in GF(16)
        let b = vec![
            vec![0x0, 0x1, 0x8, 0x4, 0x5], 
            vec![0x1, 0x2, 0x3, 0x4, 0x5], 
            // SHOULD SWAP THESE TWO ROWS
        ];

        // Expected result after echelon form transformation (The two rows should be swapped)
        let expected = vec![
            vec![0x1, 0x2, 0x3, 0x4, 0x5], 
            vec![0x0, 0x1, 0x8, 0x4, 0x5], 
        ];

        let result = echelon_form(b, k, o);

        assert_eq!(result, expected, "Echelon form did not match expected result");
    }


    #[test]
    fn test_echelon_form_more_complex() {


        /* 
        Must satisfy the following conditions:
            k < n − o
            k*o ≥ m
        where: 
            m is the number of multivariate quadratic polynomials,
            n is number of variables in the multivariate polynomials, 
            k is the whipping parameter,
            o is the dimension of the oil space.
        */

        // n = 8
        // m = 4
        let k = 2; // Whipping parameter
        let o = 4; // Oil space parameter


        // Example matrix in GF(16), represented as u8
        let b = vec![
            vec![0x4, 0x4, 0x4, 0x4, 0x5, 0x5, 0x5, 0x5, 0x6], 
            vec![0x2, 0x4, 0x6, 0x4, 0x2, 0x4, 0x6, 0x4, 0x8], 
            vec![0x3, 0x6, 0x5, 0x4, 0x3, 0x6, 0x5, 0x4, 0x7],
            vec![0x0, 0x2, 0x3, 0x4, 0x0, 0x2, 0x3, 0x4, 0x1],  
        ];


        // Expected result after echelon form transformation
        let expected = vec![
            vec![0x1, 0x2, 0x3, 0x4], 
            vec![0x0, 0x1, 0x8, 0x4], 
            vec![0x0, 0x0, 0x1, 0x4], 
            vec![0x0, 0x0, 0x1, 0x4],
        ];

        let result = echelon_form(b, k, o);

        assert_eq!(result, expected, "Echelon form did not match expected result");
    }



}
