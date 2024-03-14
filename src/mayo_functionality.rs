use std::vec;

use crate::bitsliced_functionality::{
    decode_bit_sliced_matrices, decode_bytestring_to_matrix, decode_bytestring_to_vector,
    encode_bit_sliced_matrices, encode_vector_to_bytestring,
};
use crate::crypto_primitives::{safe_aes_128_ctr, safe_randombytes, safe_shake256};
use crate::finite_field::{add, matrix_add, matrix_mul, matrix_sub, matrix_vector_mul, mul, sub};
use crate::sample::sample_solution;
use crate::utils::{transpose_matrix, transpose_vector};

use crate::constants::{
    CSK_BYTES, DIGEST_BYTES, EPK_BYTES, ESK_BYTES, F_Z, K, L_BYTES, M, N, O, V, O_BYTES, P1_BYTES,
    P2_BYTES, P3_BYTES, PK_SEED_BYTES, R_BYTES, SALT_BYTES, SIG_BYTES, SK_SEED_BYTES, V_BYTES,
};



// MAYO algorithm 5:
pub fn compact_key_gen() -> (Vec<u8>, Vec<u8>) {


    // Pick seed_sk at random (using NIST API for randomness)
    let mut sk_seed: Vec<u8> = vec![0u8; SK_SEED_BYTES];
    safe_randombytes(&mut sk_seed, SK_SEED_BYTES as u64);

    // Derive pk_seed and Oil space from sk_seed
    let mut s: Vec<u8> = vec![0u8; PK_SEED_BYTES + O_BYTES];
    safe_shake256(
        &mut s,
        (PK_SEED_BYTES + O_BYTES) as u64,
        &sk_seed,
        SK_SEED_BYTES as u64,
    );

    // Set pk_seed
    let pk_seed_slice = &s[0..PK_SEED_BYTES];
    let pk_seed: [u8; PK_SEED_BYTES] = pk_seed_slice
        .try_into()
        .expect("Slice has incorrect length");


    // Decode Oil space from seed_pk
    let o_bytes = s[PK_SEED_BYTES..].to_vec();
    let o = decode_bytestring_to_matrix(V, O, o_bytes);

    //Derive P1_i and P2_i from pk_seed
    let mut p: Vec<u8> = vec![0u8; P1_BYTES + P2_BYTES];
    safe_aes_128_ctr(
        &mut p,
        (P1_BYTES + P2_BYTES) as u64,
        &pk_seed,
        PK_SEED_BYTES as u64,
    );
    let p1_bytes = p[0..P1_BYTES].to_vec();
    let p2_bytes = p[P1_BYTES..].to_vec();

    // m p1 matrices are of size (n−o) × (n−o)
    // m p2 matrices are of size (n−o) × o (not upper triangular matrices)
    let p1 = decode_bit_sliced_matrices(V, V, p1_bytes, true);
    let p2 = decode_bit_sliced_matrices(V, O, p2_bytes, false);
    let mut p3 = vec![vec![vec![0u8; O]; O]; M]; // Allocate space for P_{i}^(3). Size is o × o

    // Compute P3_i as (−O^T * P1_i * O ) − (O^T * P2_i)
    // Notice, negation is omimtted as GF(16) negation of an element is the same as the element itself.
    for i in 0..M {

        let transposed_o = transpose_matrix(&o);

        // Compute: −O^T * P1_i * O
        let mut left_term = matrix_mul(&transposed_o, &p1[i]);
        left_term = matrix_mul(&left_term, &o);

        // Compute: −O^T * P2_i
        let right_term: Vec<Vec<u8>> = matrix_mul(&transposed_o, &p2[i]);

        // Compute: (−O^T * P1_i * O ) − (−O^T * P2_i)
        let sub = matrix_sub(&left_term, &right_term);

        p3[i] = upper(sub); 
    }

    let mut encoded_p3 = encode_bit_sliced_matrices(O, O, p3, true);

    // Public and secret keys
    let mut cpk = Vec::with_capacity(PK_SEED_BYTES + P3_BYTES); // contains pk_seed and encoded_p3
    let csk = sk_seed;

    cpk.extend_from_slice(&pk_seed);
    cpk.append(&mut encoded_p3);

    return (cpk, csk);
}






// MAYO algorithm 6.
// Expands a secret key from its compact representation
pub fn expand_sk(csk: &Vec<u8>) -> Vec<u8> {
    

    let sk_seed: &Vec<u8> = csk;

    // Derive S
    let mut s: Vec<u8> = vec![0u8; PK_SEED_BYTES + O_BYTES];
    safe_shake256(
        &mut s,
        (PK_SEED_BYTES + O_BYTES) as u64,
        &sk_seed,
        SK_SEED_BYTES as u64,
    );

    // Set pk_seed
    let pk_seed_slice = &s[0..PK_SEED_BYTES];
    let pk_seed: [u8; PK_SEED_BYTES] = pk_seed_slice
        .try_into()
        .expect("Slice has incorrect length");

    // Make Oil space from o_bytes. 
    let mut o_bytes = s[PK_SEED_BYTES..].to_vec();
    let o = decode_bytestring_to_matrix(V, O, o_bytes.clone());

    // Derive P1_i and P2_i from pk_seed
    let mut p_bytes: Vec<u8> = vec![0u8; P1_BYTES + P2_BYTES];
    safe_aes_128_ctr(
        &mut p_bytes,
        (P1_BYTES + P2_BYTES) as u64,
        &pk_seed,
        PK_SEED_BYTES as u64,
    );

    let mut p1_bytes = p_bytes[0..P1_BYTES].to_vec();
    let p2_bytes = p_bytes[P1_BYTES..].to_vec();

    // m p1 matrices are of size (n−o) × (n−o)
    // m p2 matrices are of size (n−o) × o (not upper triangular matrices)
    let p1 = decode_bit_sliced_matrices(V, V, p1_bytes.clone(), true);
    let p2 = decode_bit_sliced_matrices(V, O, p2_bytes, false);
    let mut l = vec![vec![vec![0u8; O]; V]; M]; // Allocate space for L_i in [m]. Size is (n−o) × o per matrix

    // Compute L matrices
    for i in 0..M {
      
        let transposed_p1_i = transpose_matrix(&p1[i]);
        let added_p1 = matrix_add(&p1[i], &transposed_p1_i);

        let left_term = matrix_mul(&added_p1, &o);

        l[i] = matrix_add(&left_term, &p2[i]);
    }

    let mut encoded_l = encode_bit_sliced_matrices(V, O, l, false);

    // To follow the refference implementation append O_bytestring at the end
    // Do not add sk_seed to the expanded secret key
    let mut expanded_sk: Vec<u8> = Vec::with_capacity(ESK_BYTES - SK_SEED_BYTES);

    expanded_sk.append(&mut p1_bytes);
    expanded_sk.append(&mut encoded_l);
    expanded_sk.append(&mut o_bytes);

    return expanded_sk;
}







// Mayo algorithm 7
// Expands a public key from its compact representation
pub fn expand_pk(cpk: Vec<u8>) -> Vec<u8> {


    let pk_seed_slice = &cpk[0..PK_SEED_BYTES];
    let pk_seed: [u8; PK_SEED_BYTES] = pk_seed_slice
        .try_into()
        .expect("Slice has incorrect length");

    let mut aes_bytes = vec![0u8; P1_BYTES + P2_BYTES];
    safe_aes_128_ctr(
        &mut aes_bytes,
        (P1_BYTES + P2_BYTES) as u64,
        &pk_seed,
        PK_SEED_BYTES as u64,
    );

    let mut expanded_pk = Vec::with_capacity(EPK_BYTES);
    let mut cpk_bytes = cpk[PK_SEED_BYTES..].to_vec();

    expanded_pk.append(&mut aes_bytes);
    expanded_pk.append(&mut cpk_bytes);

    return expanded_pk;
}







// MAYO algorithm 8
// Signs a message using an expanded secret key
pub fn sign(compact_secret_key: &Vec<u8>, message: &Vec<u8>) -> Vec<u8> {

    // Initialize x and v to zero
    let mut x: Vec<u8> = vec![0u8; K * O]; 
    let mut v: Vec<Vec<u8>> = vec![vec![0u8; V]; K]; 

    let expanded_sk = expand_sk(&compact_secret_key);

    // Decode expanded secret key
    let p1_bytestring = expanded_sk[..P1_BYTES].to_vec();
    let l_bytestring = expanded_sk[P1_BYTES..L_BYTES + P1_BYTES].to_vec();
    let o_bytestring = expanded_sk[P1_BYTES + L_BYTES..].to_vec();

    // Assign matrices with decoded information
    let o = decode_bytestring_to_matrix(V, O, o_bytestring);
    let p1 = decode_bit_sliced_matrices(V, V, p1_bytestring, true);
    let l = decode_bit_sliced_matrices(V, O, l_bytestring, false);

    // Hash message
    let mut m_digest: Vec<u8> = vec![0u8; DIGEST_BYTES];
    safe_shake256(
        &mut m_digest,
        DIGEST_BYTES as u64,
        &message,
        message.len() as u64,
    );

    // Assign randomness to r using NIST API randomness
    let mut r: Vec<u8> = vec![0x0; R_BYTES];
    safe_randombytes(&mut r, R_BYTES as u64);


    // Derive salt
    let mut salt_input: Vec<u8> = Vec::with_capacity(DIGEST_BYTES + R_BYTES + SK_SEED_BYTES);
    salt_input.extend(&m_digest); 
    salt_input.append(&mut r);
    salt_input.extend(compact_secret_key); 
    let mut salt = vec![0u8; SALT_BYTES];
    safe_shake256(
        &mut salt,
        SALT_BYTES as u64,
        &salt_input,
        (DIGEST_BYTES + R_BYTES + SK_SEED_BYTES) as u64,
    );

    // Derive t
    let mut t_shake_input = Vec::with_capacity(DIGEST_BYTES + SALT_BYTES);
    t_shake_input.extend(&m_digest); 
    t_shake_input.extend(&salt); 
    let t_shake_output_length = if M % 2 == 0 { M / 2 } else { M / 2 + 1 }; // Ceil (M * log_2(q) / 8)

    let mut t_output: Vec<u8> = vec![0u8; t_shake_output_length];
    safe_shake256(
        &mut t_output,
        t_shake_output_length as u64,
        &t_shake_input,
        (DIGEST_BYTES + SALT_BYTES) as u64,
    );

    let t = decode_bytestring_to_vector(M, t_output);


    // Attempt to find a preimage for t
    for ctr in 0..=255 {
        // Derive V from shake256
        let mut v_shake_input = Vec::with_capacity(DIGEST_BYTES + SALT_BYTES + CSK_BYTES + 1);
        v_shake_input.extend(&m_digest);
        v_shake_input.extend(&salt);
        v_shake_input.extend(compact_secret_key);
        v_shake_input.extend(vec![ctr]);
        let ceil_exp = if K * O % 2 == 0 {
            K * O / 2
        } else {
            K * O / 2 + 1
        }; // Ceil (K*O * log_2(q) / 8)
        let v_shake_output_length = K * V_BYTES + ceil_exp;

        let mut v_bytestring: Vec<u8> = vec![0u8; v_shake_output_length];
        safe_shake256(
            &mut v_bytestring,
            v_shake_output_length as u64,
            &v_shake_input,
            (DIGEST_BYTES + SALT_BYTES + SK_SEED_BYTES + 1) as u64,
        );

        // Derive v_i
        for i in 0..K {
            let v_bytestring_slice = v_bytestring[i * V_BYTES..(i + 1) * V_BYTES].to_vec();
            v[i] = decode_bytestring_to_vector(V, v_bytestring_slice)
        }

        // Derive r (Notice r is redefined and have nothing to do with previous r (large R in the paper))
        let v_bytestring_remainder = v_bytestring[K * V_BYTES..].to_vec();
        let r = decode_bytestring_to_vector(K * O, v_bytestring_remainder); // Remainding part of v_bytestring.



        // Build the linear system Ax = y
        let shifts: usize = (K * (K + 1) / 2) - 1; // Number of shifts in the polynomial (max ell)

        // Assign additional space for A and y to allow for shifts (multiplication with z^ell)
        let mut a: Vec<Vec<u8>> = vec![vec![0u8; K * O]; M + shifts]; 
        let mut y = Vec::with_capacity(M + shifts);
        y.extend(t.clone()); 
        y.extend(vec![0u8; shifts]);
        let mut ell = 0;
        let mut m_matrices: Vec<Vec<Vec<u8>>> = vec![vec![vec![0u8; O]; M]; K]; // Vector of size m x o of zeroes

        // Build K matrices of size M x O
        for i in 0..K {
            let v_i_transpose = transpose_vector(&v[i]);

            for j in 0..M {
                let res = matrix_mul(&v_i_transpose, &l[j]);
                m_matrices[i][j] = res[0].clone(); // Set the j-th row of m_i (unpack (o x 1) to row vector of size o)
            }
        }

        for i in 0..K {
            for j in (i..K).rev() {
                let v_i_transpose = transpose_vector(&v[i]);
                let mut u = vec![0x0 as u8; M];

                if i == j {
                    for a in 0..M {
                        let trans_mult = matrix_mul(&v_i_transpose, &p1[a]);

                        u[a] = matrix_vector_mul(&trans_mult, &v[i])[0];
                    }
                } else {
                    for a in 0..M {
                        let trans_mult = matrix_mul(&v_i_transpose, &p1[a]);
                        let left_term = matrix_vector_mul(&trans_mult, &v[j])[0];

                        let v_j_transpose = transpose_vector(&v[j]);
                        let trans_mult = matrix_mul(&v_j_transpose, &p1[a]);
                        let right_term = matrix_vector_mul(&trans_mult, &v[i])[0];

                        u[a] = add(left_term, right_term);
                    }
                }

                // y = y - u * z^ell - Instead of subtracting with shifted u,
                // we just sub (XOR) with shifted y for easier loop structre.
                for d in 0..M {
                    y[d + ell] ^= u[d];
                }

                // Update A cols with + z^ell * Mj
                for col in i * O..(i + 1) * O {
                    for row in 0..M {
                        a[row + ell][col] ^= m_matrices[j][row][col % O];
                    }
                }

                if i != j {
                    // Update A cols with + z^ell * Mi
                    for col in j * O..(j + 1) * O {
                        for row in 0..M {
                            a[row + ell][col] ^= m_matrices[i][row][col % O];
                        }
                    }
                }
                ell += 1;
            }
        }

        y = reduce_mod_f(y);
        a = reduce_a_mod_f(a);

        // Try to solve the linear system Ax = y
        x = match sample_solution(a, y, r) {
            Ok(x) => x, // If Ok
            Err(_) => {
                continue; // If Err (no solution found), continue to the next iteration of the loop
            }
        };
        break; // If Ok, break the loop
    } // ctr loop ends

    // Finish and output signature
    let mut signature = Vec::with_capacity(K * N);

    for i in 0..K {
        let mut x_idx = x[i * O..(i + 1) * O].to_vec();
        let mut ox: Vec<u8> = matrix_vector_mul(&o, &x_idx);


        // add v[i] to ox
        for j in 0..V {
            ox[j] = add(ox[j], v[i][j]);
        }

        signature.append(&mut ox);
        signature.append(&mut x_idx);
    }

    let mut sign_con_salt = Vec::with_capacity(SIG_BYTES);
    let signature_encoded = encode_vector_to_bytestring(signature);
    sign_con_salt.extend(signature_encoded);
    sign_con_salt.extend(salt);
    return sign_con_salt;
}










// MAYO algorithm 9
// Verifies the signature of a message using an expanded public key
pub fn verify(expanded_pk: Vec<u8>, signature: Vec<u8>, message: &Vec<u8>) -> bool {


    // retrieve the public information from the expanded public key
    let p1_bytestring = expanded_pk[0..P1_BYTES].to_vec();
    let p2_bytestring = expanded_pk[P1_BYTES..P1_BYTES + P2_BYTES].to_vec();
    let p3_bytestring = expanded_pk[P1_BYTES + P2_BYTES..].to_vec();

    // decode the public information into matrices
    let p1 = decode_bit_sliced_matrices(V, V, p1_bytestring, true);
    let p2 = decode_bit_sliced_matrices(V, O, p2_bytestring, false);
    let p3 = decode_bit_sliced_matrices(O, O, p3_bytestring, true);

    // decode signature and derive salt
    let salt = signature[SIG_BYTES - SALT_BYTES..SIG_BYTES].to_vec();
    let s = decode_bytestring_to_vector(K * N, signature);

    let mut s_matrix = vec![vec![0u8; N]; K];
    for i in 0..K {
        s_matrix[i] = s[i * N..(i + 1) * N].to_vec();
    }

    // Hash message
    let mut m_digest: Vec<u8> = vec![0u8; DIGEST_BYTES];
    safe_shake256(
        &mut m_digest,
        DIGEST_BYTES as u64,
        &message,
        message.len() as u64,
    );


    // Derive t
    let mut t_shake_input = Vec::with_capacity(DIGEST_BYTES + SALT_BYTES);
    t_shake_input.extend(&m_digest); 
    t_shake_input.extend(&salt); 
    let t_shake_output_length = if M % 2 == 0 { M / 2 } else { M / 2 + 1 }; // Ceil (M * log_2(q) / 8)

    let mut t_output: Vec<u8> = vec![0u8; t_shake_output_length];
    safe_shake256(
        &mut t_output,
        t_shake_output_length as u64,
        &t_shake_input,
        (DIGEST_BYTES + SALT_BYTES) as u64,
    );

    let t = decode_bytestring_to_vector(M, t_output);


    // Compute P*(s)
    let shifts: usize = (K * (K + 1) / 2) - 1; // Number of shifts in the polynomial (max ell)
    let mut y = vec![0u8; M + shifts];
    let mut ell = 0;

    // Construct the M matrices of size N x N s.t. (P^1_a P^2_a)
    // for every matrix a ∈ [M]                    (0     P^3_a)
    let big_p = create_big_p_matrices(p1, p2, p3);

    for i in 0..K {
        
        let s_i_trans = transpose_vector(&s_matrix[i]);

        for j in (i..K).rev() {
            let mut u = vec![0u8; M];
            let s_j_trans = transpose_vector(&s_matrix[j]);

            for a in 0..M {
                let res_mul = matrix_mul(&s_i_trans, &big_p[a]);

                if i == j {
                    u[a] = matrix_vector_mul(&res_mul, &s_matrix[i])[0];
                } else {
                    let left_term = matrix_vector_mul(&res_mul, &s_matrix[j])[0];

                    let resmul2 = matrix_mul(&s_j_trans, &big_p[a]);
                    let right_term = matrix_vector_mul(&resmul2, &s_matrix[i])[0];

                    u[a] = add(left_term, right_term);
                }
            }

            for d in 0..M {
                y[d + ell] ^= u[d];
            }

            ell = ell + 1;
        }
    }

    y = reduce_mod_f(y);

    // Accept signature if y = t
    return y == t;
}






// MAYO algorithm 10
// Expands a secret key from its compact representation and signs a message input
pub fn api_sign(mut message: Vec<u8>, csk: Vec<u8>) -> Vec<u8> {
  

    // Create signature based on expanded secret key and message
    let mut signature = sign(&csk, &message);

    // Concatenate the signature and the message
    let mut sign_con_mes = Vec::with_capacity(SIG_BYTES + message.len());
    sign_con_mes.append(&mut signature);
    sign_con_mes.append(&mut message);

    return sign_con_mes;
}







// MAYO algorithm 11
// Expands a public key from its compact representation and verifies a signature
pub fn api_sign_open(sign_con_mes: Vec<u8>, pk: Vec<u8>) -> (bool, Vec<u8>) {
    
    // Expand the public key
    let expanded_pk = expand_pk(pk);

    // Extract signature and message from input
    let signature: Vec<u8> = sign_con_mes[0..SIG_BYTES].to_vec();
    let mut message = sign_con_mes[SIG_BYTES..].to_vec();

    // Verifie signature 
    let result = verify(expanded_pk, signature, &message);

    if result == false {
        message = vec![0u8]; // Message of zeroes if the signature is not valid
    }
    return (result, message);
}













// Helper function to apply the upper function to a matrix (as described in the MAYO paper)
pub fn upper(mut matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let n = matrix.len();

    // Iterate over everything above the diagonal
    for i in 0..n {
        for j in (i + 1)..n {
            matrix[i][j] ^= matrix[j][i]; // GF(16) addition is the same as XOR
            matrix[j][i] = 0;
        }
    }
    return matrix;
}




// Construct the matrices on format (P_1 P_2)
//                                  (0   P_3)
fn create_big_p_matrices(
    mut p1: Vec<Vec<Vec<u8>>>,
    mut p2: Vec<Vec<Vec<u8>>>,
    mut p3: Vec<Vec<Vec<u8>>>,
) -> Vec<Vec<Vec<u8>>> {
    let mut result: Vec<Vec<Vec<u8>>> = Vec::with_capacity(M);

    for mat in 0..M {
        let mut rows = Vec::with_capacity(N);

        let mut zero_rows = vec![vec![0u8; N - O]; O]; // O rows of zeroes of len N-O.

        for i in 0..(N - O) {
            let new_vec = Vec::with_capacity(N);
            rows.push(new_vec);
            rows[i].append(&mut p1[mat][i]);
            rows[i].append(&mut p2[mat][i]);
        }

        for i in (N - O)..N {
            let new_vec = Vec::with_capacity(N);
            rows.push(new_vec);
            rows[i].append(&mut zero_rows[i - (N - O)]);
            rows[i].append(&mut p3[mat][i - (N - O)]);
        }

        result.push(rows);
    }
    return result;
}



// Perform the reduction of with f(z)
pub fn reduce_mod_f(mut polynomial: Vec<u8>) -> Vec<u8> {
    
    for i in (M..polynomial.len()).rev() {
        for (shift, coef) in F_Z {

            let mul_res = mul(polynomial[i], coef);
            polynomial[i - M + shift] = sub(polynomial[i - M + shift], mul_res);
        }
        polynomial[i] = 0; // set original term to 0 After distributing coefficient
    }

    // Truncate to first m entries (every other entry is zero after reduction)
    polynomial.truncate(M);

    return polynomial;
}


// Perform the reduction of with f(z) on a matrix.
// The reduction is applied to the columns of the matrix
pub fn reduce_a_mod_f(mut a: Vec<Vec<u8>>) -> Vec<Vec<u8>> {

    let shifts: usize = (K * (K + 1) / 2) - 1; // Number of shifts in the polynomial (max ell)
    for col in 0..K * O {
        for row in (M..M + shifts).rev() {

            for (shift, coef) in F_Z {
                let mul_res = mul(a[row][col], coef);

                a[row - M + shift][col] = sub(a[row - M + shift][col], mul_res);
            }
            a[row][col] = 0;
        }
    }

    // Remove additional rows from reduction (remaining rows are zero after reduction)
    a.truncate(M);

    return a;
}


#[cfg(test)]
mod tests {
    use crate::utils::print_matrix;

    use super::*;
    use rand::Rng;

    #[test]
    fn test_create_large_matrices() {
        let mut rng = rand::thread_rng();

        let mut p1: Vec<Vec<Vec<u8>>> = vec![vec![vec![1u8; N - O]; N - O]; M];
        let mut p2: Vec<Vec<Vec<u8>>> = vec![vec![vec![2u8; O]; N - O]; M];
        let mut p3: Vec<Vec<Vec<u8>>> = vec![vec![vec![3u8; O]; O]; M];
        // Generate a random matrix of size (rows, cols)

        for m in 0..M {
            for i in 0..N - O {
                for j in 0..N - O {
                    p1[m][i][j] = rng.gen_range(00..=15);

                    if j < O {
                        p2[m][i][j] = rng.gen_range(00..=15);
                    }

                    if i < O && j < O {
                        p3[m][i][j] = rng.gen_range(00..=15);
                    }
                }
            }
        }

        let big_matrices = create_big_p_matrices(p1.clone(), p2.clone(), p3.clone());

        for m in 0..M {
            println!("NEW matrix");
            print_matrix(big_matrices[m].clone());
        }

        let mut succeded: bool = true;

        for m in 0..M {
            for i in 0..N {
                for j in 0..N {
                    if i < N - O && j < N - O {
                        if big_matrices[m][i][j] != p1[m][i][j] {
                            succeded = false;
                        }
                    }
                    if i < N - O && j < O {
                        if big_matrices[m][i][j + (N - O)] != p2[m][i][j] {
                            succeded = false;
                        }
                    }

                    // Should be zero
                    if i < O && j < O {
                        if big_matrices[m][i + (N - O)][j] != 0 {
                            succeded = false;
                        }
                    }

                    if i < O && j < O {
                        if big_matrices[m][i + (N - O)][j + (N - O)] != p3[m][i][j] {
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
