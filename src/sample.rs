use crate::finite_field::{add, inv, mul, sub};
use crate::constants::{M, K, O};
use crate::{matrix_vec_mul, vec_add};


// MAYO Algorithm 1: Echelon Form
// Function to perform the echelon form algorithm on matrix B.
pub fn echelon_form(mut b: [[u8 ; K*O+1]; M]) -> [[u8 ; K*O+1]; M] {
    const ROWS: usize = M;
    const COLS: usize = K*O+1;
    let mut pivot_row = 0;
    let mut pivot_column = 0;

    while pivot_row < ROWS && pivot_column < COLS {
        // Find the first possible pivot in the current column
        let mut next_pivot_row = None;
        for i in pivot_row..ROWS {
            if b[i][pivot_column] != 0 {
                next_pivot_row = Some(i);
                break; // Optimized to break early if pivot is found 
            }
        }
        let next_pivot_row = match next_pivot_row {
            Some(row) => row,
            None => {
                pivot_column += 1; // Move to next column if there is no pivot
                continue;
            }
        };
        b.swap(pivot_row, next_pivot_row); // Swap rows

        // Make the leading entry a "1" by multiplying the row by the inverse of the pivot
        let inv_idx = inv(b[pivot_row][pivot_column]);
        for j in pivot_column..COLS {
            b[pivot_row][j] = mul(inv_idx, b[pivot_row][j]);
        }

        // Eliminate entries below the pivot
        for i in pivot_row + 1..ROWS {
            // From next pivot row to m - 1
            let factor = b[i][pivot_column];
            for j in pivot_column..COLS {
                let mult = mul(factor, b[pivot_row][j]);
                let res = sub(b[i][j], mult);
                b[i][j] = res // b[i][j] - (factor * b[pivot_row][j]);
            }
        }
        pivot_row += 1;
        pivot_column += 1;
    }
    return b
}








// MAYO Algorithm 2: Sample Solution
// Function to solve the equation Ax = y in GF(16) using gaussian elimination.
pub fn sample_solution(a: [[u8; K * O]; M], y: [u8; M], r: [u8; K*O]) -> Result<[u8; K*O], &'static str> {


    let mut x: [u8; K*O] = r.clone();
    let mut temp = matrix_vec_mul!(a, x, M, K*O); //  (m x K*O) * (K*O x 1) = (m x 1)
    temp = vec_add!(temp, y, M); // Add same as subtracting in GF(16)



    let mut pre_ech_a: [[u8; K*O + 1]; M] = [[0u8 ; K*O + 1] ; M];

    for i in 0..M {
        pre_ech_a[i][..K*O].copy_from_slice(&a[i]);
        pre_ech_a[i][K*O] = temp[i];
    }


    // Put (A | y) in echelon form with leading 1's.
    let a: [[u8; K*O+1]; M] = echelon_form(pre_ech_a);


    // Split the matrix into A and y
    let mut a_ech: [[u8; K*O]; M] = [[0; K*O]; M];
    let mut y_ech: [u8; M] = [0; M];
    for (i, row) in a.iter().enumerate() {
        a_ech[i].copy_from_slice(&row[..K*O]);
        y_ech[i] = row[K*O];
    }

    // Check if the matrix A has full rank (E.g. no full row of zeros in the echelon form)
    if a_ech[M - 1].iter().all(|&i| i == 0) {
        return Err("The matrix A does not have full rank. No solution is found");
    }

    // Back-substitution
    for r in (0..M).rev() {
        // Let c be the index of first non-zero element of A[r, :]
        let c = a_ech[r].iter().position(|&i| i != 0).unwrap();
        x[c] = add(x[c], y_ech[r]);

        // Prepare for updating y_ech
        let mut temp_mult: [u8; M] = [0; M]; // Initialize an array with zeros

        for (i, row) in a_ech.iter().enumerate() {
            temp_mult[i] = mul(y_ech[r], row[c]);
        }

        vec_add!(y_ech, temp_mult, M); // Add same as subtracting in GF(16)
    }
    Ok(x)
}













// test echoleon_form
#[cfg(test)]
mod tests {

    use crate::constants::M;
    use rand::Rng;
    use super::*;


    // Function to check if a matrix is in echelon form (not reduced).
    // A matrix is in echelon form if row entries are all zero below the leading entry.
    fn is_echelon_form(matrix: &[[u8; K*O+1]; M]) -> bool {
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
                    // A fully zero row, check the rest of the matrix for any non-zero row
                    let mut rest_of_matrix = matrix.iter().skip_while(|&r| r != row).skip(1);
                    if rest_of_matrix.any(|subsequent_row| subsequent_row.iter().any(|&x| x != 0)) {
                        return false;
                    }
                    // If a zero row exists and no non-zero row after it, matrix is still potentially in echelon form, continue checking
                    // Note: If this condition is met, and there are no more rows, the loop will naturally end.
                },
            }
        }
        true
    }













    #[test]
    fn test_echelon_form() {
        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            let mut matrix = [[0u8; K*O+1]; M];

            // Fill the matrix with random numbers
            for i in 0..M {
                for j in 0..(K*O+1) {
                    matrix[i][j] = rng.gen_range(0..=15);
                }
            }

            let echelon_matrix = echelon_form(matrix.clone());
           // print_matrix(echelon_matrix.clone());
            assert!(is_echelon_form(&echelon_matrix));
        }
    }



    #[test]
    fn test_sample_solution() {


        pub fn sample_rand() -> [u8; K*O] {
            let mut rng = rand::thread_rng(); // Using `rand` crate's thread_rng for simplicity
            let mut vals = [0u8; K*O]; // Initialize an array with N elements
        
            for i in 0..K*O {
                vals[i] = rng.gen_range(0..16) as u8; // GF(16) implies values from 0 to 15
            }
        
            vals
        }


        let mut rng = rand::thread_rng();
        for _ in 0..50 {

            // Generate a random matrix of size (rows, cols)
            let mut a = [[0u8; K*O]; M];

            // Fill the matrix with random numbers
            for row in &mut a {
                for elem in row.iter_mut() {
                    *elem = rng.gen_range(0..=15); // Random number in GF(16)
                }
            }

            let a_input = a.clone(); // Clone the matrix for result comparison

            // Expected result aka. y
            let mut expected = [0u8; M]; 
            for i in 0..M {
                expected[i] = rng.gen_range(0..=15); // Fill each element with a random value
            } 

            let r = sample_rand();

            match sample_solution(a, expected.clone(), r) {
                Ok(x) => {
                    // Calculate the result of Ax = y for comparison
                    let a_times_x_equal_y: Vec<u8> = a_input
                        .iter()
                        .map(|row| {
                            row.iter()
                                .zip(x.iter())
                                .map(|(a_row_idx, x_idx)| mul(*a_row_idx, *x_idx))
                                .fold(0, |acc, x| add(acc, x))
                        })
                        .collect();


                    /* println!("Ax_eq_y:  {:?}", a_times_x_equal_y);
                    println!("expected: {:?}", expected); */

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
