use std::vec;


use crate::{
    constants::{L_BYTES, M, O, P3_BYTES, V},

};




 



#[macro_export]
macro_rules! encode_to_bytestring_array {
    ($x:expr, $IN_LEN:expr, $OUT_LEN:expr) => {{
        let mut bytestring = [0u8; $OUT_LEN];

        // Iterate over each element in pairs and encode them into a single byte
        for (byteindex, pair) in $x.chunks(2).enumerate().take($OUT_LEN) {
            let first_nibble = pair[0];
            let second_nibble = if pair.len() == 2 { pair[1] } else { 0 }; // If the length of x is odd, pad the last byte with a zero nibble
            // Combine the two nibbles into a single byte
            // (second_nibble is the 4 most significant bits and first_nibble is the 4 least significant bits)
            bytestring[byteindex] = second_nibble << 4 | first_nibble;
        }

        bytestring
    }};
}




#[macro_export]
macro_rules! decode_bytestring_to_array {
    ($bytestring:expr, $OUT_LEN:expr) => {{
        // Calculate the number of full bytes and if there's an extra nibble
        let mut x = [0u8; $OUT_LEN];

        let extra_nibble = $OUT_LEN % 2;
        let full_bytes = $OUT_LEN / 2;

        let mut idx = 0;
        // Iterate over all bytes with two nibbles in each
        for &byte in $bytestring.iter().take(full_bytes) {
            x[idx] = byte & 0x0F; // Put the first nibble (4 least significant bits) into the first byte
            idx += 1;
            x[idx] = byte >> 4; // Put the second nibble (4 most significant bits) into the second byte
            idx += 1;
        }

        if extra_nibble == 1 {
            // Put the first nibble (4 least significant bits) into the last byte in the array (ignore the second nibble)
            x[idx] = $bytestring.get(full_bytes).unwrap() & 0x0F;
        }
        x
    }};
}

#[macro_export]
macro_rules! decode_bytestring_matrix_array {
    ($bytestring:expr, $rows:expr, $cols:expr) => {{
        // Assuming `decode_bytestring_to_array!` macro is accessible here
        // and can be used to decode the bytestring into a flat array.
        let v = decode_bytestring_to_array!($bytestring, $rows * $cols);

        // Initialize the matrix array with zeros. Requires `Default` trait bound on the element type.
        let mut result = [[0u8; $cols]; $rows];

        // Chunk the flat array back into a matrix.
        for (i, chunk) in v.chunks($cols).enumerate() {
            result[i].copy_from_slice(chunk);
        }
        result
    }};
}









// Mayo Algorithm 4 (inverse): Decodes a bitsliced representation of a vector v ∈ F_{16}^{m} into a vector
#[macro_export]
macro_rules! decode_bit_sliced_array {
    ($bytestring:expr, $OUT_LEN:expr) => {{
        let mut v = [0u8; $OUT_LEN];

        for i in 0..($OUT_LEN / 8) {
            let b0 = $bytestring[i];
            let b1 = $bytestring[$OUT_LEN / 8 + i];
            let b2 = $bytestring[$OUT_LEN / 4 + i];
            let b3 = $bytestring[3 * $OUT_LEN / 8 + i];

            for j in 0..8 {
                // Reconstruct each element from the bits
                let a0 = (b0 >> j) & 0x1; // Least significant bit
                let a1 = (b1 >> j) & 0x1; // Second least significant bit
                let a2 = (b2 >> j) & 0x1; // Third least significant bit
                let a3 = (b3 >> j) & 0x1; // Most significant bit

                // Combine the bits to form an element of GF(16)
                v[i * 8 + j] = (a3 << 3) | (a2 << 2) | (a1 << 1) | a0;
            }
        }
        v
    }};
}



#[macro_export]
macro_rules! decode_bit_sliced_matrices {
    ($bytestring:expr, $rows:expr, $cols:expr, $matrices:expr, $upper_triangular:expr) => {{
        let sub_byte_end = $matrices / 2;
        let mut curr_byte_idx = 0;

        let mut a = [[[0u8; $cols]; $rows]; $matrices]; // Initialize the matrices array

        for i in 0..$rows {
            for j in 0..$cols {
                if i <= j || $upper_triangular == false {
                    let slice_end = curr_byte_idx + sub_byte_end;
                    let encoded_bits = &$bytestring[curr_byte_idx..slice_end];
                    let indices_array = decode_bit_sliced_array!(encoded_bits, $matrices);

                    for (mat_index, &value) in indices_array.iter().enumerate() {
                        a[mat_index][i][j] = value;
                    }
                    curr_byte_idx = slice_end;
                }
            }
        }
        a
    }};
}



// Mayo Algorithm 4: Encodes a vector v ∈ F_{16}^{m} into a bitsliced representation
pub fn encode_bit_sliced_vector(v: Vec<u8>) -> Vec<u8> {
    let m = v.len();
    let mut bytestring = vec![0u8; m / 2]; // Bytestring of length m/2 of all 0s

    for i in 0..(m / 8) {
        let mut b0: u8 = 0x0;
        let mut b1: u8 = 0x0;
        let mut b2: u8 = 0x0;
        let mut b3: u8 = 0x0;

        for j in (0..8).rev() {
            //Encode 8 elements of v into 4 bytes
            let a0 = v[i * 8 + j] & 0x1; // Least significant bit
            let a1 = (v[i * 8 + j] & 0x2) >> 1; // Second least significant bit
            let a2 = (v[i * 8 + j] & 0x4) >> 2; // Third least significant bit
            let a3 = (v[i * 8 + j] & 0x8) >> 3; // Most significant bit (in our GF(16) representation)

            b0 = (b0 << 1) | a0; // b0 = b0 * 2 + a0
            b1 = (b1 << 1) | a1; // b1 = b1 * 2 + a1
            b2 = (b2 << 1) | a2; // b2 = b2 * 2 + a2
            b3 = (b3 << 1) | a3; // b3 = b3 * 2 + a3
        }
        bytestring[i] = b0;
        bytestring[m / 8 + i] = b1;
        bytestring[m / 4 + i] = b2;
        bytestring[3 * m / 8 + i] = b3;
    }
    return bytestring;
}





pub fn encode_bit_sliced_array(v: [u8 ; M]) -> [u8 ; M/2]{
    let mut bytestring = [0u8; M / 2]; // Bytestring of length m/2 of all 0s

    for i in 0..(M / 8) {
        let mut b0: u8 = 0x0;
        let mut b1: u8 = 0x0;
        let mut b2: u8 = 0x0;
        let mut b3: u8 = 0x0;

        for j in (0..8).rev() {
            //Encode 8 elements of v into 4 bytes
            let a0 = v[i * 8 + j] & 0x1; // Least significant bit
            let a1 = (v[i * 8 + j] & 0x2) >> 1; // Second least significant bit
            let a2 = (v[i * 8 + j] & 0x4) >> 2; // Third least significant bit
            let a3 = (v[i * 8 + j] & 0x8) >> 3; // Most significant bit (in our GF(16) representation)

            b0 = (b0 << 1) | a0; // b0 = b0 * 2 + a0
            b1 = (b1 << 1) | a1; // b1 = b1 * 2 + a1
            b2 = (b2 << 1) | a2; // b2 = b2 * 2 + a2
            b3 = (b3 << 1) | a3; // b3 = b3 * 2 + a3
        }
        bytestring[i] = b0;
        bytestring[M / 8 + i] = b1;
        bytestring[M / 4 + i] = b2;
        bytestring[3 * M / 8 + i] = b3;
    }
    return bytestring;
}

pub fn encode_l_bit_sliced_matrices_array(a: [[[u8; O]; V]; M],is_triangular: bool,) -> [u8 ; L_BYTES] {
    let mut bytestring = [0u8 ; L_BYTES];

    // Encode bits from the matrices in the following order:
    // A0[0, 0], A1[0, 0], . . . , Am−1[0, 0]
    // Ai[0, 1] entries up to the Ai[0, c − 1]
    // Ai[r − 1, c − 1]
    let mut byte_index = 0;

    for i in 0..V {
        for j in 0..O {
            if i <= j || is_triangular == false {
                let mut indices_arr = [0u8 ; M];

                let mut idx = 0;
                for mat in &a {
                    // concatenate the bitsliced representation of the triangular matrix
                    indices_arr[idx] = mat[i][j];
                    idx += 1;
                }

                let encoded_bits = encode_bit_sliced_array(indices_arr);

                bytestring[byte_index*M/2..(byte_index+1)*M/2].copy_from_slice(&encoded_bits);
                byte_index += 1;
            }
        }
    }

    return bytestring;
}


pub fn encode_p3_bit_sliced_matrices_array(a: [[[u8; O]; O]; M],is_triangular: bool,) -> [u8 ; P3_BYTES] {
    let mut bytestring = [0u8 ; P3_BYTES];

    // Encode bits from the matrices in the following order:
    // A0[0, 0], A1[0, 0], . . . , Am−1[0, 0]
    // Ai[0, 1] entries up to the Ai[0, c − 1]
    // Ai[r − 1, c − 1]
    let mut byte_index = 0;

    for i in 0..O {
        for j in 0..O {
            if i <= j || is_triangular == false {
                let mut indices_arr = [0u8 ; M];

                let mut idx = 0;
                for mat in &a {
                    // concatenate the bitsliced representation of the triangular matrix
                    indices_arr[idx] = mat[i][j];
                    idx += 1;
                }

                let encoded_bits = encode_bit_sliced_array(indices_arr);

                bytestring[byte_index*M/2..(byte_index+1)*M/2].copy_from_slice(&encoded_bits);
                byte_index += 1;
            }
        }
    }

    return bytestring;
}


