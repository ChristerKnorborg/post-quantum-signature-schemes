use crate::finite_field as ff;
use crate::utils as util;
use crate::constants::{K, O};
use rand::rngs::StdRng as rng;
use rand::{Rng, SeedableRng};


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

pub fn sample_rand() -> Vec<u8> {
    let num_elems: u16 = (K * O) as u16;

    // Cryptographically secure random number generation
    let mut rand_core = rng::from_entropy();
    let vals: Vec<u8> = (0..num_elems)
        .map(|_| rand_core.gen_range(0..15) as u8)
        .collect();

    return vals;
}

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

    println!("y vector: {:?}", y);

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
    use super::*;
    use std::vec;





    // Function to check if a matrix is in echelon form (not reduced).
    // A matrix is in echelon form if row entries are all zero below the leading entry.
    fn is_echelon_form(matrix: &Vec<Vec<u8>>) -> bool {
        let mut last_leading_column = None;

        for row in matrix {
            let mut row_iter = row.iter().enumerate().filter(|&(_, &val)| val != 0);

            if let Some((leading_column, _)) = row_iter.next() {
                if let Some(last_col) = last_leading_column {
                    if leading_column <= last_col {
                        // Leading entry is not to the right of the one above
                        return false;
                    }
                }
                last_leading_column = Some(leading_column);

                if row_iter.any(|(col, _)| col < leading_column) {
                    // There are non-zero entries after the leading entry
                    return false;
                }
            }
        }

        // All entries below leading entries must be zero
        for col in 0..matrix[0].len() {
            let mut found_nonzero = false;
            for row in matrix {
                if found_nonzero && row[col] != 0 {
                    // Found a non-zero entry below a leading entry
                    return false;
                }
                if row[col] != 0 {
                    found_nonzero = true;
                }
            }
        }

        true
    }





    // Test method to check if a matrix is in reduced echelon form.
    // A matrix is in reduced echelon form if all leading row entries are 1 and all other entries in the same column below the leading entry are 0.
    fn is_reduced_echelon_form(matrix: &Vec<Vec<u8>>) -> bool {
        if !is_echelon_form(matrix) {
            // If it's not in echelon form, it can't be in reduced echelon form.
            return false;
        }

        for (i, row) in matrix.iter().enumerate() {
            if let Some(leading_one_index) = row.iter().position(|&x| x == 1) {
                // Ensure the leading one is the only non-zero entry in its column
                if matrix.iter().enumerate().any(|(j, other_row)| j != i && other_row[leading_one_index] != 0) {
                    return false;
                }
                // Ensure that all leading ones are to the right of the leading one in the row above
                if i > 0 && matrix[i - 1].iter().position(|&x| x == 1) >= Some(leading_one_index) {
                    return false;
                }
            } else {
                // If there's no leading one and the row is not all zeros, it's not in reduced form
                if row.iter().any(|&x| x != 0) {
                    return false;
                }
            }
        }

        return true
    }






























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


        // Matrix in GF(16)
        let b = vec![
            vec![0x0, 0x1, 0x8, 0x4, 0x5],
            vec![0x1, 0x2, 0x3, 0x4, 0x5],
            // SHOULD SWAP THESE TWO ROWS
        ];

        // Expected result after echelon form transformation (The two rows should be swapped)
        let expected = vec![vec![0x1, 0x2, 0x3, 0x4, 0x5], vec![0x0, 0x1, 0x8, 0x4, 0x5]];

        let result = echelon_form(b);

        assert_eq!(
            result, expected,
            "Echelon form did not match expected result"
        );
    }






    #[test]
    fn test_echelon_form() {
        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            let rows = rng.gen_range(1..10);
            let cols = rng.gen_range(1..10);
            let mut matrix = vec![vec![0u8; cols]; rows];

            // Fill the matrix with random numbers
            for row in &mut matrix {
                for elem in row.iter_mut() {
                    *elem = rng.gen_range(0..=15);
                }
            }

            let echelon_matrix = echelon_form(matrix);
            assert!(is_echelon_form(&echelon_matrix));
        }
    }



    #[test]
    fn test_sample_solution() {
        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            let rows = rng.gen_range(1..10);
            let cols = rng.gen_range(1..10);
            let mut a = vec![vec![0u8; cols]; rows];

            // Fill the matrix with random numbers
            for row in &mut matrix {
                for elem in row.iter_mut() {
                    *elem = rng.gen_range(0..=15);
                }
            }

            let expected: Vec<u8> = (0..rows).map(|_| rng.gen_range(0..=15)).collect();


            let solution_matrix = match sample_solution(matrix, expected.clone()) {
                Ok(x) => x, // If Ok, destructure the tuple into `a` and `x`
                Err(e) => {
                    println!("Error: {}", e);
                    return; // Exit the function early in case of error.
                }
            };


            //assert!(is_reduced_echelon_form(solution_matrix));
        }
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

        let result = echelon_form(b);

        assert_eq!(
            result, expected,
            "Echelon form did not match expected result"
        );
    }

    #[test]
    fn test_sample_solution_OLD() {
        //TODO maybe make it run 20 times every time
        let mut rand_test = rng::from_entropy();
        // Input matrix A
        let mut a = vec![
            vec![0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x4],
            vec![0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xf],
            vec![0x8, 0x9, 0xf, 0xe, 0x3, 0x4, 0x5, 0x6],
            vec![0x8, 0x9, 0x5, 0x6, 0xe, 0xa, 0xb, 0x1],
        ];
        // Input vector y (hash/tag)
        let mut expected = vec![0x1, 0x2, 0x3, 0x4];

        for row in a.iter_mut() {
            for elem in row.iter_mut() {
                *elem = rand_test.gen_range(0..15);
            }
        }

        let a_input = a.clone();

        for elem in expected.iter_mut() {
            *elem = rand_test.gen_range(0..15);
        }

        println!("Input Matrix: {:?}", a);
        println!("Input Vector (hash): {:?}", expected);

        let x: Vec<u8> = match sample_solution(a, expected.clone()) {
            Ok(x) => x, // If Ok, destructure the tuple into `a` and `x`
            Err(e) => {
                println!("Error: {}", e);
                return; // Exit the function early in case of error.
            }
        };

        let a_times_x_equal_y: Vec<u8> = a_input
            .iter()
            .map(|row| {
                row.iter()
                    .zip(x.iter())
                    .map(|(a_row_idx, x_idx)| ff::mul(*a_row_idx, *x_idx))
                    .fold(0, |acc, x| ff::add(acc, x))
            })
            .collect();

        println!("Ax_eq_y: {:?}", a_times_x_equal_y);
        println!("expected: {:?}", expected);

        assert_eq!(
            a_times_x_equal_y, expected,
            "Echelon form did not match expected result"
        );
    }
}
