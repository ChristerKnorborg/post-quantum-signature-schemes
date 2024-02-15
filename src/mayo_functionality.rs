use sha3::{Shake256, Digest};
use sha3::digest::{Update, ExtendableOutput, XofReader};
use rand::{Rng, rngs::OsRng, RngCore, SeedableRng};
use aes::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
use byteorder::{ByteOrder, LittleEndian};

use crate::bitsliced_functionality::{decode_bit_sliced_vector, decode_bytestring_to_matrix, decode_bytestring_to_vector};
use crate::constants::{CSK_BYTES, DIGEST_BYTES, EPK_BYTES, ESK_BYTES, F_Z, K, L_BYTES, M, N, O, O_BYTES, P1_BYTES, P2_BYTES, P3_BYTES, PK_SEED_BYTES, R_BYTES, SALT_BYTES, SIG_BYTES, SK_SEED_BYTES, V_BYTES};
use crate::finite_field::{matrix_mul, mul};
use crate::sample::sample_solution;
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
    type Aes128Ctr64LE = ctr::Ctr64LE<aes::Aes128>; // Define the type of the cipher (AES-128-CTR in little-endian mode)

    let key = pk_seed; // 16 bytes key
    let iv: [u8; 16] = [0u8; 16]; // 16 bytes IV

    let mut cipher = Aes128Ctr64LE::new(&key.into(), &iv.into());

    let mut output = Vec::with_capacity(output_length);

    let mut ctr: u128 = 0u128; // 128-bit counter (0 initial value) to encrypt

    while output.len() < output_length {
        let mut buf = [0u8; 16]; // 16 bytes buffer to store the counter
        LittleEndian::write_u128(&mut buf, ctr); // Write the counter to the buffer (array of bytes)
        cipher.apply_keystream(&mut buf); // Encrypt the counter with the key and IV
        output.extend_from_slice(&buf); // Append the encrypted counter to the output vector

        ctr += 1;
    }

    // Truncate the output to the desired length (if not multiple of 16 bytes)
    output.truncate(output_length);

    return output
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

    
    // Set pk_seed
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

        // transpose (Negation omitted as GF(16) negation of an element is the same as the element itself)
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


    let mut sk_seed: Vec<u8> = csk;
    let n_minus_o = N - O; // rows of O matrix

    let s = shake256(&sk_seed, PK_SEED_BYTES + O_BYTES);


    // Derive pk_seed from sk_seed
    let pk_seed_slice = &s[0..PK_SEED_BYTES];
    let pk_seed: [u8; PK_SEED_BYTES] = pk_seed_slice.try_into()
    .expect("Slice has incorrect length");


    // Derive O from sk_seed
    let mut o_bytestring = s[PK_SEED_BYTES ..].to_vec();
    let o = decode_bytestring_to_matrix(n_minus_o, O, o_bytestring.clone());

    
    // Derive P_{i}^(1) and P_{i}^(2) from pk_seed
    let p_bytes = aes_128_ctr_seed_expansion(pk_seed, P1_BYTES + P2_BYTES);
    let mut p1_bytes = p_bytes[0..P1_BYTES].to_vec();
    let p2_bytes = p_bytes[P1_BYTES..].to_vec();


    // m p1 matrices are of size (n−o) × (n−o)
    // m p2 matrices are of size (n−o) × o (not upper triangular matrices)
    let p1 = bf::decode_bit_sliced_matrices(n_minus_o, n_minus_o, p1_bytes.clone(), true);
    let p2 = bf::decode_bit_sliced_matrices(n_minus_o, O, p2_bytes, false);

    // Allocate space for L_i in [m]. Size is (n−o) × o per matrix
    let mut l = vec![vec![vec![0u8; O]; n_minus_o]; M];

    // Compute L matrices
    for i in 0..M {
         
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
pub fn sign(expanded_sk: Vec<u8>, message: &Vec<u8>) -> Vec<u8> {
    
    let n_minus_o = N - O; // rows of O matrix
    let mut x: Vec<u8> = vec![0u8; K*O]; // Initialize x to zero
    let mut v: Vec<Vec<u8>> = vec![vec![0u8; n_minus_o]; K];  // Initialize v to zero


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
    t_shake_input.extend(&m_digest); // Extend to prevent emptying original 
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
        for i in 0..K {
            let v_bytestring_slice = v_bytestring[i * V_BYTES..(i+1)*V_BYTES].to_vec(); 
            v[i] = bf::decode_bytestring_to_vector(n_minus_o, v_bytestring_slice)
        }

        // Derive r (Notice r is redefined and have nothing to do with previous r)
        let v_bytestring_remainder = v_bytestring[K*V_BYTES..].to_vec(); 
        let r = decode_bytestring_to_vector(K*O, v_bytestring_remainder); // Remainding part of v_bytestring. 
    


        // Build the linear system Ax = y
        let a: Vec<Vec<u8>> = vec![vec![0u8; K*O]; M]; // Make matrix of size m x k*o
        let mut y = &t;
        let ell = 0;
        let mut m_matrices: Vec<Vec<Vec<u8>>> = vec![vec![vec![0u8; O]; M]; K]; // Vector of size m x o of zeroes

        // Build K matrices of size M x O 
        for i in 0..K {
            
            let v_i_transpose = transpose_vector(&v[i]);

            for j in 0..M {
                let res = ff::matrix_mul(&v_i_transpose, &l[j]);
                m_matrices[i][j] = res[0].clone(); // Set the j-th row of m_i (unpack (o x 1) to row vector of size o)
            }
        }

        for i in 0..K {
            for j in (i..K).rev() {

                let v_i_transpose = transpose_vector(&v[i]);
                let mut u = vec![0x0 as u8; M];

                if i == j {
                    for a in 0..M {
                        let trans_mult = ff::matrix_mul(&v_i_transpose, &p1[a]);
                        

                        // Size (1 x (n-o)) * ((n-o) x (n-o)) * ((n-o)) x 1) gives size 1 x 1.
                        u[a] = ff::matrix_vector_mul(&trans_mult, &v[i])[0];  
                    }
                }
                else {
                    for a in 0..M {    

                        let trans_mult = ff::matrix_mul(&v_i_transpose, &p1[a]);
                        let left_term = ff::matrix_vector_mul(&trans_mult, &v[j])[0];

                        let v_j_transpose = transpose_vector(&v[j]);
                        let trans_mult = ff::matrix_mul(&v_j_transpose, &p1[a]);
                        let right_term = ff::matrix_vector_mul(&trans_mult, &v[i])[0];

                        u[a] = ff::add(left_term, right_term);
                    }
                }


                
                // let y_sub_u: Vec<u8> = y.iter().zip(u.iter()).map(|(y_idx, u_idx)| ff::sub(*y_idx, *u_idx)).collect();
                // for d in 0..m {
                //     y[d+ell] = y_sub_u[d];
                // }
                // let e_raised_to_ell = vec![0u8; F_Z.len()]; // [0, 0, 0, 0, 0]
                // for power in 0..ell {
                //     for poly_idx in 0..F_Z.len() {
                //         e_raised_to_ell[poly_idx] ^= ff::mul(F_Z[poly_idx], ell);
        
                //     }
                // }

                // reduce_y_mod_f(&mut y);
            }
        }

        // Try to solve the linear system Ax = y
        // x = match sample_solution(a, y) {
        //     Ok(x) => x, // If Ok
        //     Err(e) => {
        //         continue; // If Err (no solution found), continue to the next iteration of the loop
        //     }
        // };
        // break; // If Ok (solution found), break the loop

    } // ctr loop end


    return x;

    // // Finish and output signature
    // let mut signature = vec![0u8; K*N];

    // for i in 0..K {
        

    //     let x_idx = vec![x[i*O..(i+1)*O].to_vec()];
    //     let ox: Vec<u8> = ff::matrix_mul(&o, &x_idx)[0];
    //     let vi_mat = vec![v[i].clone()];
    //     let vi_plus_ox = ff::matrix_add(&vi_mat, &ox);

    //     signature.append(&mut vi_plus_ox);
    //     signature.append(&mut x_idx[0]);

    // }

    // return signature;

}

pub fn verify (expanded_pk: Vec<u8>, signature: Vec<u8>, message: &Vec<u8>) -> bool {

    let n_minus_o = N - O; // rows of O matrix

    // retrieves the public information from the expanded public key
    let p1_bytestring = expanded_pk[0..P1_BYTES].to_vec();
    let p2_bytestring = expanded_pk[P1_BYTES..P1_BYTES + P2_BYTES].to_vec();
    let p3_bytestring = expanded_pk[P1_BYTES + P2_BYTES..].to_vec();

    // decodes the public information into matrices
    let mut p1 = bf::decode_bit_sliced_matrices(n_minus_o, n_minus_o, p1_bytestring, true);
    let mut p2 = bf::decode_bit_sliced_matrices(n_minus_o, O, p2_bytestring, false);
    let mut p3 = bf::decode_bit_sliced_matrices(O, O, p3_bytestring, true);

    
    
    
    // decode signature and derive salt
    let n_times_k = if N*K % 2 == 0 {N*K / 2} else {N*K / 2 + 1}; // Ceil (N*K/2)
    let salt = signature [n_times_k .. n_times_k + SALT_BYTES].to_vec();
    let s = decode_bytestring_to_vector(K*N, signature);
    let mut s_matrix = vec![vec![0u8; N]; s.len()];
    for i in 0..K {
        s_matrix[i] = s[i*N..(i+1)*N].to_vec();
    }

    
    // derive and decode t
    let m_digest = shake256(&message, DIGEST_BYTES);
    let mut t_shake_input = Vec::new();
    t_shake_input.extend(&m_digest);
    t_shake_input.extend(&salt);
    let t_shake_output_length = if M % 2 == 0 {M / 2} else {M / 2 + 1}; // Ceil (M * log_2(q) / 8)
    let t_input: Vec<u8> = shake256(&t_shake_input, t_shake_output_length);
    let t: Vec<u8> = decode_bit_sliced_vector(t_input);

    // Compute P*(s)
    let y: Vec<u8> = vec![0u8; M];
    let mut ell: u8 = 0;

    // Construct the M matrices of size N x N s.t. (P^1_a P^2_a)
    // for every matrix a ∈ [M]                    (0     P^3_a)                             
    let big_p = create_large_matrices(p1, p2, p3); 

 
    println!("number of matrices: {:?}", big_p.len());
    println!("rows per matrix: {:?}", big_p[0].len());
    println!("cols per matrix: {:?}", big_p[0][0].len());


    for i in 0..K {
        let s_i_trans = transpose_vector(&s_matrix[i]);
        

        for j in (i..K).rev() {

            let mut u = vec![0u8; M];
            let s_j_trans = transpose_vector(&s_matrix[j]);

            for a in 0..M{

                let s_i_trans_big_p = ff::matrix_mul(&big_p[a], &s_i_trans);

                if i == j {
                    u[a] = ff::matrix_vector_mul(&s_i_trans_big_p, &s_matrix[i])[0];
                } else{

                    let left_term = ff::matrix_vector_mul(&s_i_trans_big_p, &s_matrix[j])[0];

                    let s_j_trans_big_p = ff::matrix_mul(&big_p[a], &s_j_trans);
                    let right_term = ff::matrix_vector_mul(&s_j_trans_big_p, &s_matrix[i])[0];
                    
                    u[a] = ff::add(left_term, right_term);   
                }

                // Y UPDATE HERE
                
                ell = ell + 1;

                println!("U: {:?}", u);
            }
        }
    }

    // Accept signature if y = t
    return y == t;
}



// Construct the matrix (P_1 P_2)
//                      (0   P_3)
fn create_large_matrices(mut p1: Vec<Vec<Vec<u8>>>, mut p2: Vec<Vec<Vec<u8>>>, mut p3: Vec<Vec<Vec<u8>>>) -> Vec<Vec<Vec<u8>>> {

    let mut result: Vec<Vec<Vec<u8>>> = Vec::with_capacity(M);



    for mat in 0..M {

        let mut rows = Vec::with_capacity(N);
        
        let mut zero_rows = vec![vec![0u8; N-O]; O ]; // O rows of zeroes of len N-O.
            
        for i in 0..(N-O ) {
            let new_vec = Vec::new();
            rows.push(new_vec);
            rows[i].append(&mut p1[mat][i]);
            rows[i].append(&mut p2[mat][i]);
        }

        for i in (N-O)..N {
            let new_vec = Vec::new();
            rows.push(new_vec);
            rows[i].append(&mut zero_rows[i-(N-O)]);
            rows[i].append(&mut p3[mat][i-(N-O)]);
        }

        result.push(rows);
    }


    return result;
}


    

fn reduce_y_mod_f(y: &mut Vec<u8>) {
    for i in (M..M + K * (K + 1) / 2 - 1).rev() {
        for j in 0..F_Z.len() {
            if i >= M + j {
                y[i - M + j] ^= ff::mul(y[i], F_Z[j]);
            }
        }
        y[i] = 0;
    }
}

pub fn api_sign(mut message : Vec<u8>, sk: Vec<u8>) -> Vec<u8> {
    //Expands the secret key
    let expanded_sk = expand_sk(sk);

    

    //creates the signature based on expanded secret key and message
    let mut signature = sign(expanded_sk, &message);

    //concatenates the signature and the message
    let mut sign_con_mes = Vec::new();
    sign_con_mes.append(&mut signature);
    sign_con_mes.append(&mut message);


    return sign_con_mes;
}

pub fn api_sign_open(sign_con_mes: Vec<u8>, pk: Vec<u8>) -> (bool , Vec<u8>) {
    
    //Expands the public key
    let expanded_pk = expand_pk(pk);


    //Extracts the signature and the message from the input
    let signature: Vec<u8> = sign_con_mes[0..SIG_BYTES].to_vec();
    let mut message = sign_con_mes[SIG_BYTES..].to_vec();

    //Verifies the signature based on expanded public key and message
    let result = verify(expanded_pk, signature, &message);

    //returns result and message
    if result == false
    {
        //dummy / false message if the signature is not valid
        message = vec![0u8];
    }


    return (result , message);
}



    








#[cfg(test)]
mod tests {
    use crate::utils::print_matrix;

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
    
    #[test]
    fn test_create_large_matrices(){

        let mut rng = rand::thread_rng();

        let mut p1: Vec<Vec<Vec<u8>>> = vec![vec![vec![1u8; N-O]; N-O]; M];
        let mut p2: Vec<Vec<Vec<u8>>> = vec![vec![vec![2u8; O]; N-O]; M];
        let mut p3: Vec<Vec<Vec<u8>>> = vec![vec![vec![3u8; O]; O]; M];
                    // Generate a random matrix of size (rows, cols)


                for m in 0..M{
                    for i in 0..N-O{
                        for j in 0..N-O{
                            
                            p1[m][i][j] = rng.gen_range(00..=15);

                            if(j < O){
                                p2[m][i][j] = rng.gen_range(00..=15);
                            }

                            if(i < O && j < O){
                                p3[m][i][j] = rng.gen_range(00..=15);
                            }
                        }
                    }
                }

        let big_matrix = create_large_matrices(p1.clone(), p2.clone(), p3.clone());
        
        
        
        let mut succeded: bool = true;

    for m in 0..M {
        for i in 0..N{
            for j in 0..N{
                if(i < N-O && j < N-O){
                    if(big_matrix[m][i][j] != p1[m][i][j]){
                        succeded = false;
                }
            }
                if (i < N-O && j < O) {
                    if(big_matrix[m][i][j+(N-O)] != p2[m][i][j]){
                        succeded = false;
                    }
                }

                // Should be zero
                if (i < O && j < O) {
                    if(big_matrix[m][i+(N-O)][j] != 0){
                        succeded = false;
                    }
                }
                
                if (i < O && j < O) {
                    if(big_matrix[m][i+(N-O)][j+(N-O)] != p3[m][i][j]){
                        succeded = false;
                    }
                }

            }
        }
    }


        assert_eq!(succeded, true);
        println!("Big Matrix test result: {:?}", succeded);

    }

}