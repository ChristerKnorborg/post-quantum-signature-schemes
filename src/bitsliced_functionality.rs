use std::vec;




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

    let mut bytestring: Vec<u8> = Vec::new(); // Bytestring of length 256 of all 0s

    for i in 0..rows {
        for j in 0..cols {
            if i <= j || is_triangular == false {

                let mut indices_vec: Vec<u8> = Vec::new();
                
                for mat in &a {
                    // concatenate the bitsliced representation of the triangular matrix
                    indices_vec.push(mat[i][j]);
                }

                let encoded_bits = encode_bit_sliced_vector(indices_vec);
                println!("Encoded bits: {:?}", encoded_bits);
                bytestring.extend(encoded_bits);
            }
        }
    }

    return bytestring;
} 











// MAYO Algorithm 3 (inverse): Decodes a bitsliced representation of m matrices denoted a.
pub fn decode_bit_sliced_matrices(rows: usize, cols: usize, bytestring: Vec<u8>, is_triangular: bool) -> Vec<Vec<Vec<u8>>> {


    let elements_per_matrix = if is_triangular {
        rows * (rows + 1) / 2
    } else {
        rows * cols
    };

    // Calculate the number of matrices m:
    // If the matrix is non-triangular: bytestring len is (m*r*c)/2
    // Else: bytestring len is (m*r*(r+1))/2

    // rewrite formula: m = (2*B)/(rc)
    let num_matrices = 2 * bytestring.len() / (elements_per_matrix);

    let mut a = vec![vec![vec![0u8; cols]; rows]; num_matrices];
    let mut curr_byte_idx = 0;


    println!("Elements per matrix {}", elements_per_matrix);
    println!("Bytestring len {}", bytestring.len());

    println!("num_matrices: {}", num_matrices);
  
    println!("rows: {}", rows);
    println!("cols: {}", cols);
    for i in 0..rows {
        for j in 0..cols {
            if i <= j || is_triangular == false {

                // Slice the bytestring to get the exact bytes for decodin
                // Remember bytestring is m/2 aka num_matrices/2
                let sub_byte_end = num_matrices/2;
                let slice_end = curr_byte_idx + sub_byte_end;
                let encoded_bits = &bytestring[curr_byte_idx..slice_end];

                println!("Encoded bits length {}", encoded_bits.len());
                println!("Encoded bits: {:?}", encoded_bits);
                // Decode the bitsliced vector back into indices
                let indices_vec = decode_bit_sliced_vector(encoded_bits.to_vec());
                
                println!("Indices_vec length {}", indices_vec.len());

                println!("Indices_vec: {:?}", indices_vec.clone());
        

                // Distribute the decoded elements back into the matrices
                for (mat_index, &value) in indices_vec.iter().enumerate() {
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
    fn test_encode_and_decode_matrices() {
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

}
