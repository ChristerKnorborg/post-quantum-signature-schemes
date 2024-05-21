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
macro_rules! decode_bytestring_to_matrix {
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





// Mayo Algorithm 4: Encodes an array v ∈ F_{16}^{m} into a bitsliced representation
#[macro_export]
macro_rules! encode_bit_sliced_array {
    ($v:expr, $OUT_LEN:expr) => {{
        let mut bytestring = [0u8; $OUT_LEN / 2]; // Bytestring of length M/2 of all 0s

        for i in 0..($OUT_LEN / 8) {
            let mut b0: u8 = 0x0;
            let mut b1: u8 = 0x0;
            let mut b2: u8 = 0x0;
            let mut b3: u8 = 0x0;

            for j in (0..8).rev() {
                // Encode 8 elements of v into 4 bytes
                let a0 = $v[i * 8 + j] & 0x1; // Least significant bit
                let a1 = ($v[i * 8 + j] & 0x2) >> 1; // Second least significant bit
                let a2 = ($v[i * 8 + j] & 0x4) >> 2; // Third least significant bit
                let a3 = ($v[i * 8 + j] & 0x8) >> 3; // Most significant bit (in our GF(16) representation)

                b0 = (b0 << 1) | a0; // b0 = b0 * 2 + a0
                b1 = (b1 << 1) | a1; // b1 = b1 * 2 + a1
                b2 = (b2 << 1) | a2; // b2 = b2 * 2 + a2
                b3 = (b3 << 1) | a3; // b3 = b3 * 2 + a3
            }
            bytestring[i] = b0;
            bytestring[$OUT_LEN / 8 + i] = b1;
            bytestring[$OUT_LEN / 4 + i] = b2;
            bytestring[3 * $OUT_LEN / 8 + i] = b3;
        }
        bytestring
    }};
}





// Mayo Algorithm 4 (inverse): Decodes a bitsliced representation of a array v ∈ F_{16}^{m} into an array
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
macro_rules! encode_bit_sliced_matrices {
    ($a:expr, $rows:expr, $cols:expr, $M:expr, $is_triangular:expr, $OUT_BYTES:expr) => {{
        let mut bytestring = [0u8; $OUT_BYTES];
        
        // Initialize variables for indexing and iteration
        let mut byte_index = 0;

        for i in 0..$rows {
            for j in 0..$cols {
                if i <= j || !$is_triangular {
                    let mut indices_arr = [0u8; $M];
                    
                    // Populate indices_arr with elements from the matrices
                    for mat_index in 0..$M {
                        // Calculate the 1D index for accessing the matrix element
                        let index = mat_index * $rows * $cols + i * $cols + j;
                        indices_arr[mat_index] = $a[index];
                    }

                    // Use the provided encode function/macro on indices_arr
                    let encoded_bits: [u8; $M / 2] = encode_bit_sliced_array!(indices_arr, $M);

                    // Copy the encoded bits into the bytestring
                    let slice_start = byte_index * $M / 2;
                    let slice_end = (byte_index + 1) * $M / 2;
                    bytestring[slice_start..slice_end].copy_from_slice(&encoded_bits);
                    byte_index += 1;
                }
            }
        }
        bytestring
    }};
}




#[macro_export]
macro_rules! decode_and_concatenate_matrices {
    ($p1_bytestring:expr, $p2_bytestring:expr, $p3_bytestring:expr, $V:expr, $O:expr, $M:expr) => {{
        let mut result = [0u8; $M * N * N];

        let sub_byte_end = $M / 2;

        let mut p1_curr_byte_idx = 0;
        let mut p2_curr_byte_idx = 0;
        let mut p3_curr_byte_idx = 0;

        // One row of P1 and P2 at a time
        for i in 0..$V {
            // Decode P1 (Upper Triangular) into the top-left corner
            for j in i..$V {
                let slice_end = p1_curr_byte_idx + sub_byte_end;
                let encoded_bits = &$p1_bytestring[p1_curr_byte_idx..slice_end];
                let indices_array = decode_bit_sliced_array!(encoded_bits, $M);

                for (mat_index, &value) in indices_array.iter().enumerate() {
                    let index = mat_index * (N * N) + i * N + j;
                    result[index] = value;
                }
                p1_curr_byte_idx = slice_end;
            }

            // Decode P2 into the top-right corner
            for j in 0..$O {
                let slice_end = p2_curr_byte_idx + sub_byte_end;
                let encoded_bits = &$p2_bytestring[p2_curr_byte_idx..slice_end];
                let indices_array = decode_bit_sliced_array!(encoded_bits, $M);

                for (mat_index, &value) in indices_array.iter().enumerate() {
                    let index = mat_index * (N * N) + i * N + ($V + j);
                    result[index] = value;
                }
                p2_curr_byte_idx = slice_end;
            }
        }

        // Decode P3 (upper triangular) into the bottom-right corner
        for i in 0..$O {
            for j in i..$O {
                let slice_end = p3_curr_byte_idx + sub_byte_end;
                let encoded_bits = &$p3_bytestring[p3_curr_byte_idx..slice_end];
                let indices_array = decode_bit_sliced_array!(encoded_bits, $M);

                for (mat_index, &value) in indices_array.iter().enumerate() {
                    let index = mat_index * (N * N) + ($V + i) * N + ($V + j);
                    result[index] = value;
                }
                p3_curr_byte_idx = slice_end;
            }
        }
        result
    }};
}








// Mayo Algorithm 3 (inverse): Decodes a bitsliced representation of a vector v ∈ F_{16}^{m} into a vector
#[macro_export]
macro_rules! decode_bit_sliced_matrices {
    ($bytestring:expr, $rows:expr, $cols:expr, $matrices:expr, $upper_triangular:expr) => {{
        let sub_byte_end = $matrices / 2;
        let mut curr_byte_idx = 0;

        // Create a single 1D array to hold all elements
        let mut a = [0u8; $matrices * $rows * $cols]; // Initialize the matrices array

        for i in 0..$rows {
            for j in 0..$cols {
                if i <= j || $upper_triangular == false {
                    let slice_end = curr_byte_idx + sub_byte_end;
                    let encoded_bits = &$bytestring[curr_byte_idx..slice_end];
                    let indices_array = decode_bit_sliced_array!(encoded_bits, $matrices);

                    for (mat_index, &value) in indices_array.iter().enumerate() {
                        let index = mat_index * $rows * $cols + i * $cols + j;
                        a[index] = value;
                    }
                    curr_byte_idx = slice_end;
                }
            }
        }
        a
    }};
}




#[macro_export]
macro_rules! decode_bit_sliced_matrices_double_array {
    ($bytestring:expr, $rows:expr, $cols:expr, $matrices:expr, $upper_triangular:expr) => {{
        let sub_byte_end = $matrices / 2;
        let mut curr_byte_idx = 0;

        // Create a 2D array representation for each matrix
        let mut a = [[0u8; $rows * $cols]; $matrices]; // Initialize the array of matrices

        for i in 0..$rows {
            for j in 0..$cols {
                if i <= j || !$upper_triangular {
                    let slice_end = curr_byte_idx + sub_byte_end;
                    let encoded_bits = &$bytestring[curr_byte_idx..slice_end];
                    let indices_array = decode_bit_sliced_array!(encoded_bits, $matrices);

                    for (mat_index, &value) in indices_array.iter().enumerate() {
                        let index = i * $cols + j; // Index for the 2D slice of the matrix
                        a[mat_index][index] = value;
                    }
                    curr_byte_idx = slice_end;
                }
            }
        }
        a
    }};
}

