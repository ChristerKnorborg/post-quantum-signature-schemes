use std::vec;
use crate::crypto_primitives::{safe_aes_128_ctr, safe_random_bytes, safe_shake256};
use crate::finite_field::{add, mul};
use crate::sample::sample_solution;

use crate::constants::{
    CPK_BYTES, CSK_BYTES, DIGEST_BYTES, EPK_BYTES, ESK_BYTES, F_Z, K, L_BYTES, M, N, V, O, O_BYTES, P1_BYTES, P2_BYTES, P3_BYTES,
    PK_SEED_BYTES, R_BYTES, SALT_BYTES, SIG_BYTES, SK_SEED_BYTES, V_BYTES, SHIFTS
};
use crate::{
    decode_bit_sliced_array, decode_bit_sliced_matrices, decode_bytestring_matrix_array, decode_bytestring_to_array, 
    encode_bit_sliced_array, encode_bit_sliced_matrices, encode_to_bytestring_array, matrix_add, matrix_mul, matrix_vec_mul,
    transpose_matrix_array, vector_matrix_mul, vector_mul, vector_transposed_matrix_mul
};





// MAYO algorithm 5:
pub fn compact_key_gen() -> ([u8 ; CPK_BYTES], [u8 ; CSK_BYTES]) {
    
    // Pick random seed_sk at random (using NIST randomness source)
    let mut sk_seed = [0u8; SK_SEED_BYTES];
    safe_random_bytes(&mut sk_seed, SK_SEED_BYTES as u64);

    // Derive pk_seed and Oil space O from sk_seed
    let mut s = [0u8; PK_SEED_BYTES + O_BYTES];
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

    // Make Oil space from o_bytes. Only a single is yielded from decode_bit_sliced_matrices in this case
    let o_bytes = &s[PK_SEED_BYTES..PK_SEED_BYTES+O_BYTES];
    let o = decode_bytestring_matrix_array!(o_bytes, V, O);

    // Derive P1_i and P2_i from pk_seed
    let mut p = [0u8; P1_BYTES + P2_BYTES];
    safe_aes_128_ctr(
        &mut p,
        (P1_BYTES + P2_BYTES) as u64,
        &pk_seed,
        PK_SEED_BYTES as u64,
    );
    let p1_bytes = &p[0..P1_BYTES];
    let p2_bytes = &p[P1_BYTES..];

    
    let p1 = decode_bit_sliced_matrices!(p1_bytes, V, V, M, true);    // m p1 matrices are of size (n−o) × (n−o)
    let p2 = decode_bit_sliced_matrices!(p2_bytes, V, O, M, false);    // m p2 matrices are of size (n−o) × o (not upper triangular matrices)
    let mut p3 = [[[0u8; O]; O]; M];                                    // Allocate space for P_{i}^(3). Size is o × o


    // Compute P3_i as (−O^T * P1_i * O ) − (−O^T * P2_i)
    // Notice, negation is omimtted as GF(16) negation of an element is the same as the element itself.
    for i in 0..M {
        // transpose (Negation omitted as GF(16) negation of an element is the same as the element itself)
        let transposed_o = transpose_matrix_array!(o, V, O);


        // P3 = O^t * (P1*O + P2) 
        // Compute: P1*O + P2
        let mut temp = matrix_mul!(p1[i], V, V, o, O); // (n−o) × (n−o) * (n−o) × o = (n−o) × o
        temp = matrix_add!(temp, p2[i], V, O);
        

        // Upper triangular part of the result
        p3[i] = upper(matrix_mul!(transposed_o, O, V, temp, O)); //  o × (n−o) * (n−o) × o = o × o
    }

    let encoded_p3: [u8 ; P3_BYTES] = encode_bit_sliced_matrices!(p3, O, O, M, true, P3_BYTES); // m p3 matrices are of size o × o

    // Public and secret keys
    let mut cpk = [0u8 ; PK_SEED_BYTES + P3_BYTES]; // contains pk_seed and encoded_p3
    let csk: [u8 ; CSK_BYTES] = sk_seed;

    cpk[..PK_SEED_BYTES].copy_from_slice(&pk_seed);
    cpk[PK_SEED_BYTES..].copy_from_slice(&encoded_p3);

    return (cpk, csk);
}





// MAYO algorithm 6.
// Expands a secret key from its compact representation
pub fn expand_sk(csk: [u8 ; CSK_BYTES]) -> [u8 ; ESK_BYTES-SK_SEED_BYTES]{
    let sk_seed = csk;

    // Derive pk_seed and Oil space from sk_seed
    let mut s = [0u8; PK_SEED_BYTES + O_BYTES];
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

    // Make Oil space from o_bytes. Only a single is yielded from decode_bit_sliced_matrices in this case
    let o_bytes = &s[PK_SEED_BYTES..PK_SEED_BYTES+O_BYTES];
    let o = decode_bytestring_matrix_array!(o_bytes, V, O);


    //Derive P_{i}^(1) and P_{i}^(2) from pk_seed
    let mut p = [0u8; P1_BYTES + P2_BYTES];
    safe_aes_128_ctr(
        &mut p,
        (P1_BYTES + P2_BYTES) as u64,
        &pk_seed,
        PK_SEED_BYTES as u64,
    );
    let p1_bytes = &p[0..P1_BYTES];
    let p2_bytes = &p[P1_BYTES..];


    let p1 = decode_bit_sliced_matrices!(p1_bytes, V, V, M, true);  // m p1 matrices are of size (n−o) × (n−o)
    let p2 = decode_bit_sliced_matrices!(p2_bytes, V, O, M, false);  // m p2 matrices are of size (n−o) × o (not upper triangular matrices)
    let mut l = [[[0u8; O]; V]; M];                                  // Allocate space for L_i in [m]. Size is (n−o) × o per matrix
    

    // Compute L matrices
    for i in 0..M {

        // P1^T + P1
        let mut added_p1 = transpose_matrix_array!(p1[i], V, V);
        added_p1 = matrix_add!(added_p1, p1[i], V, V); 

        // (P1 + P^T) i )*O + P2
        let mut temp = matrix_mul!(added_p1, V, V, o, O);
        let temp = matrix_add!(temp, p2[i], V, O);

        l[i] = temp;
    }

    let encoded_l = encode_bit_sliced_matrices!(l, V, O, M, false, L_BYTES); // m matrices of size (n−o) × o

    // To follow the refference implementation append O_bytestring at the end
    // Do not add sk_seed to the expanded secret key
    let mut expanded_sk = [0u8 ; ESK_BYTES - SK_SEED_BYTES];

    expanded_sk[..P1_BYTES].copy_from_slice(&p1_bytes);
    expanded_sk[P1_BYTES..P1_BYTES+L_BYTES].copy_from_slice(&encoded_l);
    expanded_sk[P1_BYTES+L_BYTES..].copy_from_slice(&o_bytes);

    return expanded_sk;
}






// Mayo algorithm 7
// Expands a public key from its compact representation
pub fn expand_pk(cpk: [u8 ; CPK_BYTES]) -> [u8 ; EPK_BYTES] {

    // Parse cpk
    let pk_seed_slice = &cpk[0..PK_SEED_BYTES];
    let pk_seed: [u8; PK_SEED_BYTES] = pk_seed_slice
        .try_into()
        .expect("Slice has incorrect length");


    // Expand seed_pk and return
    let mut aes_bytes = [0u8; P1_BYTES + P2_BYTES];
    safe_aes_128_ctr(
        &mut aes_bytes,
        (P1_BYTES + P2_BYTES) as u64,
        &pk_seed,
        PK_SEED_BYTES as u64,
    );

    let mut expanded_pk = [0u8 ; EPK_BYTES];
    let cpk_bytes = &cpk[PK_SEED_BYTES..];
    expanded_pk[..P1_BYTES + P2_BYTES].copy_from_slice(&aes_bytes);
    expanded_pk[P1_BYTES + P2_BYTES..].copy_from_slice(&cpk_bytes);

    return expanded_pk;
}

// MAYO algorithm 8
// Signs a message using an expanded secret key
// MAYO algorithm 8
// Signs a message using an expanded secret key
pub fn sign(compact_secret_key: [u8 ; CSK_BYTES], message: Vec<u8>) -> [u8 ; SIG_BYTES] {

    let mut x = [0u8; K*O]; // Initialize x to zero
    let mut v = [[0u8; V]; K]; // Initialize v to zero

    // Unlike specifcation, sk_seed is NOT included ESK
    let expanded_sk: [u8 ; ESK_BYTES-SK_SEED_BYTES] = expand_sk(compact_secret_key);

    // Decode expanded secret key
    let p1_bytestring = &expanded_sk[..P1_BYTES];
    let l_bytestring = &expanded_sk[P1_BYTES..L_BYTES + P1_BYTES];
    let o_bytestring = &expanded_sk[P1_BYTES + L_BYTES..];

    // Assign matrices with decoded information
    let o = decode_bytestring_matrix_array!(o_bytestring, V, O);
    let p1 = decode_bit_sliced_matrices!(p1_bytestring, V, V, M, true);
    let l = decode_bit_sliced_matrices!(l_bytestring, V, O, M, false);

    // Hash message 
    let mut m_digest = [0u8; DIGEST_BYTES];
    safe_shake256(
        &mut m_digest,
        DIGEST_BYTES as u64,
        &message,
        message.len() as u64,
    );

    // Derive salt
    let mut r = [0u8; R_BYTES]; 
    safe_random_bytes(&mut r, R_BYTES as u64);

    let mut salt_input = [0u8 ; DIGEST_BYTES + R_BYTES + SK_SEED_BYTES];
    salt_input[..DIGEST_BYTES].copy_from_slice(&m_digest);
    salt_input[DIGEST_BYTES..DIGEST_BYTES + R_BYTES].copy_from_slice(&r);
    salt_input[DIGEST_BYTES + R_BYTES..].copy_from_slice(&compact_secret_key);

    let mut salt = [0u8; SALT_BYTES];
    safe_shake256(
        &mut salt,
        SALT_BYTES as u64,
        &salt_input,
        (DIGEST_BYTES + R_BYTES + SK_SEED_BYTES) as u64,
    );

    // Derive t
    let mut t_shake_input = [0u8 ; DIGEST_BYTES + SALT_BYTES];
    t_shake_input[..DIGEST_BYTES].copy_from_slice(&m_digest);
    t_shake_input[DIGEST_BYTES..].copy_from_slice(&salt);

    let mut t_output = [0u8 ; M/2]; // Ceil (M * log_2(q) / 8)
    safe_shake256(
        &mut t_output,
        (M/2) as u64,
        &t_shake_input,
        (DIGEST_BYTES + SALT_BYTES) as u64,
    );

    let t = decode_bytestring_to_array!(&t_output, M);
    

    // Attempt to find a preimage for t
    for ctr in 0..=255 {

        // Derive v_i and r
        let mut v_shake_input = [0u8 ; DIGEST_BYTES + SALT_BYTES + CSK_BYTES + 1];
        v_shake_input[..DIGEST_BYTES].copy_from_slice(&m_digest);
        v_shake_input[DIGEST_BYTES..DIGEST_BYTES + SALT_BYTES].copy_from_slice(&salt);
        v_shake_input[DIGEST_BYTES + SALT_BYTES..DIGEST_BYTES + SALT_BYTES + CSK_BYTES].copy_from_slice(&compact_secret_key);
        v_shake_input[DIGEST_BYTES + SALT_BYTES + CSK_BYTES] = ctr;


        const CEIL: usize = K*O / 2; // Ceil (K*O * log_2(q) / 8) - Notice, all versions does not require ceil
        let mut v_bytestring = [0u8; K * V_BYTES + CEIL];
        safe_shake256(
            &mut v_bytestring,
            (K * V_BYTES + CEIL) as u64,
            &v_shake_input,
            (DIGEST_BYTES + SALT_BYTES + SK_SEED_BYTES + 1) as u64,
        );

        // Derive v_i
        for i in 0..K {
            let v_bytestring_slice = &v_bytestring[i * V_BYTES..(i + 1) * V_BYTES];
            v[i] = decode_bytestring_to_array!(v_bytestring_slice, V)
        }

        // Derive r (Notice r is redefined and have nothing to do with previous r)
        let v_bytestring_remainder = &v_bytestring[K * V_BYTES..];
        let r = decode_bytestring_to_array!(v_bytestring_remainder, K*O); // Remainding part of v_bytestring.

        // Build the linear system Ax = y
        let mut a = [[0u8; K * O]; M + SHIFTS];
        let mut y = [0u8 ; M + SHIFTS];
        y[..M].copy_from_slice(&t);
        let mut ell = 0;
        let mut m_matrices = [[[0u8; O]; M]; K]; // Vector of size m x o of zeroes

        // Build K matrices of size M x O
        for i in 0..K {
            // Set the j-th row of m_i 
            for j in 0..M {
                let res = vector_matrix_mul!(v[i], l[j], V, O); // (1 x (n-o)) * ((n-o) x o) = 1 x o
                m_matrices[i][j] = res; 
            }
        }

        for i in 0..K {

            let mut trans_mult_v = [[0u8 ; V]; M];
            for a in 0..M {
                trans_mult_v[a] = vector_transposed_matrix_mul!(v[i], p1[a], V, V); //  (1 x (n-o)) * ((n-o) x (n-o)) = 1 x (n-o).
            }

            for j in (i..K).rev() {

                let mut u = [0u8; M];
                if i == j {
                    for a in 0..M {

                        u[a] = vector_mul!(trans_mult_v[a], v[i], V);  // (1 x (n-o)) * ((n-o) x (n-o)) * ((n-o)) x 1) = 1 x 1.
                    }
                } else {
                    for a in 0..M {
                        let left_term = vector_mul!(trans_mult_v[a], v[j], V); // (1 x (n-o)) * ((n-o) x 1) = 1 x 1.

                        let trans_mult = vector_transposed_matrix_mul!(v[j], p1[a], V, V); // (1 x (n-o)) * ((n-o) x (n-o)) = 1 x (n-o).
                        let right_term = vector_mul!(trans_mult, v[i], V); // (1 x (n-o)) * ((n-o) x 1) = 1 x 1.

                        u[a] = add(left_term, right_term); 
                    }
                }

                // y = y - u * z^ell - Instead of subtracting with shifted u,
                // we just sub (XOR) with shifted y for easier loop structre
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


        let y = reduce_mod_f(y);
        let a = reduce_a_mod_f(a);
        
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
    let mut signature = [0u8; K * N];


    for i in 0..K {
        let x_idx: [u8 ; O] = x[i * O..(i + 1) * O]
            .try_into()
            .expect("Slice has incorrect length");
        let ox: [u8 ; V] = matrix_vec_mul!(o, x_idx, V, O); // (n−o) × o * o × 1 = (n−o) × 1

        for j in 0..V {
            v[i][j] = add(ox[j], v[i][j]);
        }

        signature[i * N..(i + 1) * N - O].copy_from_slice(&v[i]);
        signature[i*N + V..(i + 1) * N].copy_from_slice(&x_idx);
    }

    let mut sig_con_salt = [0u8 ; SIG_BYTES];
    let signature_encoded = encode_to_bytestring_array!(signature, K*N, SIG_BYTES-SALT_BYTES); // SALT_BYTES is NOT included in the signature

    sig_con_salt[..SIG_BYTES-SALT_BYTES].copy_from_slice(&signature_encoded);
    sig_con_salt[SIG_BYTES-SALT_BYTES..].copy_from_slice(&salt);

    return sig_con_salt;
}





// MAYO algorithm 9
// Verifi the signature of a message using the expanded public key
pub fn verify(expanded_pk: [u8 ; EPK_BYTES], signature: &[u8], message: &Vec<u8>) -> bool {

    // Retrieve the public information from the expanded public key
    let p1_bytestring = &expanded_pk[0..P1_BYTES];
    let p2_bytestring = &expanded_pk[P1_BYTES..P1_BYTES + P2_BYTES];
    let p3_bytestring = &expanded_pk[P1_BYTES + P2_BYTES..];

    // Decode the public information into matrices
    let p1 = decode_bit_sliced_matrices!(p1_bytestring, V, V, M, true);
    let p2 = decode_bit_sliced_matrices!(p2_bytestring, V, O, M, false);
    let p3 = decode_bit_sliced_matrices!(p3_bytestring, O, O, M, true);

    // Decode signature and derive salt
    let salt = &signature[SIG_BYTES - SALT_BYTES..SIG_BYTES];
    let s_bytes = &signature[0..SIG_BYTES - SALT_BYTES];
    let s = decode_bytestring_to_array!(s_bytes, K*N);


    let mut s_matrix = [[0u8; N]; K];
    for i in 0..K {
        s_matrix[i].copy_from_slice(&s[i * N..(i + 1) * N]);
    }


    // Hash message 
    let mut m_digest = [0u8; DIGEST_BYTES];
    safe_shake256(
        &mut m_digest,
        DIGEST_BYTES as u64,
        &message,
        message.len() as u64,
    );


    // Derive t
    let mut t_shake_input =[ 0u8 ; DIGEST_BYTES + SALT_BYTES];
    t_shake_input[..DIGEST_BYTES].copy_from_slice(&m_digest); 
    t_shake_input[DIGEST_BYTES..].copy_from_slice(salt); 


    let mut t_output = [0u8; M/2]; // Ceil (M * log_2(q) / 8)
    safe_shake256(
        &mut t_output,
        (M/2) as u64,
        &t_shake_input,
        (DIGEST_BYTES + SALT_BYTES) as u64,
    );
    let t = decode_bytestring_to_array!(t_output, M);


    // Compute P*(s)
    let mut y = [0u8; M + SHIFTS];
    let mut ell = 0;

    // Construct matrices P*_i of size N x N s.t. (P^1_a P^2_a)
    // for every matrix a ∈ [m]                   (0     P^3_a)
    let big_p = create_big_p_matrices(p1, p2, p3);

    for i in 0..K {

        let mut mul_res_arr = [[0u8 ; N]; M];
        for a in 0..M {
            mul_res_arr[a] = vector_transposed_matrix_mul!(s_matrix[i], big_p[a], N, N); // (1 x n) * (n x n) = 1 x n
        }

        for j in (i..K).rev() {

            let mut u = [0u8; M];
            for a in 0..M {


                if i == j {
                    u[a] = vector_mul!(mul_res_arr[a], s_matrix[i], N); // (1 x n) * (n x 1) = 1 x 1

                } else {
                    let left_term = vector_mul!(mul_res_arr[a], s_matrix[j], N); // (1 x n) * (n x 1) = 1 x 1

                    let mul_res2 = vector_transposed_matrix_mul!(s_matrix[j], big_p[a], N, N); // (1 x n) * (n x n) = 1 x n
                    let right_term = vector_mul!(mul_res2, s_matrix[i], N); // (1 x n) * (n x 1) = 1 x 1

                    u[a] = add(left_term, right_term);
                }
            }

            // y = y - u * z^ell - Instead of subtracting with shifted u,
            // sub (XOR) with shifted y.
            for d in 0..M {
                y[d + ell] ^= u[d];
            }
            ell = ell + 1;
        }
    }

    let y = reduce_mod_f(y);

    // Accept signature if y = t
    return y == t;
} 






// MAYO algorithm 10
// Expand a secret key from its compact representation and sign a message
pub fn api_sign(message: Vec<u8>, csk: [u8 ; CSK_BYTES]) -> Vec<u8> {

    // Create signature based on expanded secret key and message
    let signature = sign(csk, message.clone());

    // Concatenate signature and message
    // Note the message length cannot be known at compile time (Hence vec is used instead of array)
    let mut sign_con_mes = Vec::with_capacity(SIG_BYTES + message.len());
    sign_con_mes.append(&mut signature.to_vec());
    sign_con_mes.append(&mut message.to_vec());

    return sign_con_mes;
}






// MAYO algorithm 11
// Expand a public key from its compact representation and verify a signature
pub fn api_sign_open(sign_con_mes: Vec<u8>, pk: [u8 ; CPK_BYTES]) -> (bool, Vec<u8>) {
    
    // Expand public key
    let expanded_pk = expand_pk(pk);

    // Extract signature and message from input
    let signature = &sign_con_mes[0..SIG_BYTES];
    let mut message = sign_con_mes[SIG_BYTES..].to_vec();

    // Verify the signature based on expanded public key and message
    let result = verify(expanded_pk, signature, &message);

    if result == false {
        message = vec![0u8]; // If the signature is invalid, the message is set to zero
    }
    return (result, message);
}





// Method to apply the upper function to a matrix (as described in the MAYO paper)
pub fn upper(mut matrix: [[u8 ; O] ; O]) -> [[u8 ; O] ; O] {

    // Iterate over everything above the diagonal
    for i in 0..O {
        for j in (i + 1)..O {
            matrix[i][j] ^= matrix[j][i]; // GF(16) addition is the same as XOR
            matrix[j][i] = 0;
        }
    }
    return matrix;
}






// Construct the matrix (P_1 P_2)
//                      (0   P_3)
fn create_big_p_matrices(p1: [[[u8 ; V]; V]; M], p2: [[[u8 ; O]; V]; M], p3: [[[u8 ; O]; O]; M]) -> [[[u8 ; N]; N]; M] {
    let mut result = [[[0u8 ; N]; N]; M];

    for mat in 0..M {
        for i in 0..V {
            result[mat][i][..V].copy_from_slice(&p1[mat][i]);
            result[mat][i][V..].copy_from_slice(&p2[mat][i]);
        }
        for i in V..N {
            result[mat][i][V..].copy_from_slice(&p3[mat][i - V]);
        }
    }
    return result;
}




// Perform reduction of a polynomial with f(z)
pub fn reduce_mod_f(mut polynomial: [u8 ; M + SHIFTS]) -> [u8 ; M] {

    for i in (M..polynomial.len()).rev() {
        for (shift, coef) in F_Z {

            let mul_res = mul(polynomial[i], coef);
            polynomial[i - M + shift] ^= mul_res // Same as add
        }
        polynomial[i] = 0; // set original term to 0 After distributing coefficient
    }

    let mut reduced_polynomial = [0u8 ; M];
    reduced_polynomial.copy_from_slice(&polynomial[..M]); // Truncate the polynomial to M terms (all other entries are zero after reduction)
    
    return reduced_polynomial;
}




// Perform reduction of a matrix's cols with f(z)
pub fn reduce_a_mod_f(mut a: [[u8 ; K*O]; M+SHIFTS]) -> [[u8 ; K*O]; M] {
    
    for col in 0..K * O {
        for row in (M..M + SHIFTS).rev() {
            for (shift, coef) in F_Z {
                let mul_res = mul(a[row][col], coef);
                
                a[row - M + shift][col] ^= mul_res; // Same as add
            }
            a[row][col] = 0; // set original term to 0 After distributing coefficient
        }
    }

    let mut reduced_a = [[0u8; K*O]; M];
    for i in 0..M {
        reduced_a[i].copy_from_slice(&a[i][..]);  // Truncate the polynomial to M terms (all other rows are zero after reduction)
    }

    return reduced_a
}

