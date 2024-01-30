

// Function to perform the echelon form algorithm on matrix B
pub fn echelon_form(mut b: Vec<Vec<f64>>, k: usize, o: usize) -> Vec<Vec<f64>> {
    let m = b.len(); // Number of rows
    let n = k * o + 1; // Number of columns based on the given parameters k and o
    let mut pivot_row = 0;
    let mut pivot_column = 0;

    while pivot_row < m && pivot_column < n {
        // Find the pivot
        let possible_pivots: Vec<usize> = (pivot_row..m)
            .filter(|&i| b[i][pivot_column] != 0.0)
            .collect();

        if possible_pivots.is_empty() {
            // No pivot in this column, move to the next
            pivot_column += 1;
            continue;
        }

        let next_pivot_row = *possible_pivots.iter().min().unwrap();

        // Swap rows
        b.swap(pivot_row, next_pivot_row);

        // Make the leading entry a "1"
        let leading_entry = b[pivot_row][pivot_column];
        for j in 0..n {
            b[pivot_row][j] /= leading_entry;
        }

        // Eliminate entries below the pivot
        for i in pivot_row + 1..m {
            let factor = b[i][pivot_column];
            for j in pivot_column..n {
                b[i][j] -= factor * b[pivot_row][j];
            }
        }

        pivot_row += 1;
        pivot_column += 1;
    }

    b
}






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