use std::vec;

use crate::{
    constants::{DIGEST_BYTES, K, L_BYTES, M, N, O, O_BYTES, P3_BYTES, R_BYTES, SALT_BYTES, SIG_BYTES, V, V_BYTES},
    utils::bytes_to_hex_string,
};

 
pub fn encode_signature_to_bytestring(x: [u8 ; K*N]) -> [u8 ; SIG_BYTES-SALT_BYTES] {
    let mut bytestring = [0u8 ; SIG_BYTES-SALT_BYTES];

    // Iterate over each element in pairs and encode them into a single byte
    let mut byteindex = 0;
    for pair in x.chunks(2) {
        let first_nibble = pair[0];
        let second_nibble = if pair.len() == 2 {
            pair[1]
        } else {
            0 // If the length of x is odd, pad the last byte with a zero nibble
        };
        // Combine the two nibbles into a single byte (second_nibble is the 4 most significant bits and first_nibble is the 4 least significant bits)
        bytestring[byteindex] = second_nibble << 4 | first_nibble;
        byteindex +=1;
    }

    return bytestring;
}





// Function to decode a byte-string back into a matrix.
pub fn decode_o_bytestring_to_matrix_array(bytestring: &[u8]) -> [[u8; O] ; V ] {

    // Decode the bytestring into a vector.
    let v = decode_bytestring_to_vector_array( bytestring);

    // Chunk the flat array back into a matrix.
    let mut result: [[u8; O]; N - O] = [[0; O]; N - O]; // Array of arrays

    for (i, chunk) in v.chunks(O).take(N - O).enumerate() {
        result[i].copy_from_slice(chunk);
    }

    return result;
}

// Function to decode a bytestring back into a vector of field elements in GF(16).
// Two nibbles previously represented in a single byte encoding are now decoded to two individual bytes.
pub fn decode_bytestring_to_vector_array(bytestring: &[u8]) -> [u8; (V)*O] {
    // Calculate the number of full bytes and if there's an extra nibble
    let mut x = [0u8 ; (V)*O];

    let mut idx = 0;
    // Iterate over all bytes with two nibbles in each
    for &byte in bytestring.iter().take(O_BYTES) {
        x[idx] = byte & 0x0F; // Put the first nibble (4 least significant bits) into the first byte
        idx += 1;
        x[idx] = byte >> 4; // Put the second nibble (4 most significant bits) into the second byte (4 most significant bits)
        idx += 1;
    }

    return x;
}

pub fn decode_t_bytestring_to_array(bytestring: &[u8]) -> [u8; M] {
    // Calculate the number of full bytes and if there's an extra nibble
    let mut x = [0u8 ; M];

    let mut idx = 0;
    // Iterate over all bytes with two nibbles in each
    for &byte in bytestring.iter().take(DIGEST_BYTES) {
        x[idx] = byte & 0x0F; // Put the first nibble (4 least significant bits) into the first byte
        idx += 1;
        x[idx] = byte >> 4; // Put the second nibble (4 most significant bits) into the second byte (4 most significant bits)
        idx += 1;
    }

    return x;
}

pub fn decode_v_bytestring_to_array(bytestring: &[u8]) -> [u8; V] {
    // Calculate the number of full bytes and if there's an extra nibble
    let mut x = [0u8 ; V];

    let extra_nibble = V % 2;
    let full_bytes = V_BYTES - extra_nibble;

    let mut idx = 0;
    // Iterate over all bytes with two nibbles in each
    for &byte in bytestring.iter().take(full_bytes) {
        x[idx] = byte & 0x0F; // Put the first nibble (4 least significant bits) into the first byte
        idx += 1;
        x[idx] = byte >> 4; // Put the second nibble (4 most significant bits) into the second byte (4 most significant bits)
        idx += 1;
    }

    if extra_nibble == 1 {
        // Put the first nibble (4 least significant bits) into the last byte in the byte vector (ignore the second nibble of 0)
        x[idx] = bytestring.get(V/2).unwrap() & 0x0F;
    }
    
    return x;
}

pub fn decode_r_bytestring_to_array(bytestring: &[u8]) -> [u8; K*O] {
    // Calculate the number of full bytes and if there's an extra nibble
    let mut x = [0u8 ; K*O];

    let mut idx = 0;
    // Iterate over all bytes with two nibbles in each
    for &byte in bytestring.iter().take(K*O/2) {
        x[idx] = byte & 0x0F; // Put the first nibble (4 least significant bits) into the first byte
        idx += 1;
        x[idx] = byte >> 4; // Put the second nibble (4 most significant bits) into the second byte (4 most significant bits)
        idx += 1;
    }

    return x;
}

pub fn decode_signature_bytestring_to_array(bytestring: &[u8]) -> [u8; K*N] {
    // Calculate the number of full bytes and if there's an extra nibble
    let mut x = [0u8 ; K*N];
    let extra_nibble = K*N % 2;
    let full_bytes = (SIG_BYTES-SALT_BYTES) - extra_nibble;



    let mut idx = 0;
    // Iterate over all bytes with two nibbles in each
    for &byte in bytestring.iter().take(full_bytes) {
        x[idx] = byte & 0x0F; // Put the first nibble (4 least significant bits) into the first byte
        idx += 1;
        x[idx] = byte >> 4; // Put the second nibble (4 most significant bits) into the second byte (4 most significant bits)
        idx += 1;
    }

    if extra_nibble == 1 {
        // Put the first nibble (4 least significant bits) into the last byte in the byte vector (ignore the second nibble of 0)
        // minus 1 is a mystery.
        x[idx] = bytestring.get((SIG_BYTES-SALT_BYTES-1)).unwrap() & 0x0F;
    }

    return x;
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



pub fn decode_p1_bit_sliced_matrices_array(bytestring: &[u8]) -> [[[u8; V]; V]; M]  {

    let sub_byte_end = M / 2;
    let mut curr_byte_idx = 0;

    let mut a = [[[0u8; V]; V]; M]; // Initialize the matrices array

    for i in 0.. V {
        for j in 0.. V {
            if i <= j  {
                let slice_end = curr_byte_idx + sub_byte_end;
                let encoded_bits = &bytestring[curr_byte_idx..slice_end];
                let indices_vec = decode_bit_sliced_vector(encoded_bits.to_vec());

                for (mat_index, &value) in indices_vec.iter().enumerate() {
                    a[mat_index][i][j] = value;
                }
                curr_byte_idx = slice_end;
            }
        }
    }
    a
}


// Remember p2 is not triangular
pub fn decode_p2_bit_sliced_matrices_array(bytestring: &[u8]) -> [[[u8; O]; V]; M]  {
    let sub_byte_end = M / 2;
    let mut curr_byte_idx = 0;

    let mut a = [[[0u8; O]; V]; M]; // Initialize the matrices array

    for i in 0..V {
        for j in 0..O {
            let slice_end = curr_byte_idx + sub_byte_end;
            let encoded_bits = &bytestring[curr_byte_idx..slice_end];
            let indices_vec = decode_bit_sliced_vector(encoded_bits.to_vec());

            for (mat_index, &value) in indices_vec.iter().enumerate() {
                a[mat_index][i][j] = value;
            }
            curr_byte_idx = slice_end;
        }
    }
    a
}

pub fn decode_p3_bit_sliced_matrices_array(bytestring: &[u8]) -> [[[u8; O]; O]; M]  {

    let sub_byte_end = M / 2;
    let mut curr_byte_idx = 0;

    let mut a = [[[0u8; O]; O]; M]; // Initialize the matrices array

    for i in 0.. O {
        for j in 0.. O {
            if i <= j { 
                let slice_end = curr_byte_idx + sub_byte_end;
                let encoded_bits = &bytestring[curr_byte_idx..slice_end];
                let indices_vec = decode_bit_sliced_vector(encoded_bits.to_vec());

                for (mat_index, &value) in indices_vec.iter().enumerate() {
                    a[mat_index][i][j] = value;
                }
                curr_byte_idx = slice_end;
            }
        }
    }
    a
}





#[cfg(test)]
mod tests {

    use super::*;
    use rand::random;
    use rand::{distributions::Uniform, Rng};
    use std::vec;

    #[test]
    fn test_encode_vector_simple() {
        let test_vec: Vec<u8> = vec![0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8];

        let result = encode_bit_sliced_vector(test_vec);
        let expected: Vec<u8> = vec![85, 102, 120, 128];

        assert_eq!(
            result, expected,
            "Encode form did not match expected result"
        );
    }

    #[test]
    fn test_encode_vector_then_decode() {
        let test_vec: Vec<u8> = vec![0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8];

        let encoding = encode_bit_sliced_vector(test_vec);
        let result = decode_bit_sliced_vector(encoding);
        let expected: Vec<u8> = vec![0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8];

        assert_eq!(
            result, expected,
            "Decode form did not match expected result"
        );
    }

    #[test]
    fn test_multiple_encode_vector_and_decode() {
        // Test that 1000 random vectors give the same result after encoding and decoding
        for _ in 0..1000 {
            let plain_input: Vec<u8> = (0..8).map(|_| random::<u8>() % 15).collect(); // Random vector of length 8 with elements in GF(16)
            let encoding = encode_bit_sliced_vector(plain_input.clone());
            let result = decode_bit_sliced_vector(encoding);

            assert_eq!(
                result, plain_input,
                "Decode form did not match expected result"
            );
        }
    }

    
}
