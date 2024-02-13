use crate::finite_field as ff;
use crate::utils as util;
use crate::constants::{K, O};
use rand::rngs::StdRng as rng;
use rand::{Rng, SeedableRng};


// MAYO Algorithm 1: Echelon Form
// Function to perform the echelon form algorithm on matrix B.
pub fn echelon_form(mut b: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let rows = b.len();
    let cols = b[0].len();
    let mut pivot_row = 0;
    let mut pivot_column = 0;

    while pivot_row < rows && pivot_column < (K * O + 1) as usize {
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
        for i in pivot_row + 1..rows {
            // From next pivot row to m - 1
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

    return b
}



// Function to sample a random vector of size K*O in GF(16).
// Used to generate the random vector r in the algorithm 2
pub fn sample_rand() -> Vec<u8> {
    let num_elems: u16 = (K * O) as u16;

    // Cryptographically secure random number generation
    let mut rand_core = rng::from_entropy();
    let vals: Vec<u8> = (0..num_elems)
        .map(|_| rand_core.gen_range(0..15) as u8)
        .collect();

    return vals;
}



// MAYO Algorithm 2: Sample Solution
// Function to solve the equation Ax = y in GF(16) using gaussian elimination.
pub fn sample_solution(mut a: Vec<Vec<u8>>, mut y: Vec<u8>) -> Result<Vec<u8>, &'static str> {
    let rows = a.len();


    let r: Vec<u8> = sample_rand();
    let mut x: Vec<u8> = r.clone();

    // Perform y = y - Ar in single iteration over y and A
    y = a
        .iter()
        .zip(y.iter())
        .map(|(row, &y_val)| {
            let ar_val: u8 = row
                .iter()
                .zip(r.iter())
                .map(|(a_row_idx, r_idx)| ff::mul(*a_row_idx, *r_idx))
                .fold(0, |acc, x| ff::add(acc, x)); // Compute the dot product of the current row of A and r
            ff::sub(y_val, ar_val) // Perform subtraction y - Ar
        })
        .collect(); // Collect new vector of size m


    // Append the first element of y to the first row of A, the second element of y to the second row of A etc.
    for (row, &y_val) in a.iter_mut().zip(y.iter()) {
        row.push(y_val);
    }

    // Put (A | y) in echelon form with leading 1's.
    let a = echelon_form(a);

    // Split the matrix into A and y
    let a_ech: Vec<Vec<u8>> = a.iter().map(|row| row[0..row.len() - 1].to_vec()).collect();
    let mut y_ech: Vec<u8> = a.iter().map(|row| *row.last().unwrap()).collect();

    if a_ech[rows - 1].iter().all(|&i| i == 0) {
        return Err("The matrix A does not have full rank. No solution is found");
    }

    // Back-substitution
    for r in (0..rows).rev() {
        // Let c be the index of first non-zero element of A[r,:]
        // Calc x_c = x_c + y[r]
        let c = a_ech[r].iter().position(|&i| i != 0).unwrap();
        x[c] = ff::add(x[c], y_ech[r]);

        // Calc temp_mult = y[r] * A[:,c]
        let temp_mult: Vec<u8> = a_ech.iter().map(|row| ff::mul(y_ech[r], row[c])).collect();

        // Calc y = y - y[r] * A[:,c]
        y_ech = y_ech
            .iter()
            .zip(temp_mult.iter())
            .map(|(y_idx, temp_mult_idx)| ff::sub(*y_idx, *temp_mult_idx))
            .collect();
    }

    return Ok(x)
}













// test echoleon_form
#[cfg(test)]
mod tests {
    use crate::{constants::M, utils::print_matrix};

    use super::*;
    use std::vec;





    // Function to check if a matrix is in echelon form (not reduced).
    // A matrix is in echelon form if row entries are all zero below the leading entry.
    fn is_echelon_form(matrix: &Vec<Vec<u8>>) -> bool {

        let mut last_leading_col: isize = -1; // Track column index of leading entry in the previous row
        
        for row in matrix.iter() {

            // Find the index of the first non-zero element in the current row
            let current_leading_col = row.iter().position(|&x| x != 0);
            
            match current_leading_col {
                Some(idx) => {
                    // Not in echelon form if the current leading element is to the left or in the same column as the last
                    if idx as isize <= last_leading_col {
                        return false;
                    }
                    // Update last_leading_col to the current row's leading element column index
                    last_leading_col = idx as isize;
                },
                None => {
                    // Checks if there is a non-zero row after finding a full zero row (then not in echelon form)
                    for subsequent_row in matrix.iter().skip_while(|&r| r != row).skip(1) {
                        if subsequent_row.iter().any(|&x| x != 0) {
                            return false;
                        }
                    }
                    // If zero row exist and no non-zero row after it, matrix is in echelon form and we can break
                    break;
                },
            }
        }
        return true
    }













    #[test]
    fn test_echelon_form() {
        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            let rows = M;
            let cols = K * O;
            let mut matrix = vec![vec![0u8; cols]; rows];

            // Fill the matrix with random numbers
            for row in &mut matrix {
                for elem in row.iter_mut() {
                    *elem = rng.gen_range(0..=15);
                }
            }

            let echelon_matrix = echelon_form(matrix.clone());
            print_matrix(echelon_matrix.clone());
            assert!(is_echelon_form(&echelon_matrix));
        }
    }



    #[test]
    fn test_sample_solution() {

        let mut rng = rand::thread_rng();
        for _ in 0..50 {

            // Generate a random matrix of size (rows, cols)
            let rows = M;
            let cols = K * O;
            let mut a = vec![vec![0u8; cols]; rows];

            // Fill the matrix with random numbers
            for row in &mut a {
                for elem in row.iter_mut() {
                    *elem = rng.gen_range(0..=15); // Random number in GF(16)
                }
            }

            let a_input = a.clone(); // Clone the matrix for result comparison
            let expected: Vec<u8> = (0..rows).map(|_| rng.gen_range(0..=15)).collect(); // Expected result aka. y

            match sample_solution(a, expected.clone()) {
                Ok(x) => {


                    // Calculate the result of Ax = y for comparison
                    let a_times_x_equal_y: Vec<u8> = a_input
                        .iter()
                        .map(|row| {
                            row.iter()
                                .zip(x.iter())
                                .map(|(a_row_idx, x_idx)| ff::mul(*a_row_idx, *x_idx))
                                .fold(0, |acc, x| ff::add(acc, x))
                        })
                        .collect();


                    println!("Ax_eq_y:  {:?}", a_times_x_equal_y);
                    println!("expected: {:?}", expected);

                    assert_eq!(
                        a_times_x_equal_y, expected,
                        "Echelon form did not match expected result"
                    );

                },
                Err(_e) => {
                    continue; // Test next randomly generated matrix in case no solution found.
                }
            };

        }
    }


}
