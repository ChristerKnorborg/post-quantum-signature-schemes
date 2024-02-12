use std::vec;





// Function to encode a field element in GF(16) as a nibble. 
// The method only reorders thelast 4 bits of the u8.
// E.g. 0xa (00001010) is encoded as 0x5 (00000101)
pub fn encode_field_element_to_nibble(a: u8) -> u8 {
    
    let mut nibble = 0;
    nibble |= (a & 0x1) << 3; // bit 0 to position 3
    nibble |= (a & 0x2) << 1; // bit 1 to position 2
    nibble |= (a & 0x4) >> 1; // bit 2 to position 1
    nibble |= (a & 0x8) >> 3; // bit 3 to position 0

    return nibble
}


// Function to decode a nibble to a field element in GF(16)
// The method only reorders thelast 4 bits of the u8.
// E.g. 0xa (00001010) is encoded as 0x5 (00000101)
pub fn decode_nibble_to_field_element(nibble: u8) -> u8 {

    let mut field_element = 0;
    field_element |= (nibble & 0x8) >> 3; // bit 3 to position 0
    field_element |= (nibble & 0x4) >> 1; // bit 2 to position 1
    field_element |= (nibble & 0x2) << 1; // bit 1 to position 2
    field_element |= (nibble & 0x1) << 3; // bit 0 to position 3

    return field_element
}



// Function to encode a vector of field elements in GF(16) into a bytestring.
// Two nibbles previously represented in individual bytes are now represented in a single byte.
pub fn encode_vector_to_bytestring(x: Vec<u8>) -> Vec<u8> {

    let mut bytestring = vec![];

    // Iterate over each element in pairs and encode them into a single byte
    for pair in x.chunks(2) {

        let first_nibble = encode_field_element_to_nibble(pair[0]);
        let second_nibble = if pair.len() == 2 {
            encode_field_element_to_nibble(pair[1])
        } else {
            0 // If the length of x is odd, pad the last byte with a zero nibble
        };
        bytestring.push(first_nibble << 4 | second_nibble);
    }
    return bytestring
}


// Function to decode a bytestring back into a vector of field elements in GF(16).
// Two nibbles previously represented in a single byte encoding are now decoded to two individual bytes.
pub fn decode_bytestring_to_vector(n: usize, bytestring: Vec<u8>) -> Vec<u8> {

    let mut x = Vec::with_capacity(n);

    // Calculate the number of full bytes and if there's an extra nibble
    let full_bytes = n / 2;
    let extra_nibble = n % 2;

    // Iterate over all bytes with two nibbles in each
    for &byte in bytestring.iter().take(full_bytes) {
        x.push(decode_nibble_to_field_element(byte >> 4)); // Decode the first (most significant) nibble
        x.push(decode_nibble_to_field_element(byte & 0x0F)); // Decode the second (least significant) nibble
    }

    // Decode an extra nibble if n is odd
    if extra_nibble == 1 {
        let &last_byte = bytestring.last().unwrap(); // Unwrap is safe cause at least one byte if n is odds
        x.push(decode_nibble_to_field_element(last_byte >> 4)); // Decode the first (most significant) nibble
    }

    return x
}





// Function to encode a matrix into a bytestring, 
pub fn encode_matrix_to_bytestring(matrix: Vec<Vec<u8>>) -> Vec<u8> {

    // Flatten the matrix rows into a single vector.
    let v: Vec<u8> = matrix.into_iter().flatten().collect();

    // Encode the resulting vector of the matrix rows into a bytestring.
    return encode_vector_to_bytestring(v)
}



// Function to decode a byte-string back into a matrix.
pub fn decode_bytestring_to_matrix(rows: usize, cols: usize, bytestring: Vec<u8>) -> Vec<Vec<u8>> {

    // Decode the bytestring into a vector.
    let v = decode_bytestring_to_vector(rows * cols, bytestring);

    // Chunk the flat vector back into a matrix.
    v.chunks(cols).map(|chunk| chunk.to_vec()).collect()
}









// Mayo Algorithm 4: Encodes a vector v ∈ F_{16}^{m} into a bitsliced representation
pub fn encode_bit_sliced_vector(v: Vec<u8>) -> Vec<u8>{

    let m = v.len();
    let mut bytestring = vec![0u8; m/2]; // Bytestring of length m/2 of all 0s

    for i in 0..(m/8) {
        
        let mut b0: u8 = 0x0;
        let mut b1: u8 = 0x0;
        let mut b2: u8 = 0x0;
        let mut b3: u8 = 0x0;

        for j in (0..8).rev() {
            //Encode 8 elements of v into 4 bytes
            let a0 = v[i*8 + j] & 0x1; // Least significant bit
            let a1 = (v[i*8 + j] & 0x2) >> 1; // Second least significant bit
            let a2 = (v[i*8 + j] & 0x4) >> 2; // Third least significant bit
            let a3 = (v[i*8 + j] & 0x8) >> 3; // Most significant bit (in our GF(16) representation)

            b0 = (b0 << 1) | a0; // b0 = b0 * 2 + a0
            b1 = (b1 << 1) | a1; // b1 = b1 * 2 + a1
            b2 = (b2 << 1) | a2; // b2 = b2 * 2 + a2
            b3 = (b3 << 1) | a3; // b3 = b3 * 2 + a3
        }
        bytestring[i]         = b0;
        bytestring[m/8 +   i] = b1;
        bytestring[m/4 +   i] = b2;
        bytestring[3*m/8 + i] = b3;
    }
    return bytestring;
}


// Mayo Algorithm 4 (inverse): Decodes a bitsliced representation of a vector v ∈ F_{16}^{m} into a vector
pub fn decode_bit_sliced_vector(bytestring: Vec<u8>) -> Vec<u8> {

    let m = bytestring.len() * 2;
    
    let mut v = vec![0u8; m];

    for i in 0..(m/8) {
        let b0 = bytestring[i];
        let b1 = bytestring[m/8     + i];
        let b2 = bytestring[m/4     + i];
        let b3 = bytestring[3*m/8   + i];

        for j in 0..8 {
            // Reconstruct each element from the bits
            let a0 = (b0 >> (j)) & 0x1; // Least significant bit
            let a1 = (b1 >> (j)) & 0x1; // Second least significant bit
            let a2 = (b2 >> (j)) & 0x1; // Third least significant bit
            let a3 = (b3 >> (j)) & 0x1; // Most significant bit

            // Combine the bits to form an element of GF(16)
            v[i*8+j] = (a3 << 3) | (a2 << 2) | (a1 << 1) | a0;
        }
    }
    return v
}





// MAYO Algorithm 3: Encodes m matrices A_i of ∈ F_{16}^{r x c} into a bitsliced representation
pub fn encode_bit_sliced_matrices(rows: usize, cols: usize, a: Vec<Vec<Vec<u8>>>, is_triangular: bool) -> Vec<u8>{

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
pub fn decode_bit_sliced_matrices(rows: usize, cols: usize, bytestring: Vec<u8>, is_triangular: bool) -> Vec<Vec<Vec<u8>>> {

    let num_matrices = if is_triangular {
        (4 * bytestring.len()) / (rows * (rows + 1)) // If the matrix is triangular: m = 4*len(bytestring)/(rows*(rows+1))
    } else {
        (2 * bytestring.len()) / (rows * cols) // If the matrix is non-triangular: m = 2*len(bytestring)/(rows*cols)
    };

    let mut a = vec![vec![vec![0u8; cols]; rows]; num_matrices]; // Initialize the matrices list of size m x rows x cols 
    let sub_byte_end = num_matrices/2;
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
                    //println!("mat_index: {}", mat_index);
                    a[mat_index][i][j] = value;
                }
                // Update the byte index for the next set of bytes
                curr_byte_idx = slice_end;
            }
        }
    }
    return a
}






#[cfg(test)]
mod tests {
    
    use super::*;
    use std::vec;
    use rand::random;
    use rand::{distributions::Uniform, Rng};



    #[test]
    fn test_encode_and_decode_nibble() {
        // Test the encoding and decoding of a field element
        let original_element = 0x2; // 00000010
        let encoded_nibble = encode_field_element_to_nibble(original_element);
        assert_eq!(encoded_nibble, 0x4); // Should be 00000100

        let decoded_element = decode_nibble_to_field_element(encoded_nibble);
        assert_eq!(decoded_element, original_element); 


        let original_element = 0x8; // 00001000
        let encoded_nibble = encode_field_element_to_nibble(original_element);
        assert_eq!(encoded_nibble, 0x1); // Should be 00000001

        let decoded_element = decode_nibble_to_field_element(encoded_nibble);
        assert_eq!(decoded_element, original_element);
    }


    #[test]
    fn test_encode_decode_vector_to_bytestring() {

        // Encode and decode even length vector
        let original_vector_even = vec![0xa, 0x1, 0x2, 0xb];
        let encoded_bytestring_even = encode_vector_to_bytestring(original_vector_even.clone());
        let decoded_vector_even = decode_bytestring_to_vector(original_vector_even.len(), encoded_bytestring_even);
        assert_eq!(decoded_vector_even, original_vector_even);

        // Encode and decode odd length vector
        let original_vector_odd = vec![0xa, 0x1, 0x2];
        let encoded_bytestring_odd = encode_vector_to_bytestring(original_vector_odd.clone()); // Should add padding
        assert_eq!(encoded_bytestring_odd.len(), 2); // Check that the padding does not add an extra byte
        assert_eq!(encoded_bytestring_odd[1], 64); // Check that the padding is added (should be 0100 concatenated with 0000 - 64 in decimal 0)
        let decoded_vector_odd = decode_bytestring_to_vector(original_vector_odd.len(), encoded_bytestring_odd); // Should removes the padding
        assert_eq!(decoded_vector_odd, original_vector_odd);
    }





    #[test]
    fn test_encode_and_decode_matrix_to_bytestring() {



        let matrix = vec![
            vec![0x1, 0x2, 0x3],
            vec![0x4, 0x5, 0x6],
            vec![0x7, 0x8, 0x9],
        ];
        let rows = matrix.len();
        let cols = matrix[0].len();

        let encoded_bytestring = encode_matrix_to_bytestring(matrix.clone());
        let decoded_matrix = decode_bytestring_to_matrix(rows, cols, encoded_bytestring);

        assert_eq!(decoded_matrix, matrix);
    }




    #[test]
    fn test_encode_and_decode_random_matrices() {
        let mut rng = rand::thread_rng();
        let rows_range = Uniform::from(1..100); // Matrices between 1x1 and 99x99 in size
        let cols_range = Uniform::from(1..100);
        let element_range = Uniform::from(0..16); // Elements will be between 0 and 15 (GF(16))

        for _ in 0..50 {
            let rows = rng.sample(rows_range);
            let cols = rng.sample(cols_range);
            let matrix: Vec<Vec<u8>> = (0..rows)
                .map(|_| (0..cols).map(|_| rng.sample(element_range)).collect())
                .collect();

            let encoded_bytestring = encode_matrix_to_bytestring(matrix.clone());
            let decoded_matrix = decode_bytestring_to_matrix(rows, cols, encoded_bytestring);

            assert_eq!(decoded_matrix, matrix, "Failed on matrix: {:?}", matrix);
        }
    }







    #[test]
    fn test_encode_vector_simple() {
       let test_vec: Vec<u8> = vec![0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8]; 

       let result = encode_bit_sliced_vector(test_vec);
       let expected: Vec <u8> = vec![85, 102, 120, 128];

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
        let expected: Vec <u8> = vec![0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8];

        assert_eq!(
            result, expected,
            "Decode form did not match expected result"
        );
    }



    #[test]
    fn test_multiple_encode_vector_and_decode(){
            
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



    #[test]
    fn test_encode_and_decode_matrices_non_triangular() {
        let vec_1: Vec<Vec<u8>> = vec![
            vec![0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x4], 
            vec![0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xf], 
            vec![0x8, 0x9, 0xf, 0xe, 0x3, 0x4, 0x5, 0x6],
            vec![0x8, 0x9, 0x5, 0x6, 0xe, 0xa, 0xb, 0x1],  
        ];
        
        let vec_2 = vec_1.clone();
        let vec_3 = vec_1.clone();
        let vec_4 = vec_1.clone();
        let vec_5 = vec_1.clone();
        let vec_6 = vec_1.clone();
        let vec_7 = vec_1.clone();
        let vec_8 = vec_1.clone();

        let rows = vec_1.len();
        let cols = vec_1[0].len();

        let plain_input: Vec<Vec<Vec<u8>>> = vec![vec_1.clone(), vec_2, vec_3, vec_4, vec_5, vec_6, vec_7, vec_8]; 

        let bytestring
         = encode_bit_sliced_matrices(rows, cols, plain_input.clone(), false);

        let result = decode_bit_sliced_matrices(rows, cols, bytestring, false);

        assert_eq!(
            result, plain_input,
            "Decode form did not match expected result"
        );
    }

    #[test]
    fn test_encode_and_decode_random_matrices_non_triangular() {
        use rand::random;
    
        for _ in 0..10 { // Run the test for 10 different matrix sizes

            let random_value: usize = rand::random();
            let max_multiple_of_8 = 200 / 8;
            let m = (random_value % max_multiple_of_8 + 1) * 8; // Random multiple of 8 between 8 and 200
            let rows = random::<usize>() % 250 + 1; // rows between 1 and 250
            let cols = random::<usize>() % 250 + 1; // cols between 1 and 250
    
            let plain_input: Vec<Vec<Vec<u8>>> = (0..m)
                .map(|_| (0..rows)
                    .map(|_| (0..cols)
                        .map(|_| random::<u8>() % 16) // Elements in GF(16)
                        .collect())
                    .collect())
                .collect();
    
            let bytestring = encode_bit_sliced_matrices(rows, cols, plain_input.clone(), false);
            let result = decode_bit_sliced_matrices(rows, cols, bytestring, false);
    
            assert_eq!(result, plain_input, "Decode form did not match expected result for non-triangular matrices");
        }
    }


    #[test]
    fn test_encode_and_decode_matrices_triangular() {
        let vec_1: Vec<Vec<u8>> = vec![
            vec![0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x4], 
            vec![0x0, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xf], 
            vec![0x0, 0x0, 0x4, 0x0, 0x3, 0x4, 0x5, 0x6],
            vec![0x0, 0x0, 0x0, 0x2, 0x1, 0x2, 0x4, 0x1],  
            vec![0x0, 0x0, 0x0, 0x0, 0x2, 0x2, 0x2, 0x4], 
            vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x8, 0x9, 0xf], 
            vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x5, 0x6],
            vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1],  
        ];
        
        let vec_2 = vec_1.clone();
        let vec_3 = vec_1.clone();
        let vec_4 = vec_1.clone();
        let vec_5 = vec_1.clone();
        let vec_6 = vec_1.clone();
        let vec_7 = vec_1.clone();
        let vec_8 = vec_1.clone();

        let rows = vec_1.len();
        let cols = vec_1[0].len();

        let plain_input: Vec<Vec<Vec<u8>>> = vec![vec_1.clone(), vec_2, vec_3, vec_4, vec_5, vec_6, vec_7, vec_8]; 

        let bytestring
         = encode_bit_sliced_matrices(rows, cols, plain_input.clone(), true);

        let result = decode_bit_sliced_matrices(rows, cols, bytestring, true);

        assert_eq!(
            result, plain_input,
            "Decode form did not match expected result"
        );
    }


    #[test]
    fn test_encode_and_decode_random_matrices_triangular() {
        use rand::random;

        for _ in 0..10 { // Run the test for 10 different matrix sizes randomly chosen

            let random_value: usize = rand::random();
            let max_multiple_of_8 = 200 / 8;
            let m = (random_value % max_multiple_of_8 + 1) * 8; // Random multiple of 8 between 8 and 200
            let size = random::<usize>() % 250 + 1; // Square matrix size between 1 and 250

            let plain_input: Vec<Vec<Vec<u8>>> = (0..m)
                .map(|_| (0..size)
                    .map(|i| (0..size)
                        .map(|j| if i <= j { random::<u8>() % 16 } else { 0 }) // Upper triangular condition
                        .collect())
                    .collect())
                .collect();

            let bytestring = encode_bit_sliced_matrices(size, size, plain_input.clone(), true);
            let result = decode_bit_sliced_matrices(size, size, bytestring, true);

            assert_eq!(result, plain_input, "Decode form did not match expected result for triangular matrices");
        }
    }





}
