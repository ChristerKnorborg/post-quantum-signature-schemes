
use sha3::{Shake256, Digest};
use sha3::digest::{Update, ExtendableOutput, XofReader};
use rand::{Rng, rngs::OsRng, RngCore, SeedableRng};
use aes_prng::AesRng;

use crate::constants::{L_BYTES, M, N, O, O_BYTES, P1_BYTES, P2_BYTES, P3_BYTES, PK_SEED_BYTES, SALT_BYTES};
use crate::{bitsliced_functionality as bf, finite_field as ff};







// Function to hash a bytestring with SHAKE256 to a specified output length
pub fn shake256(bytestring: &Vec<u8>, output_length: usize) -> Vec<u8> {

    let mut hasher = Shake256::default();

    hasher.update(&bytestring);

    let mut output = vec![0; output_length]; // Allocate space for the output
    let mut reader = hasher.finalize_xof(); // Get the reader for the output
    reader.read(&mut output); // Read the output into the allocated space

    return output;
}





pub fn aes_128_ctr_seed_expansion(pk_seed: [u8; 16], output_length: usize) -> Vec<u8> {

    let mut rng = AesRng::from_seed(pk_seed);
    
    // sample random bytes
    let mut expanded_seed = vec![0u8; output_length];
    rng.fill_bytes(&mut expanded_seed);

    return expanded_seed
}



pub fn upper(matrix: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let n = matrix.len(); // Assuming it's a square matrix

    let mut upper_matrix = vec![vec![0; n]; n]; // Initialize the upper triangular matrix with zeros

    for i in 0..n {
        for j in i..n {
            upper_matrix[i][j] = matrix[i][j]; // Copy the upper triangular part
        }
    }

    return upper_matrix
}







// MAYO algorithm 5: 
pub fn compact_key_gen() -> (Vec<u8>, Vec<u8>){


    // Pick random seed (same length as salt_bytes)
    let mut sk_seed: Vec<u8> = vec![0u8; SALT_BYTES];
    OsRng.fill(&mut sk_seed[..]); // Fill cryptographically secure with random bytes


    // Derive pk_seed and Oil space from sk_seed
    let output_len = PK_SEED_BYTES + O_BYTES;
    let s = shake256(&sk_seed, output_len);

    
    let pk_seed_slice = &s[0..PK_SEED_BYTES];
    let pk_seed: [u8; PK_SEED_BYTES] = pk_seed_slice.try_into()
    .expect("Slice has incorrect length");


    let n_minus_o = N - O;
  

    // Make Oil space from o_bytes. Only a single is yielded from decode_bit_sliced_matrices in this case 
    let o_bytes = s[PK_SEED_BYTES..].to_vec();

    let o = bf::decode_bytestring_to_matrix(n_minus_o, O, o_bytes); 


    // Derive P_{i}^(1) and P_{i}^(2) from pk_seed
    let p_bytes = aes_128_ctr_seed_expansion(pk_seed, P1_BYTES + P2_BYTES);
    
    let p1_bytes = p_bytes[0..P1_BYTES].to_vec();
    let p2_bytes = p_bytes[P1_BYTES..].to_vec();


    // m p1 matrices are of size (n−o) × (n−o)
    let p1 = bf::decode_bit_sliced_matrices(n_minus_o, n_minus_o, p1_bytes, true);

    // m p2 matrices are of size (n−o) × o (not upper triangular matrices)
    let p2 = bf::decode_bit_sliced_matrices(n_minus_o, O, p2_bytes, false);


    // Allocate space for P_{i}^(3). Size is o × o
    let mut p3 = vec![vec![vec![0u8; O]; O]; M];


    // Compute P_{i}^(3) as (−O^{T} * P^{(1)}_i * O ) − (−O^{T} * P^{(2)}_i )
    // Notice, negation is omimtted as GF(16) negation of an element is the same as the element itself.
    for i in 0..M {

        // transpose 
        let transposed_o = transpose(&o);

        let p1_i = &p1[i];
        let p2_i = &p2[i];

        // Compute: −O^{T} * P^{(1)}_i * O 
        let mut left_term = ff::matrix_mul(&transposed_o, &p1_i);
        left_term = ff::matrix_mul(&left_term, &o);

        // Compute: −O^{T} * P^{(2)}_i 
        let right_term: Vec<Vec<u8>> = ff::matrix_mul(&transposed_o, &p2_i);
        
        // Compute: (−O^{T} * P^{(1)}_i * O ) − (−O^{T} * P^{(2)}_i )
        let sub = ff::matrix_sub(&left_term, &right_term);

        p3[i] = upper(&sub); // Upper triangular part of the result
    }

    let mut encoded_p3 = bf::encode_bit_sliced_matrices(n_minus_o, O, p3, true);

    // Public and secret keys
    let mut cpk = Vec::with_capacity(PK_SEED_BYTES + P3_BYTES); // contains pk_seed and encoded_p3
    let csk = sk_seed;

    cpk.extend_from_slice(&pk_seed); // pk_seed is an array, so we need to use extend_from_slice
    cpk.append(&mut encoded_p3);
    
    return (cpk, csk);
}



// Helper function to transpose a matrix (as described in the MAYO paper)
pub fn transpose(matrix: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    // Create a new transposed matrix with the dimensions swapped
    let mut transposed = vec![vec![0u8; rows]; cols]; 

    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = matrix[i][j]; // Swap elements
        }
    }
    return transposed
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shake256() {
        let input = vec![ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05 ];
        let output_length = 32;
        let result = shake256(&input, output_length);
        assert_eq!(result.len(), output_length);
        println!("{:?}", result);
    }


    #[test]
    fn test_aes_128_ctr_seed_expansion() {
        let input = [0x00; 16];
        let output_length = 32;
        let result = aes_128_ctr_seed_expansion(input, output_length);
        assert_eq!(result.len(), output_length);
        println!("{:?}", result);
    }

}