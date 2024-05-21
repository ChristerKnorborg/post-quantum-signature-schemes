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


    let mut x: [u8; K*O] = r;
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


