/* 
    This file contains code heavily inspired by the MAYO C implementation for NIST found at: https://github.com/PQCMayo/MAYO-C.
    Much of this code is adapted from the original C implementation to fit our Rust implementation for doing bitsliced arithmetic 
*/
use crate::constants::{M, O, V};
use crate::crypto_primitives::{safe_mul_add_bitsliced_m_vec, safe_mul_add_bitsliced_m_vec_mayo1};



const U32_PER_IDX: usize = M / 2 / 4; // number of u32 to represent a single index for all m matrices



/// Performs multiplication of a bitsliced matrix (`$bs_mat`) that is possibly upper triangular
/// with a standard matrix (`$mat`), and adds the result to an accumulator (`$acc`).
#[macro_export]
macro_rules! bitsliced_mat_mul_mat_add {
    ($bs_mat:expr, $mat:expr, $acc:expr, $bs_mat_rows:expr, $bs_mat_cols:expr, $mat_cols:expr, $upper_triangular:expr) => {{

        let mut entries_used = 0;

        let mut counter = 0;

        for r in 0..$bs_mat_rows {

            let c_start = if $upper_triangular { r } else { 0 }; 
            let uneven_last = $bs_mat_cols - c_start % 2;
            counter = 0; 
            for c in (c_start..$bs_mat_cols-2).step_by(2) {
                 // Only iterate corresponding to upper part row of bitsliced matrix (as lower part is not stored in bitsliced representation)
                 println!("c: {}", c);
                counter += 1;
                for k in 0..$mat_cols {
                    let bs_mat_start_idx = entries_used * U32_PER_IDX;
                    let acc_start_idx = (r * $mat_cols + k ) * U32_PER_IDX;

                    safe_mul_add_bitsliced_m_vec_mayo1(&$bs_mat, bs_mat_start_idx.try_into().unwrap(), $mat[c][k], $mat[c+1][k], $acc, acc_start_idx.try_into().unwrap());

                    // if (c == $bs_mat_cols-1 && extra_iteration == 1) {
                    //     safe_mul_add_bitsliced_m_vec(&$bs_mat, bs_mat_start_idx.try_into().unwrap(), $mat[c+2][k], $acc, acc_start_idx.try_into().unwrap());
                    // }
                }

                if uneven_last == 1 {
                    let bs_mat_start_idx = entries_used * U32_PER_IDX;
                    let acc_start_idx = (r * $mat_cols + $mat_cols-1 ) * U32_PER_IDX;

                    safe_mul_add_bitsliced_m_vec(&$bs_mat, bs_mat_start_idx.try_into().unwrap(), $mat[c][$mat_cols-1], $acc, acc_start_idx.try_into().unwrap());
                
                    entries_used += 1;
                } else {
                    let bs_mat_start_idx = entries_used * U32_PER_IDX;
                    let acc_start_idx = (r * $mat_cols + $mat_cols-1 ) * U32_PER_IDX;

                    safe_mul_add_bitsliced_m_vec_mayo1(&$bs_mat, bs_mat_start_idx.try_into().unwrap(), $mat[c][$mat_cols-1], $mat[c+1][$mat_cols-1], $acc, acc_start_idx.try_into().unwrap());

                    entries_used += 2;
                }


                //println!("Counter: {}", counter);    
            }
        }
    }};
}


/// Performs multiplication of a standard matrix (`$mat`) with a bitsliced matrix (`$bs_mat`),
/// and adds the result to an accumulator (`$acc`).
#[macro_export]
macro_rules! transposed_mat_mul_bitsliced_mat_add {
    ($mat:expr, $bs_mat:expr, $acc:expr, $mat_rows:expr, $mat_cols:expr, $bs_mat_cols:expr) => {{

        for r in 0..$mat_cols { // Switched rows and cols iteration to transpose mat
            for c in 0..$mat_rows {
                for k in 0..$bs_mat_cols {
                    let bs_mat_start_idx = (c * $bs_mat_cols + k) * U32_PER_IDX;
                    let acc_start_idx = (r * $bs_mat_cols + k) * U32_PER_IDX;

                    mul_add_bitsliced_m_vec($bs_mat, bs_mat_start_idx, $mat[c][r], $acc, acc_start_idx);
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! upper_triangular_bitsliced_mat_mul_transposed_mat_add {
    ($bs_mat:expr, $mat:expr, $acc:expr, $bs_mat_rows:expr, $bs_mat_cols:expr, $mat_rows:expr) => {{

        let mut entries_used = 0;
        for r in 0..$bs_mat_rows {
            for c in r..$bs_mat_cols { // Only iterate corresponding to upper part row of bitsliced matrix (as lower part is not stored in bitsliced representation)
                for k in 0..$mat_rows {
                    let bs_mat_start_idx = entries_used * U32_PER_IDX;
                    let acc_start_idx = (r * $mat_rows + k ) * U32_PER_IDX;

                    mul_add_bitsliced_m_vec(&$bs_mat, bs_mat_start_idx, $mat[k][c], $acc, acc_start_idx);
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
    ($matrix:expr, $matrix_upper:expr, $rows:expr, $cols:expr) => {
        {

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
        }
    };
}



#[macro_export]
macro_rules! mat_mul_bitsliced_mat_add {
    ($mat:expr, $bs_mat:expr, $acc:expr, $mat_rows:expr, $mat_cols:expr, $bs_mat_cols:expr) => {{
        for r in 0..$mat_rows {
            for c in 0..$mat_cols {
                for k in 0..$bs_mat_cols { // Iterate over all elements in the bitsliced matrix column
                    let bs_mat_start_idx = (c * $bs_mat_cols + k) * U32_PER_IDX;
                    let acc_start_idx = (r * $bs_mat_cols + k) * U32_PER_IDX;

                    mul_add_bitsliced_m_vec(&$bs_mat, bs_mat_start_idx, $mat[r][c], $acc, acc_start_idx);
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

            // Set diagonal to 0 (entries where i=j)to 0 for all m matrices (at memory location)

            // Set remaining entries [i, j] to be [j, i]. As, all entries are 0 in the lower half of p1,
            // and all entries are 0 in the upper half of p1_transposed.
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






// Construct the m matrices of (P_1 P_2)
//                             (0   P_3)
// in bitsliced format
pub fn create_big_p_bitsliced(p1: &[u32], p2: &[u32], p3: &[u32], big_p: &mut[u32]) {

    // Entries exhausted in p1, p2, p3 and big_p respectively
    let mut big_used = 0;
    let mut p1_used = 0;
    let mut p2_used = 0;

    const P2_ROW_SIZE: usize = O * U32_PER_IDX;

    // Set the first V rows to be p1 concatenated with p2
    for r in 0..V {

        // Assign V columns of p1 to the first V columns of the first big_p row 
        // Then V-1 columns of p1 to the next V-1 columns of the big_p etc.
        let p1_row_size = (V - r) * U32_PER_IDX;
        let p1_row = &p1[p1_used..p1_used + p1_row_size];

        big_p[big_used..big_used + p1_row_size].copy_from_slice(p1_row);

        p1_used += p1_row_size;
        big_used += p1_row_size;



        // Assign O columns of p2 to the second O columns of the current big_p row
        let p2_row = &p2[p2_used..p2_used + P2_ROW_SIZE];
        big_p[big_used..big_used + P2_ROW_SIZE].copy_from_slice(p2_row);

        p2_used += P2_ROW_SIZE;
        big_used += P2_ROW_SIZE;
    }

    // Set the last O rows to be p3
    big_p[big_used..].copy_from_slice(p3);
}








// This function (rewritten to Rust from the MAYO authors' C implementation) multiplies a bitsliced vectors of m field elements
// with a nibble. A bitsliced vector is has of m/32 * 4 consecutive u32s, where every m/32 u32s represent a term for all m elements.
pub fn mul_add_bitsliced_m_vec(input: &[u32], input_start: usize, nibble: u8, acc: &mut [u32], acc_start: usize) {


    const U32_PER_TERM: usize = M/32; // Number of u32 in a term of the polynomial.

    // Terms of the nibble x^3 + x^2 + x + 1. 
    // Create a mask for the nibble of 32 bits for each of the 4 degrees. E.g. 1001 becomes:
    // x0 = 11111111 11111111 11111111 11111111, x1 = 00000000 00000000 00000000 00000000 etc.
    let n0: u32 = ((nibble & 1) != 0) as u32 * u32::MAX;
    let n1: u32 = (((nibble >> 1) & 1) != 0) as u32 * u32::MAX;
    let n2: u32 = (((nibble >> 2) & 1) != 0) as u32 * u32::MAX;
    let n3: u32 = (((nibble >> 3) & 1) != 0) as u32 * u32::MAX;


    // In the group defined by the polynomial x^3 + x^2 + x + 1:    x^6 ≡ x^3 + x^2,   x^5 ≡ x^2 + x,   x^4 ≡ x + 1.
    // Therefore, the multiplication results are as follows:
    // x^0 = in0 * n0 
    // x^1 = in0 * n1 + in1 * n0
    // x^2 = in0 * n2 + in1 * n1 + in2 * n0
    // x^3 = in0 * n3 + in1 * n2 + in2 * n1 + in3 * n0
    // x^4 = in1 * n3 + in2 * n2 + in3 * n1
    // x^5 = in2 * n3 + in3 * n2
    // x^6 = in3 * n3 
    // In the loop below:
    //      * x^1 = x^1, x^4 (after reduction) and x^5 (after reduction)
    //      * x^2 = x^2, x^5 (after reduction) and x^6 (after reduction)
    //      * x^3 = x^3, x^6 (after reduction)
    // Therefore, acc1 (x^1), acc2 (x^2), acc (x^3) are set to all combination that result in those terms (inclusive reduced terms).

    for i in 0..U32_PER_TERM {

        let in0 = input_start + i;
        let in1 = input_start + U32_PER_TERM + i;
        let in2 = input_start + 2 * U32_PER_TERM + i;
        let in3 = input_start + 3 * U32_PER_TERM + i;


        let acc0 = acc_start + i;
        let acc1 = acc_start + U32_PER_TERM + i;
        let acc2 = acc_start + 2 * U32_PER_TERM + i;
        let acc3 = acc_start + 3 * U32_PER_TERM + i;

        
        let a: u32 = input[in0] ^ input[in3];
        let b: u32 = input[in3] ^ input[in2];
        let c: u32 = input[in2] ^ input[in1];

        // Degree 0 term of the nibble (x^0)
        acc[acc0] ^= n0 & input[in0];
        acc[acc1] ^= n0 & input[in1];
        acc[acc2] ^= n0 & input[in2];
        acc[acc3] ^= n0 & input[in3]; 

        // Degree 1 term of the nibble (x^1)
        acc[acc0] ^= n1 & input[in3];
        acc[acc1] ^= n1 & a;                    
        acc[acc2] ^= n1 & input[in1];
        acc[acc3] ^= n1 & input[in2];

        // Degree 2 term of the nibble (x^2)
        acc[acc0] ^= n2 & input[in2];
        acc[acc1] ^= n2 & b;                
        acc[acc2] ^= n2 & a;
        acc[acc3] ^= n2 & input[in1];

        // Degree 3 term of the nibble (x^3)
        acc[acc0] ^= n3 & input[in1];
        acc[acc1] ^= n3 & c;
        acc[acc2] ^= n3 & b;
        acc[acc3] ^= n3 & a;
    }
}



