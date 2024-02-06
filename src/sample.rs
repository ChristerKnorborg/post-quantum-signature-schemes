use crate::finite_field as ff;



// Function to perform the echelon form algorithm on matrix B (E.g. make an affine transformation matrix)
pub fn echelon_form(mut b: Vec<Vec<u8>>, k: usize, o: usize) -> Vec<Vec<u8>> {
    let rows = b.len(); // Number of rows
    let cols = k * o + 1; // Number of columns based on the given parameters k and o
    let mut pivot_row = 0;
    let mut pivot_column = 0;

    while pivot_row < rows && pivot_column < cols {
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




/*
// Sample a solution to a system of linear equations
pub fn sample_solution(a: Vec<Vec<f64>>, mut y: Vec<f64>, r: Vec<f64>) -> Result<Vec<f64>, &'static str> {
    let m = a.len();
    let n = a[0].len();

    // Randomize the system using r.
    let mut x: Vec<f64> = r.iter().take(n).cloned().collect();
    y = y.iter().zip(r.iter())
        .map(|(yi, ri)| yi - ri)
        .collect();

    // Put (A | y) in echelon form with leading 1's.
    let mut augmented_matrix = a.clone();
    for (i, row) in augmented_matrix.iter_mut().enumerate() {
        row.push(y[i]);
    }

    let echelon_matrix = echelon_form(augmented_matrix, n / m, m);

    // Check if A has rank m.
    // Assuming A[m - 1, :] is the last row of the original matrix A.
    if echelon_matrix[m - 1][..n].iter().all(|&val| val == 0.0) {
        // If the last row is all zeros, the rank is less than m.
        return Err("The matrix A does not have full rank.");
    }

    // Back-substitution
    for i in (0..m).rev() {
        let mut c = None;
        for j in 0..n {
            if echelon_matrix[i][j] != 0.0 {
                c = Some(j);
                break;
            }
        }

        if let Some(c_index) = c {
            x[c_index] = y[i];
            for j in 0..i {
                y[j] -= echelon_matrix[j][c_index] * x[c_index];
            }
        }
    }

    Ok(x)
}

 */


// test echoleon_form
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echelon_form_simple() {
        // Example matrix in GF(16), represented as u8
        let b = vec![
            vec![0x0, 0x2, 0x3, 0x4], // Represents polynomials: 0, x, x+1, x^2
            vec![0x2, 0x4, 0x6, 0x4], // Represents polynomials: x, x^2, x^2+x, x^2
        ];
        let k = 1; // Adjust based on your setup
        let o = 2; // Adjust based on your setup

        // Expected result after echelon form transformation
        // It's important to calculate these expected values based on GF(16) arithmetic rules
        let expected = vec![
            vec![0x1, 0x0, 0x0], // Adjust these values based on expected echelon form
            vec![0x0, 0x1, 0x0], // Adjust these values based on expected echelon form
        ];

        let result = echelon_form(b, k, o);

        assert_eq!(result, expected, "Echelon form did not match expected result");
    }

}
