use libc::write;

use crate::bitsliced_functionality::{
    decode_bit_sliced_vector, decode_bytestring_to_matrix, decode_bytestring_to_vector, encode_bit_sliced_matrices,
    encode_vector_to_bytestring, decode_bit_sliced_matrices
};
use crate::crypto_primitives::{aes_128_ctr_seed_expansion, safe_randomBytes, shake256, safe_shake256, safe_aes_128_ctr};
use crate::finite_field::{add, mul, sub, matrix_add, matrix_mul, matrix_sub, matrix_vector_mul}; 
use crate::utils::{bytes_to_hex_string, hex_string_to_bytes, print_matrix, transpose_matrix, write_to_file, transpose_vector};
use crate::sample::sample_solution;

use crate::constants::{
    CSK_BYTES, DIGEST_BYTES, EPK_BYTES, ESK_BYTES, F_Z, K, L_BYTES, M, N, O, O_BYTES, P1_BYTES,
    P2_BYTES, P3_BYTES, PK_SEED_BYTES, R_BYTES, SALT_BYTES, SIG_BYTES, SK_SEED_BYTES, V_BYTES, O_BYTES_MAX
};



// Upper(M)_ij = M_ij + M_ji for i < j
pub fn upper(mut matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    
    let n = matrix.len();

    // Iterate over everything above the diagonal
    for i in 0..n {
        for j in (i+1)..n {
            matrix[i][j] ^= matrix[j][i]; // GF(16) addition is the same as XOR
            matrix[j][i] = 0;
        }
    }


    return matrix;
}


// MAYO algorithm 5:
pub fn compact_key_gen(mut keygen_seed: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    // Pick random seed (same length as salt_bytes)
    // let mut sk_seed: Vec<u8> = vec![0u8; SALT_BYTES];
    // OsRng.fill(&mut sk_seed[..]); // Fill cryptographically secure with random bytes

    //TODO make if statement and have some check for testing
    let mut sk_seed: Vec<u8> = vec![0u8; SK_SEED_BYTES];

    // // Derive pk_seed and Oil space from sk_seed
    // let output_len = PK_SEED_BYTES + O_BYTES;
    // let s = shake256(&sk_seed, output_len);

    safe_randomBytes(&mut sk_seed, SK_SEED_BYTES as u64);

    //sk_seed kun første 24

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

    let n_minus_o = N - O;

    // Make Oil space from o_bytes. Only a single is yielded from decode_bit_sliced_matrices in this case
    let o_bytes = s[PK_SEED_BYTES..].to_vec();
    let o = decode_bytestring_to_matrix(n_minus_o, O, o_bytes);


    //Derive P_{i}^(1) and P_{i}^(2) from pk_seed
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
    let p1 = decode_bit_sliced_matrices(n_minus_o, n_minus_o, p1_bytes, true);

    // m p2 matrices are of size (n−o) × o (not upper triangular matrices)
    let p2 = decode_bit_sliced_matrices(n_minus_o, O, p2_bytes, false);

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
        let mut left_term = matrix_mul(&transposed_o, &p1_i);
        left_term = matrix_mul(&left_term, &o);

        // Compute: −O^{T} * P^{(2)}_i
        let right_term: Vec<Vec<u8>> = matrix_mul(&transposed_o, &p2_i);

        // Compute: (−O^{T} * P^{(1)}_i * O ) − (−O^{T} * P^{(2)}_i )
        let mut sub = matrix_sub(&left_term, &right_term);

        p3[i] = upper(sub); // Upper triangular part of the result
    }

    let mut encoded_p3 = encode_bit_sliced_matrices(O, O, p3, true);

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


    // Make Oil space from o_bytes. Only a single is yielded from decode_bit_sliced_matrices in this case
    let mut o_bytes = s[PK_SEED_BYTES..].to_vec(); // From pk_seed_bytes to pk_seed_bytes + o_bytes
    let o = decode_bytestring_to_matrix(n_minus_o, O, o_bytes.clone());



    //Derive P_{i}^(1) and P_{i}^(2) from pk_seed
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
    let p1 = decode_bit_sliced_matrices(n_minus_o, n_minus_o, p1_bytes.clone(), true);
    let p2 = decode_bit_sliced_matrices(n_minus_o, O, p2_bytes, false);

    // Allocate space for L_i in [m]. Size is (n−o) × o per matrix
    let mut l = vec![vec![vec![0u8; O]; n_minus_o]; M];

    // Compute L matrices
    for i in 0..M {
        let p1_i = &p1[i];
        let p2_i = &p2[i];

        let transposed_p1_i = transpose_matrix(p1_i);
        let added_p1 = matrix_add(&p1_i, &transposed_p1_i);

        let left_term = matrix_mul(&added_p1, &o);

        l[i] = matrix_add(&left_term, &p2_i);
    }


    let mut encoded_l = encode_bit_sliced_matrices(n_minus_o, O, l, false);

    // To follow the refference implementation append O_bytestring at the end
    // Do not add sk_seed to the expanded secret key
    let mut expanded_sk: Vec<u8> = Vec::with_capacity(ESK_BYTES-SK_SEED_BYTES);

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

    let mut aes_bytes = aes_128_ctr_seed_expansion(pk_seed, P1_BYTES + P2_BYTES);

    let mut expanded_pk = Vec::with_capacity(EPK_BYTES);
    let mut cpk_bytes = cpk[PK_SEED_BYTES..].to_vec();

    expanded_pk.append(&mut aes_bytes);
    expanded_pk.append(&mut cpk_bytes);

    return expanded_pk;
}

// MAYO algorithm 8
// Signs a message using an expanded secret key
pub fn sign(expanded_sk: Vec<u8>, message: &Vec<u8>) -> Vec<u8> {
    let n_minus_o = N - O; // rows of O matrix
    let mut x: Vec<u8> = vec![0u8; K * O]; // Initialize x to zero
    let mut v: Vec<Vec<u8>> = vec![vec![0u8; n_minus_o]; K]; // Initialize v to zero

    // Decode expanded secret key
    let sk_seed: Vec<u8> = expanded_sk[0..SK_SEED_BYTES].to_vec();
    let o_bytestring = expanded_sk[SK_SEED_BYTES..SK_SEED_BYTES + O_BYTES].to_vec();
    let p1_bytestring =
        expanded_sk[SK_SEED_BYTES + O_BYTES..SK_SEED_BYTES + O_BYTES + P1_BYTES].to_vec();
    let l_bytestring = expanded_sk[SK_SEED_BYTES + O_BYTES + P1_BYTES..ESK_BYTES].to_vec();

    // Assign matrices with decoded information
    let o = decode_bytestring_to_matrix(n_minus_o, O, o_bytestring);
    let p1 = decode_bit_sliced_matrices(n_minus_o, n_minus_o, p1_bytestring, true);
    let l = decode_bit_sliced_matrices(n_minus_o, O, l_bytestring, false);

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
    let t_shake_output_length = if M % 2 == 0 { M / 2 } else { M / 2 + 1 }; // Ceil (M * log_2(q) / 8)
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
        let ceil_exp = if K * O % 2 == 0 {
            K * O / 2
        } else {
            K * O / 2 + 1
        }; // Ceil (K*O * log_2(q) / 8)
        let v_shake_output_length = K * V_BYTES + ceil_exp;
        let v_bytestring = shake256(&v_shake_input, v_shake_output_length);

        // Derive v_i
        for i in 0..K {
            let v_bytestring_slice = v_bytestring[i * V_BYTES..(i + 1) * V_BYTES].to_vec();
            v[i] = decode_bytestring_to_vector(n_minus_o, v_bytestring_slice)
        }

        // Derive r (Notice r is redefined and have nothing to do with previous r)
        let v_bytestring_remainder = v_bytestring[K * V_BYTES..].to_vec();
        let r = decode_bytestring_to_vector(K * O, v_bytestring_remainder); // Remainding part of v_bytestring.

        // Build the linear system Ax = y
        let mut a: Vec<Vec<u8>> = vec![vec![0u8; K * O]; 2 * M]; // Make matrix of size m x k*o
        let mut y = Vec::with_capacity(M * 2);
        y.extend(t.clone());
        y.extend(vec![0u8; M]);
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

                        // Size (1 x (n-o)) * ((n-o) x (n-o)) * ((n-o)) x 1) gives size 1 x 1.
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

                let y_sub_u: Vec<u8> = y
                    .iter()
                    .zip(u.iter())
                    .map(|(y_idx, u_idx)| sub(*y_idx, *u_idx))
                    .collect();
                for d in 0..M {
                    y[d + ell] = y_sub_u[d];
                }

                // Calculate A
                for d in 0..M {
                    for a_entry in 0..O {
                        a[d + ell][a_entry] ^= m_matrices[j][d][a_entry];
                    }
                }
                if i != j {
                    for d in 0..M {
                        for a_entry in 0..O {
                            a[d + ell][a_entry] ^= m_matrices[i][d][a_entry];
                        }
                    }
                }
                ell += 1;

                // let e_raised_to_ell = vec![0u8; F_Z.len()]; // [0, 0, 0, 0, 0]
                // for power in 0..ell {
                //     for poly_idx in 0..F_Z.len() {
                //         e_raised_to_ell[poly_idx] ^= mul(F_Z[poly_idx], ell);

                //     }
                // }

                //
            }
        }
        reduce_y_mod_f(&mut y);
        reduce_a_mod_f(&mut a);

        // Try to solve the linear system Ax = y
        x = match sample_solution(a, y) {
            Ok(x) => x, // If Ok
            Err(e) => {
                continue; // If Err (no solution found), continue to the next iteration of the loop
            }
        };
        break; // If Ok (solution found), break the loop
    } // ctr loop ends

    // return x;

    // Finish and output signature
    let mut signature = vec![0u8; K * N];

    for i in 0..K {
        let mut x_idx = x[i * O..(i + 1) * O].to_vec();
        let ox: Vec<u8> = matrix_vector_mul(&o, &x_idx);
        let v_i = v[i].clone();
        let mut vi_plus_ox: Vec<u8> = ox
            .iter()
            .zip(v_i.iter())
            .map(|(ox_idx, v_i_idx)| ox_idx ^ v_i_idx)
            .collect();

        signature.append(&mut vi_plus_ox);
        signature.append(&mut x_idx);
    }

    let mut sign_con_salt = Vec::new();
    let signature_encoded = encode_vector_to_bytestring(signature);
    sign_con_salt.extend(signature_encoded);
    sign_con_salt.extend(salt);
    return sign_con_salt;
}






// MAYO algorithm 9
// Verifies the signature of a message using an expanded public key
pub fn verify(expanded_pk: Vec<u8>, signature: Vec<u8>, message: &Vec<u8>) -> bool {
    let n_minus_o = N - O; // rows of O matrix

    // retrieves the public information from the expanded public key
    let p1_bytestring = expanded_pk[0..P1_BYTES].to_vec();
    let p2_bytestring = expanded_pk[P1_BYTES..P1_BYTES + P2_BYTES].to_vec();
    let p3_bytestring = expanded_pk[P1_BYTES + P2_BYTES..].to_vec();

    // decodes the public information into matrices
    let mut p1 = decode_bit_sliced_matrices(n_minus_o, n_minus_o, p1_bytestring, true);
    let mut p2 = decode_bit_sliced_matrices(n_minus_o, O, p2_bytestring, false);
    let mut p3 = decode_bit_sliced_matrices(O, O, p3_bytestring, true);

    // decode signature and derive salt
    let n_times_k = if N * K % 2 == 0 {
        N * K / 2
    } else {
        N * K / 2 + 1
    }; // Ceil (N*K/2)
    let salt = signature[n_times_k..n_times_k + SALT_BYTES].to_vec();
    let s = decode_bytestring_to_vector(K * N, signature);
    let mut s_matrix = vec![vec![0u8; N]; s.len()];
    for i in 0..K {
        s_matrix[i] = s[i * N..(i + 1) * N].to_vec();
    }

    // derive and decode t
    let m_digest = shake256(&message, DIGEST_BYTES);
    let mut t_shake_input = Vec::new();
    t_shake_input.extend(&m_digest);
    t_shake_input.extend(&salt);
    let t_shake_output_length = if M % 2 == 0 { M / 2 } else { M / 2 + 1 }; // Ceil (M * log_2(q) / 8)
    let t_input: Vec<u8> = shake256(&t_shake_input, t_shake_output_length);
    let t: Vec<u8> = decode_bit_sliced_vector(t_input);

    // Compute P*(s)
    let y: Vec<u8> = vec![0u8; M];
    let mut ell: u8 = 0;

    // Construct the M matrices of size N x N s.t. (P^1_a P^2_a)
    // for every matrix a ∈ [M]                    (0     P^3_a)
    let big_p = create_large_matrices(p1, p2, p3);

    for m in 0..M {
        println!("");
        println!("NEW");

        print_matrix(big_p[m].clone());
    }

    for i in 0..K {
        let s_i_trans = transpose_vector(&s_matrix[i]);

        for j in (i..K).rev() {
            let mut u = vec![0u8; M];
            let s_j_trans = transpose_vector(&s_matrix[j]);

            for a in 0..M {
                let s_i_trans_big_p = matrix_mul(&s_i_trans, &big_p[a]);

                if i == j {
                    u[a] = matrix_vector_mul(&s_i_trans_big_p, &s_matrix[i])[0];
                } else {
                    let left_term = matrix_vector_mul(&s_i_trans_big_p, &s_matrix[j])[0];

                    let s_j_trans_big_p = matrix_mul(&s_j_trans, &big_p[a]);
                    let right_term = matrix_vector_mul(&s_j_trans_big_p, &s_matrix[i])[0];

                    u[a] = add(left_term, right_term);
                }
            }
            // Y UPDATE HERE

            ell = ell + 1;

            //println!("U: {:?}", u);
        }
    }

    // Accept signature if y = t
    return y == t;
}






// Construct the matrix (P_1 P_2)
//                      (0   P_3)
fn create_large_matrices(
    mut p1: Vec<Vec<Vec<u8>>>,
    mut p2: Vec<Vec<Vec<u8>>>,
    mut p3: Vec<Vec<Vec<u8>>>,
) -> Vec<Vec<Vec<u8>>> {
    let mut result: Vec<Vec<Vec<u8>>> = Vec::with_capacity(M);

    for mat in 0..M {
        let mut rows = Vec::with_capacity(N);

        let mut zero_rows = vec![vec![0u8; N - O]; O]; // O rows of zeroes of len N-O.

        for i in 0..(N - O) {
            let new_vec = Vec::new();
            rows.push(new_vec);
            rows[i].append(&mut p1[mat][i]);
            rows[i].append(&mut p2[mat][i]);
        }

        for i in (N - O)..N {
            let new_vec = Vec::new();
            rows.push(new_vec);
            rows[i].append(&mut zero_rows[i - (N - O)]);
            rows[i].append(&mut p3[mat][i - (N - O)]);
        }

        result.push(rows);
    }

    return result;
}

fn reduce_y_mod_f(y: &mut Vec<u8>) {
    for i in (M..M + K * (K + 1) / 2 - 1).rev() {
        for j in 0..F_Z.len() {
            if i >= M + j {
                y[i - M + j] ^= mul(y[i], F_Z[j]);
            }
        }
        y[i] = 0;
    }
}

fn reduce_a_mod_f(a: &mut Vec<Vec<u8>>) {
    for i in (M..M + K * (K + 1) / 2 - 1).rev() {
        for k in 0..O * K {
            for j in 0..F_Z.len() {
                if i >= M + j {
                    a[i - M + j][k] ^= mul(a[i - M + j][k], F_Z[j]);
                }
            }
            a[i][k] = 0;
        }
    }
}



// MAYO algorithm 10
// Expands a secret key from its compact representation and signs a message input
pub fn api_sign(mut message: Vec<u8>, sk: Vec<u8>) -> Vec<u8> {
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


// MAYO algorithm 11
// Expands a public key from its compact representation and verifies a signature
pub fn api_sign_open(sign_con_mes: Vec<u8>, pk: Vec<u8>) -> (bool, Vec<u8>) {
    //Expands the public key
    let expanded_pk = expand_pk(pk);

    //Extracts the signature and the message from the input
    let signature: Vec<u8> = sign_con_mes[0..SIG_BYTES].to_vec();
    let mut message = sign_con_mes[SIG_BYTES..].to_vec();

    //Verifies the signature based on expanded public key and message
    let result = verify(expanded_pk, signature, &message);

    //returns result and message
    if result == false {
        //dummy / false message if the signature is not valid
        message = vec![0u8];
    }

    return (result, message);
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

                    if (j < O) {
                        p2[m][i][j] = rng.gen_range(00..=15);
                    }

                    if (i < O && j < O) {
                        p3[m][i][j] = rng.gen_range(00..=15);
                    }
                }
            }
        }

        let big_matrices = create_large_matrices(p1.clone(), p2.clone(), p3.clone());

        for m in 0..M {
            println!("NEW matrix");
            print_matrix(big_matrices[m].clone());
        }

        let mut succeded: bool = true;

        for m in 0..M {
            for i in 0..N {
                for j in 0..N {
                    if (i < N - O && j < N - O) {
                        if (big_matrices[m][i][j] != p1[m][i][j]) {
                            succeded = false;
                        }
                    }
                    if (i < N - O && j < O) {
                        if (big_matrices[m][i][j + (N - O)] != p2[m][i][j]) {
                            succeded = false;
                        }
                    }

                    // Should be zero
                    if (i < O && j < O) {
                        if (big_matrices[m][i + (N - O)][j] != 0) {
                            succeded = false;
                        }
                    }

                    if (i < O && j < O) {
                        if (big_matrices[m][i + (N - O)][j + (N - O)] != p3[m][i][j]) {
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
