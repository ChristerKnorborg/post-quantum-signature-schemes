use crate::constants::{L_BYTES, M, O, O_BYTES, P1_BYTES, P2_BYTES, V};
use crate::utils::{write_u32_array_to_file_byte, write_u32_array_to_file_int, write_u8_array_to_file_byte, write_u8_array_to_file_int};








pub fn p1_p1t_times_o_plus_p2(p1: &[u8], o: &[[u8 ; O] ; V], p2: &[u8]) -> [u32 ; L_BYTES/4] {
    

    let mut p1_u32 = [0u32 ; P1_BYTES/4];
    for (i, chunk) in p1.chunks(4).enumerate() {
        let mut array = [0u8; 4];
        array.copy_from_slice(chunk);
        p1_u32[i] = u32::from_le_bytes(array); // Use from_be_bytes for big endian
    }


    let mut p2_u32 = [0u32 ; P2_BYTES/4];
    for (i, chunk) in p2.chunks(4).enumerate() {
        let mut array = [0u8; 4];
        array.copy_from_slice(chunk);
        p2_u32[i] = u32::from_le_bytes(array); // Use from_be_bytes for big endian
    }


    

    // Size is slightly less than P1_BYTES/2 as there must be space for everything below the diagonals of the m p1 matrices.
    // Divide by 8 since we transform to u32 (e.g. divede by 4).
    // Also, we can represent 2 nibbles in a single byte with encoding (e.g. divide by 2).
    const ADDED_P1_SIZE: usize = V*V*M/8;

    let mut p1_p1t_added = [0u32 ; ADDED_P1_SIZE];
    let u32s_per_idx = M / 2 / 4; // number of u32 to represent a single index for all m matrices
    
    let mut entries_used = 0;
    // Add P1 and P1 transposed
    for r in 0..V {
        for c in r..V {

            // Set diagonal to 0 (entries where i=j)to 0 for all m matrices (at memory location)

            // Set remaining entries [i, j] to be [j, i]. As, all entries are 0 in the lower half of p1,
            // and all entries are 0 in the upper half of p1_transposed.
            if r != c {
                let start = u32s_per_idx * (r * V + c);
                for i in 0..(u32s_per_idx) {
                    p1_p1t_added[start+i] = p1_u32[u32s_per_idx * entries_used + i];
                }

                let start = u32s_per_idx * (c * V + r);
                for i in 0..(u32s_per_idx) {
                    p1_p1t_added[start+i] = p1_u32[u32s_per_idx * entries_used + i];
                }
            }
            entries_used += 1;
        }
    }


    let mut entries_used = 0; 

    // Iterat over all indexes of p1_p1t as it is NOT upper triangular.
    for r in 0..V {
        for c in 0..V {
            for k in 0..O { // Iterate over all nibbles in the current column of O
                let p1_p1t_start_idx = u32s_per_idx * entries_used;
                let p2_acc_start_idx = u32s_per_idx * (r * O + k);
                
                mul_add_bitsliced_m_vec(&p1_p1t_added, p1_p1t_start_idx, o[c][k], &mut p2_u32, p2_acc_start_idx);
            }
            entries_used += 1;
        }
    }    
    return p2_u32
}


fn mul_add_bitsliced_m_vec(input: &[u32], input_start: usize, nibble: u8, acc: &mut [u32], acc_start: usize) {


    const U32_PER_TERM: usize = M/32; // Number of u32 in a term of the polynomial. E.g. 32 for M=128

    // Terms of the nibble x^3 + x^2 + x + 1. 
    // Create a mask for the nibble of 32 bits for each of the 4 degrees. E.g. 1001 becomes:
    // a0 = 11111111 11111111 11111111 11111111, a1 = 00000000 00000000 00000000 00000000 etc.
    let x0: u32 = ((nibble & 1) != 0) as u32 * u32::MAX;
    let x1: u32 = (((nibble >> 1) & 1) != 0) as u32 * u32::MAX;
    let x2: u32 = (((nibble >> 2) & 1) != 0) as u32 * u32::MAX;
    let x3: u32 = (((nibble >> 3) & 1) != 0) as u32 * u32::MAX;

    for i in 0..U32_PER_TERM {

        let input_idx0 = input_start + i;
        let input_idx1 = input_start + U32_PER_TERM + i;
        let input_idx2 = input_start + 2 * U32_PER_TERM + i;
        let input_idx3 = input_start + 3 * U32_PER_TERM + i;


        let acc_idx0 = acc_start + i;
        let acc_idx1 = acc_start + U32_PER_TERM + i;
        let acc_idx2 = acc_start + 2 * U32_PER_TERM + i;
        let acc_idx3 = acc_start + 3 * U32_PER_TERM + i;


        // Degree 0 term of the nibble (x^0)
        acc[acc_idx0] ^= x0 & input[input_idx0];
        acc[acc_idx1] ^= x0 & input[input_idx1];
        acc[acc_idx2] ^= x0 & input[input_idx2];
        acc[acc_idx3] ^= x0 & input[input_idx3]; 

        // Degree 1 term of the nibble (x^1)
        let a: u32 = input[input_idx0] ^ input[input_idx3];
        acc[acc_idx0] ^= x1 & input[input_idx3];
        acc[acc_idx1] ^= x1 & a;
        acc[acc_idx2] ^= x1 & input[input_idx1];
        acc[acc_idx3] ^= x1 & input[input_idx2];

        // Degree 2 term of the nibble (x^2)
        let b: u32 = input[input_idx3] ^ input[input_idx2];
        acc[acc_idx0] ^= x2 & input[input_idx2];
        acc[acc_idx1] ^= x2 & b;
        acc[acc_idx2] ^= x2 & a;
        acc[acc_idx3] ^= x2 & input[input_idx1];

        // Degree 3 term of the nibble (x^3)
        let c: u32 = input[input_idx2] ^ input[input_idx1];
        acc[acc_idx0] ^= x3 & input[input_idx1];
        acc[acc_idx1] ^= x3 & c;
        acc[acc_idx2] ^= x3 & b;
        acc[acc_idx3] ^= x3 & a;
    }
}

