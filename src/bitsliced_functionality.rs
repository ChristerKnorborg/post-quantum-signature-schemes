use std::vec;



// Function to encode a vector of field elements in GF(16) into a bytestring.
// Two nibbles previously represented in individual bytes are now represented in a single byte.
pub fn encode_vector_to_bytestring(x: Vec<u8>) -> Vec<u8> {
    let mut bytestring = vec![];

    // Iterate over each element in pairs and encode them into a single byte
    for pair in x.chunks(2) {
        let first_nibble = pair[0];
        let second_nibble = if pair.len() == 2 {
            pair[1]
        } else {
            0 // If the length of x is odd, pad the last byte with a zero nibble
        };
        // Combine the two nibbles into a single byte (second_nibble is the 4 most significant bits and first_nibble is the 4 least significant bits)
        bytestring.push(second_nibble << 4 | first_nibble);
    }
    return bytestring;
}




// Function to decode a bytestring back into a vector of field elements in GF(16).
// Two nibbles previously represented in a single byte encoding are now decoded to two individual bytes.
pub fn decode_bytestring_to_vector(n: usize, bytestring: Vec<u8>) -> Vec<u8> {
    // Calculate the number of full bytes and if there's an extra nibble
    let full_bytes = n / 2;
    let extra_nibble = n % 2;
    let mut x = Vec::with_capacity(n + extra_nibble);

    // Iterate over all bytes with two nibbles in each
    for &byte in bytestring.iter().take(full_bytes) {
        x.push(byte & 0x0F); // Put the first nibble (4 least significant bits) into the first byte
        x.push(byte >> 4); // Put the second nibble (4 most significant bits) into the second byte (4 most significant bits)
    }

    // Decode an extra nibble if n is odd
    if extra_nibble == 1 {
        let &last_byte = bytestring.get(n / 2).unwrap(); // Unwrap is safe cause at least one byte if n is odds

        x.push(last_byte & 0x0F); // Put the first nibble (4 least significant bits) into the last byte in the byte vector (ignore the second nibble of 0)
    }

    return x;
}



// Function to encode a matrix into a bytestring,
pub fn encode_matrix_to_bytestring(matrix: Vec<Vec<u8>>) -> Vec<u8> {
    // Flatten the matrix rows into a single vector.
    let v: Vec<u8> = matrix.into_iter().flatten().collect();

    // Encode the resulting vector of the matrix rows into a bytestring.
    return encode_vector_to_bytestring(v);
}



// Function to decode a byte-string back into a matrix.
pub fn decode_bytestring_to_matrix(rows: usize, cols: usize, bytestring: Vec<u8>) -> Vec<Vec<u8>> {
    // Decode the bytestring into a vector.
    let v = decode_bytestring_to_vector(rows * cols, bytestring);

    // Chunk the flat vector back into a matrix.
    v.chunks(cols).map(|chunk| chunk.to_vec()).collect()
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





// Mayo Algorithm 4 (inverse): Decodes a bitsliced representation of a vector v ∈ F_{16}^{m} into a vector
pub fn decode_bit_sliced_vector(bytestring: Vec<u8>) -> Vec<u8> {
    let m = bytestring.len() * 2;

    let mut v = vec![0u8; m];

    for i in 0..(m / 8) {
        let b0 = bytestring[i];
        let b1 = bytestring[m / 8 + i];
        let b2 = bytestring[m / 4 + i];
        let b3 = bytestring[3 * m / 8 + i];

        for j in 0..8 {
            // Reconstruct each element from the bits
            let a0 = (b0 >> (j)) & 0x1; // Least significant bit
            let a1 = (b1 >> (j)) & 0x1; // Second least significant bit
            let a2 = (b2 >> (j)) & 0x1; // Third least significant bit
            let a3 = (b3 >> (j)) & 0x1; // Most significant bit

            // Combine the bits to form an element of GF(16)
            v[i * 8 + j] = (a3 << 3) | (a2 << 2) | (a1 << 1) | a0;
        }
    }
    return v;
}





// MAYO Algorithm 3: Encodes m matrices A_i of ∈ F_{16}^{r x c} into a bitsliced representation
pub fn encode_bit_sliced_matrices(
    rows: usize,
    cols: usize,
    a: Vec<Vec<Vec<u8>>>,
    is_triangular: bool,
) -> Vec<u8> {
    let m = a.len();
    let mut bytestring: Vec<u8> = Vec::new();

    // Encode bits from the matrices in the following order:
    // A0[0, 0], A1[0, 0], . . . , Am−1[0, 0]
    // Ai[0, 1] entries up to the Ai[0, c − 1]
    // Ai[r − 1, c − 1]
    for i in 0..rows {
        for j in 0..cols {
            if i <= j || is_triangular == false {
                let mut indices_vec: Vec<u8> = Vec::with_capacity(m);

                for mat in &a {
                    // concatenate the bitsliced representation of the triangular matrix
                    indices_vec.push(mat[i][j]);
                }

                let mut encoded_bits = encode_bit_sliced_vector(indices_vec);

                bytestring.append(&mut encoded_bits);
            }
        }
    }

    return bytestring;
}







// MAYO Algorithm 3 (inverse): Decodes a bitsliced representation of m matrices denoted a.
pub fn decode_bit_sliced_matrices(
    rows: usize,
    cols: usize,
    bytestring: Vec<u8>,
    is_triangular: bool,
) -> Vec<Vec<Vec<u8>>> {
    let num_matrices = if is_triangular {
        (4 * bytestring.len()) / (rows * (rows + 1)) // If the matrix is triangular: m = 4*len(bytestring)/(rows*(rows+1))
    } else {
        (2 * bytestring.len()) / (rows * cols) // If the matrix is non-triangular: m = 2*len(bytestring)/(rows*cols)
    };

    let mut a = vec![vec![vec![0u8; cols]; rows]; num_matrices]; // Initialize the matrices list of size m x rows x cols
    let sub_byte_end = num_matrices / 2;
    let mut curr_byte_idx = 0;

    for i in 0..rows {
        for j in 0..cols {
            if i <= j || is_triangular == false {
                // Slice the bytestring (of size num_matrices/2) to get the exact bytes for decoding
                let slice_end = curr_byte_idx + sub_byte_end;
                let encoded_bits = &bytestring[curr_byte_idx..slice_end];

                // Decode the bitsliced vector back into indices
                let indices_vec = decode_bit_sliced_vector(encoded_bits.to_vec());

                // Distribute the decoded elements back into the matrices
                for (mat_index, &value) in indices_vec.iter().enumerate() {
                    a[mat_index][i][j] = value;
                }
                // Update the byte index for the next set of bytes
                curr_byte_idx = slice_end;
            }
        }
    }
    return a;
}
