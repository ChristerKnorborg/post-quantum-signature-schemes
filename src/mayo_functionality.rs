
use sha3::{Shake256, Digest};
use sha3::digest::{Update, ExtendableOutput, XofReader};
use rand::{Rng, rngs::OsRng, RngCore, SeedableRng};
use aes_prng::AesRng;

use crate::bitsliced_functionality::{decode_bit_sliced_vector, decode_bytestring_to_matrix, decode_bytestring_to_vector};
use crate::constants::{CSK_BYTES, DIGEST_BYTES, EPK_BYTES, ESK_BYTES, K, L_BYTES, M, N, O, O_BYTES, P1_BYTES, P2_BYTES, P3_BYTES, PK_SEED_BYTES, R_BYTES, SALT_BYTES, SK_SEED_BYTES, V_BYTES};
use crate::finite_field::{matrix_mul, mul};
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


// Helper function to transpose a matrix (as described in the MAYO paper)
pub fn transpose_matrix(matrix: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
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

// Helper function to transpose a matrix (as described in the MAYO paper)
pub fn transpose_vector(vector: &Vec<u8>) -> Vec<Vec<u8>> {

    let rows = vector.len();

    // Create a new transposed matrix with the dimensions swapped
    let mut transposed = vec![vec![0u8; rows]; 1]; 

    for i in 0..rows {
            transposed[0][i] = vector[i]; // Swap elements
    }
    return transposed
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
        let transposed_o = transpose_matrix(&o);

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


// MAYO algorithm 6.
// Expands a secret key from its compact representation
pub fn expand_sk(csk: Vec<u8>) -> Vec<u8> {

    // Pick random seed (same length as salt_bytes)
    let mut sk_seed: Vec<u8> = csk;


    let n_minus_o = N - O; // rows of O matrix


    let s = shake256(&sk_seed, PK_SEED_BYTES + O_BYTES);

    let pk_seed_slice = &s[0..PK_SEED_BYTES];
    let pk_seed: [u8; PK_SEED_BYTES] = pk_seed_slice.try_into()
    .expect("Slice has incorrect length");

    let mut o_bytestring = s[PK_SEED_BYTES ..].to_vec();
    let o = decode_bytestring_to_matrix(n_minus_o, O, o_bytestring.clone());

    
    let p_bytes = aes_128_ctr_seed_expansion(pk_seed, P1_BYTES + P2_BYTES);
    
    let mut p1_bytes = p_bytes[0..P1_BYTES].to_vec();
    let p2_bytes = p_bytes[P1_BYTES..].to_vec();


    // m p1 matrices are of size (n−o) × (n−o)
    let p1 = bf::decode_bit_sliced_matrices(n_minus_o, n_minus_o, p1_bytes.clone(), true);

    // m p2 matrices are of size (n−o) × o (not upper triangular matrices)
    let p2 = bf::decode_bit_sliced_matrices(n_minus_o, O, p2_bytes, false);

     // Allocate space for L_i in [m]. Size is (n−o) × o per matrix
     let mut l = vec![vec![vec![0u8; O]; n_minus_o]; M];

    // Compute L matrices
    for i in 0..M {
        // transpose 
        let p1_i = &p1[i];
        let p2_i = &p2[i];

        let transposed_p1_i = transpose_matrix(p1_i);
        let added_p1 = ff::matrix_add(&p1_i, &transposed_p1_i);

        let left_term = ff::matrix_mul(&added_p1, &o);

        l[i] = ff::matrix_add(&left_term, &p2_i);
    }

    let mut encoded_l = bf::encode_bit_sliced_matrices(n_minus_o, O, l, false);

    let mut expanded_sk: Vec<u8> = Vec::with_capacity(ESK_BYTES);

    expanded_sk.append(&mut sk_seed);
    expanded_sk.append(&mut o_bytestring);
    expanded_sk.append(&mut p1_bytes);
    expanded_sk.append(&mut encoded_l);


    return expanded_sk;
    
}



// Mayo algorithm 7
// Expands a public key from its compact representation
pub fn expand_pk(cpk: Vec<u8>) -> Vec<u8> {
    let pk_seed_slice = &cpk[0..PK_SEED_BYTES];
    let pk_seed: [u8; PK_SEED_BYTES] = pk_seed_slice.try_into()
    .expect("Slice has incorrect length");

    let mut aes_bytes = aes_128_ctr_seed_expansion(pk_seed, P1_BYTES + P2_BYTES);

    let mut expanded_pk = Vec::with_capacity(EPK_BYTES);
    let mut cpk_bytes = cpk[PK_SEED_BYTES..].to_vec();

    expanded_pk.append(&mut aes_bytes);
    expanded_pk.append(&mut cpk_bytes);

    return expanded_pk;
}



// MAYO algorithm 8
// 
pub fn sign(expanded_sk: Vec<u8>, message: Vec<u8>) -> Vec<u8> {
    
    let n_minus_o = N - O; // rows of O matrix


    // Decode expanded secret key
    let sk_seed: Vec<u8> = expanded_sk[0..SK_SEED_BYTES].to_vec();
    let o_bytestring = expanded_sk[SK_SEED_BYTES..SK_SEED_BYTES + O_BYTES].to_vec();
    let p1_bytestring = expanded_sk[SK_SEED_BYTES + O_BYTES..SK_SEED_BYTES + O_BYTES + P1_BYTES].to_vec();
    let l_bytestring = expanded_sk[SK_SEED_BYTES + O_BYTES + P1_BYTES..ESK_BYTES].to_vec();

    // Assign matrices with decoded information
    let o = bf::decode_bytestring_to_matrix(n_minus_o, O, o_bytestring);
    let p1 = bf::decode_bit_sliced_matrices(n_minus_o, n_minus_o, p1_bytestring, true);
    let l = bf::decode_bit_sliced_matrices(n_minus_o, O, l_bytestring, false);
    
    // Hash message and derive salt
    let m_digest = shake256(&message, DIGEST_BYTES);
    let mut r = vec![0x0 as u8, R_BYTES as u8]; // all 0's. Optimization availible (line 9 in algorithm 8)
    let mut salt_input: Vec<u8> = Vec::new();
    salt_input.extend(&m_digest); // Extend to prevent emptying original
    salt_input.append(&mut r);
    salt_input.extend(&sk_seed); // Extend to prevent emptying original sk_seed
    let salt = shake256(&salt_input, SALT_BYTES);

    // Derive t
    let mut t_shake_input = Vec::new();
    t_shake_input.extend(m_digest.clone()); // Extend to prevent emptying original 
    t_shake_input.extend(&salt); // Extend to prevent emptying original
    let t_shake_output_length = if M % 2 == 0 {M / 2} else {M / 2 + 1}; // Ceil (M * log_2(q) / 8)
    let t_input = shake256(&t_shake_input, t_shake_output_length);
    let t = decode_bit_sliced_vector(t_input);
    

    // Attempt to find a preimage for t
    for ctr in 0..=255 {

        // Derive v_i and r 
        let mut v_shake_input = Vec::new();
        v_shake_input.extend(&m_digest);
        v_shake_input.extend(&salt);
        v_shake_input.extend(&sk_seed);
        v_shake_input.extend(vec![ctr]);
        let ceil_exp = if K*O % 2 == 0 {K*O / 2} else {K*O / 2 + 1}; // Ceil (K*O * log_2(q) / 8)
        let v_shake_output_length = K * V_BYTES + ceil_exp;
        let v_bytestring = shake256(&v_shake_input, v_shake_output_length);

        // Derive v_i
        let mut v: Vec<Vec<u8>> = Vec::with_capacity(K); 
        for i in 0..K {
            let v_bytestring_slice = v_bytestring[i * V_BYTES..(i+1)*V_BYTES].to_vec(); 
            v[i] = bf::decode_bytestring_to_vector(n_minus_o, v_bytestring_slice)
        }
        // Derive r (Notice r is redefined and have nothing to do with previous r)
        let v_bytestring_remainder = v_bytestring[K*V_BYTES..].to_vec(); 
        let r = decode_bytestring_to_vector(K*O, v_bytestring_remainder); // Remainding part of v_bytestring. 
    


    // Build the linear system Ax = y
    let a: Vec<Vec<u8>> = vec![vec![0u8; K*O]; M]; // Make matrix of size m x k*o
    let y = &t;
    let ell = 0;


    // Build K matrices of size M x O 
    for i in 0..K {
        let mut m: Vec<Vec<Vec<u8>>> = vec![vec![vec![0u8; O]; M]; K]; // Vector of size m x o of zeroes
        let v_i_transpose = transpose_vector(&v[i]);

        for j in 0..M {
            let res = ff::matrix_mul(&v_i_transpose, &l[j]);
            m[i][j] = res[0].clone(); // Set the j-th row of m_i (unpack (o x 1) to row vector of size o)
        }
    }

    for i in 0..K {
        for j in (0..K).rev() {

            let v_i_transpose = transpose_vector(&v[i]);
            let u = vec![0x0 as u8; M];

            if i == j {
                for p1_mat in p1.iter() {
                    let trans_mult = ff::matrix_mul(&v_i_transpose, p1_mat);
                    let v_i_matrix = vec![v[i]]; 

                    // Size (1 x (n-o)) * ((n-o) x (n-o)) * ((n-o)) x 1) gives size 1 x 1.
                    // Hence, we index in [0][0] as both dimensions are wrapped in a vector.
                    u[i] = ff::matrix_mul(&trans_mult, &v_i_matrix)[0][0]; // 
                }
            }
            else {
                for p1_mat in p1.iter() {    
                    let trans_mult = ff::matrix_mul(&v_i_transpose, p1_mat);
                    let v_j_matrix = vec![v[j]];

                    let left_term = ff::matrix_mul(&trans_mult, &v_j_matrix)[0][0];

                    let v_j_transpose = transpose_vector(&v[j]);
                    let trans_mult = ff::matrix_mul(&v_j_transpose, p1_mat);
                    let v_i_matrix = vec![v[i]];
                    let right_term = ff::matrix_mul(&trans_mult, &v_i_matrix)[0][0];

                    u[i] = ff::add(left_term, right_term);
                }
            }
            
            let e_ell_u = 
            y = ff::sub(y, );

            }
            
        }

        

    }

    let s = shake256(&sk_seed, PK_SEED_BYTES + O_BYTES);

    let pk_seed_slice = &s[0..PK_SEED_BYTES];
    let pk_seed: [u8; PK_SEED_BYTES] = pk_seed_slice.try_into()
    .expect("Slice has incorrect length");
 


    }

    return vec![0u8; 0];
    
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