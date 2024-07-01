/* 
    This file contains code heavily inspired by the MAYO C implementation for NIST found at: https://github.com/PQCMayo/MAYO-C.
    Much of this code is adapted from the original C implementation to fit our Rust implementation for doing bitsliced arithmetic 
*/
#[allow(unused_imports)]
use crate::crypto_primitives::{safe_mul_add_bitsliced_m_vec_mayo12, safe_mul_add_bitsliced_m_vec_mayo3, safe_mul_add_bitsliced_m_vec_mayo5};
use crate::constants::{K, M, N, O, P1_BYTES, P2_BYTES, P3_BYTES, V};

const U32_PER_TERM: usize = M/32; // Number of u32 to represent a single polynomialterm in the bitsliced vector.
const U32_PER_IDX: usize = U32_PER_TERM * 4; // number of u32 to represent a single index for all m matrices



// Performs multiplication of a bitsliced matrix (`$bs_mat`) that is possibly upper triangular
// with a standard matrix (`$mat`), and adds the result to an accumulator (`$acc`).
#[macro_export]
macro_rules! bitsliced_mat_mul_mat_add {
    ($bs_mat:expr, $mat:expr, $acc:expr, $bs_mat_rows:expr, $bs_mat_cols:expr, $mat_cols:expr, $upper_triangular:expr) => {{

        let mut entries_used = 0;

        for r in 0..$bs_mat_rows { 

            let c_start = if $upper_triangular { r } else { 0 }; 
            for c in (c_start..$bs_mat_cols) {
                 for k in 0..$mat_cols {

                    let bs_mat_start_idx = entries_used * U32_PER_IDX;
                    let acc_start_idx = (r * $mat_cols + k) * U32_PER_IDX;

                    #[cfg(any(feature = "mayo1", feature = "mayo2"))]
                    {
                        safe_mul_add_bitsliced_m_vec_mayo12(&$bs_mat, bs_mat_start_idx.try_into().unwrap(), $mat[c][k], $acc, acc_start_idx.try_into().unwrap());
                    }

                    #[cfg(feature = "mayo3")]
                    {
                        safe_mul_add_bitsliced_m_vec_mayo3(&$bs_mat, bs_mat_start_idx.try_into().unwrap(), $mat[c][k], $acc, acc_start_idx.try_into().unwrap());
                    }
                    
                    #[cfg(feature = "mayo5")]
                    {
                        safe_mul_add_bitsliced_m_vec_mayo5(&$bs_mat, bs_mat_start_idx.try_into().unwrap(), $mat[c][k], $acc, acc_start_idx.try_into().unwrap());
                    }
                }
                entries_used += 1;
            }
        }
    }};
}



#[macro_export]
macro_rules! transposed_mat_mul_bitsliced_mat_add {
    ($mat:expr, $bs_mat:expr, $acc:expr, $mat_rows:expr, $mat_cols:expr, $bs_mat_cols:expr) => {{
        
        for r in 0..$mat_cols {  // Transpose means we treat each column of $mat as a row
            for c in (0..$mat_rows) {  
                for k in 0..$mat_cols {
                    
                    let bs_mat_start_idx = (c * $bs_mat_cols + k) * U32_PER_IDX;
                    let acc_start_idx = (r * $bs_mat_cols + k) * U32_PER_IDX; 
                    
                    #[cfg(any(feature = "mayo1", feature = "mayo2"))]
                    {
                        safe_mul_add_bitsliced_m_vec_mayo12(&$bs_mat, bs_mat_start_idx.try_into().unwrap(), $mat[c][r], $acc, acc_start_idx.try_into().unwrap());
                    }
                    #[cfg(feature = "mayo3")]
                    {
                        safe_mul_add_bitsliced_m_vec_mayo3(&$bs_mat, bs_mat_start_idx.try_into().unwrap(), $mat[c][r],  $acc, acc_start_idx.try_into().unwrap());
                    }    
                    #[cfg(feature = "mayo5")]
                    {
                        safe_mul_add_bitsliced_m_vec_mayo5(&$bs_mat, bs_mat_start_idx.try_into().unwrap(), $mat[c][r],  $acc, acc_start_idx.try_into().unwrap());
                    }    
                }
            }
        }
    }};
}


#[macro_export]
macro_rules! bitsliced_mat_mul_transposed_mat_add {
    ($bs_mat:expr, $mat:expr, $acc:expr, $bs_mat_rows:expr, $bs_mat_cols:expr, $mat_rows:expr, $acc_offset:expr, $upper_triangular:expr) => {{

        let mut entries_used = 0;
        for r in 0..$bs_mat_rows {

            let c_start = if $upper_triangular { r } else { 0 };
            for c in (c_start .. $bs_mat_cols) {
                 for k in 0..$mat_rows {
                    let bs_mat_start_idx = entries_used * U32_PER_IDX;
                    let acc_start_idx = (r * $mat_rows + k) * U32_PER_IDX + $acc_offset;

                    #[cfg(any(feature = "mayo1", feature = "mayo2"))]
                    {
                        safe_mul_add_bitsliced_m_vec_mayo12(&$bs_mat, bs_mat_start_idx.try_into().unwrap(), $mat[k][c], $acc, acc_start_idx.try_into().unwrap());
                    }
                    #[cfg(feature = "mayo3")]
                    {
                        safe_mul_add_bitsliced_m_vec_mayo3(&$bs_mat, bs_mat_start_idx.try_into().unwrap(), $mat[k][c], $acc, acc_start_idx.try_into().unwrap());
                    }
                    #[cfg(feature = "mayo5")]
                    {
                        safe_mul_add_bitsliced_m_vec_mayo5(&$bs_mat, bs_mat_start_idx.try_into().unwrap(), $mat[k][c], $acc, acc_start_idx.try_into().unwrap());
                    }
                }
                entries_used += 1;
            }
        }
    }};
}









/// Method to apply the upper function to a matrix where every entry 
/// Matrix[i][j] = Matrix[j][i] + Matrix[i][j] for i>j and Matrix[i][j] = 0 for i=j
#[macro_export]
macro_rules! upper {
    ($matrix:expr, $matrix_upper:expr, $rows:expr, $cols:expr) => {{
        
        let mut entries_used = 0;
        // Iterate over everything above the diagonal
        for r in 0..$rows{
            for c in r..$cols {
                for curr_u32 in 0..U32_PER_IDX {
                    $matrix_upper[U32_PER_IDX * entries_used + curr_u32] = $matrix[U32_PER_IDX * (r * $cols + c) + curr_u32];
                }

                if r != c {
                    for curr_u32 in 0..U32_PER_IDX {
                        // add entry i,j and j,i in the upper part of matrix
                        $matrix_upper[U32_PER_IDX * entries_used + curr_u32] ^= $matrix[U32_PER_IDX * (c * $cols + r) + curr_u32];
                    }
                }
                entries_used += 1;
            }
        }
    }};
}



#[macro_export]
macro_rules! mat_mul_bitsliced_mat_add {
    ($mat:expr, $bs_mat:expr, $acc:expr, $mat_rows:expr, $mat_cols:expr, $bs_mat_cols:expr) => {{
        for r in 0..$mat_rows {
            
            for c in (0..$mat_cols) {
                for k in (0..$bs_mat_cols) { 
                    let bs_mat_start_idx = (c * $bs_mat_cols + k) * U32_PER_IDX;
                    let acc_start_idx = (r * $bs_mat_cols + k) * U32_PER_IDX;
                    
                    #[cfg(any(feature = "mayo1", feature = "mayo2"))]
                    {  
                        safe_mul_add_bitsliced_m_vec_mayo12(&$bs_mat, bs_mat_start_idx.try_into().unwrap(), $mat[r][c],  $acc, acc_start_idx.try_into().unwrap());
                    }
                    #[cfg(feature = "mayo3")]
                    {  
                        safe_mul_add_bitsliced_m_vec_mayo3(&$bs_mat, bs_mat_start_idx.try_into().unwrap(), $mat[r][c],  $acc, acc_start_idx.try_into().unwrap());
                    }
                    #[cfg(feature = "mayo5")]
                    {  
                        safe_mul_add_bitsliced_m_vec_mayo5(&$bs_mat, bs_mat_start_idx.try_into().unwrap(), $mat[r][c],  $acc, acc_start_idx.try_into().unwrap());
                    }

                }
            }
        }
    }};
}



pub fn p1_add_p1t(p1: &[u32], p1_p1t_added: &mut [u32]) {

    let mut entries_used = 0;
    // Add P1 and P1 transposed
    for r in 0..V {
        for c in r..V {

            if r != c {
                let start = U32_PER_IDX * (r * V + c);
                for i in 0..(U32_PER_IDX) {
                    p1_p1t_added[start+i] = p1[U32_PER_IDX * entries_used + i];
                }

                let start = U32_PER_IDX * (c * V + r);
                for i in 0..(U32_PER_IDX) {
                    p1_p1t_added[start+i] = p1[U32_PER_IDX * entries_used + i];
                }
            }
            entries_used += 1;
        }
    }
}


pub fn calculate_st_p(p1: [u32 ; P1_BYTES/4], p2: [u32 ; P2_BYTES/4], p3: [u32 ; P3_BYTES/4], s: [[u8 ; N] ; K]) -> [u32 ; N * K * M/8]{
    
    let mut st_p = [0u32; N * K * M/8];

    // Define s1 and s2 as 2D arrays
    let mut s1 = [[0; V]; K];
    let mut s2  = [[0; O]; K];

    for r in 0..K {
        for c in 0..V {
            s1[r][c] = s[r][c];
        }

        for c in 0..O {
            s2[r][c] = s[r][V + c];
        }
    }

    const P3_OFFSET: usize = V * K * U32_PER_TERM * 4;

    bitsliced_mat_mul_transposed_mat_add!(p1, s1, &mut st_p, V, V, K, 0, true);  // P1 * S1
    bitsliced_mat_mul_transposed_mat_add!(p2, s2, &mut st_p, V, O, K, 0, false); // P2 * S2
    bitsliced_mat_mul_transposed_mat_add!(p3, s2, &mut st_p, O, O, K, P3_OFFSET, true);  // P3 * S2
    return st_p;
}