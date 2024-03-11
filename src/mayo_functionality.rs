use std::vec;

use crate::bitsliced_functionality::{
    decode_bit_sliced_matrices, decode_bytestring_to_vector, decode_o_bytestring_to_matrix_array, decode_p1_bit_sliced_matrices_array, decode_p2_bit_sliced_matrices_array, decode_r_bytestring_to_array, decode_t_bytestring_to_array, decode_v_bytestring_to_array, encode_l_bit_sliced_matrices_array, encode_p3_bit_sliced_matrices_array, encode_signature_to_bytestring
};
use crate::crypto_primitives::{safe_aes_128_ctr, safe_randomBytes, safe_shake256};
use crate::finite_field::{add,  matrix_add_array, matrix_mul, matrix_mul_array_p2, matrix_mul_o_p1, matrix_mul_v_l, matrix_mul_v_p1, matrix_v_add_array, matrix_vector_mul, mul, p1_matrix_v_mul, sub, o_matrix_x_idx_mul};
use crate::sample::sample_solution;
use crate::utils::{bytes_to_hex_string, transpose_matrix, transpose_o_matrix_array, transpose_p1_matrix_array, transpose_v_array, transpose_vector};

use crate::constants::{
    CPK_BYTES, CSK_BYTES, DIGEST_BYTES, EPK_BYTES, ESK_BYTES, F_Z, K, L_BYTES, M, N, V, O, O_BYTES, P1_BYTES, P2_BYTES, P3_BYTES, PK_SEED_BYTES, R_BYTES, SALT_BYTES, SIG_BYTES, SK_SEED_BYTES, V_BYTES, SHIFTS
};

// Upper(M)_ij = M_ij + M_ji for i < j
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

pub fn upper_array_p3(mut matrix: [[u8 ; O] ; O]) -> [[u8 ; O] ; O] {
    let n = O;

    // Iterate over everything above the diagonal
    for i in 0..n {
        for j in (i + 1)..n {
            matrix[i][j] ^= matrix[j][i]; // GF(16) addition is the same as XOR
            matrix[j][i] = 0;
        }
    }

    return matrix;
}

// MAYO algorithm 5:
pub fn compact_key_gen(mut keygen_seed: Vec<u8>) -> (Vec<u8>,Vec<u8>) {
    // Pick random seed (same length as salt_bytes)
    let mut sk_seed = [0u8; SK_SEED_BYTES];

    // // Derive pk_seed and Oil space from sk_seed
    safe_randomBytes(&mut sk_seed, SK_SEED_BYTES as u64);

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
    let o = decode_o_bytestring_to_matrix_array(o_bytes);

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

    // m p1 matrices are of size (n−o) × (n−o)
    let p1 = decode_p1_bit_sliced_matrices_array(p1_bytes, true);


    // m p2 matrices are of size (n−o) × o (not upper triangular matrices)
    let p2 = decode_p2_bit_sliced_matrices_array(p2_bytes, false);


    // Allocate space for P_{i}^(3). Size is o × o
    let mut p3 = [[[0u8; O]; O]; M];

    // Compute P_{i}^(3) as (−O^{T} * P^{(1)}_i * O ) − (−O^{T} * P^{(2)}_i )
    // Notice, negation is omimtted as GF(16) negation of an element is the same as the element itself.
    for i in 0..M {
        // transpose (Negation omitted as GF(16) negation of an element is the same as the element itself)
        let transposed_o = transpose_o_matrix_array(o);

        let p1_i = &p1[i];
        let p2_i = &p2[i];

        // P3 = O^t * (P1*O + P2) 
        // Compute: P1*O + P2
        let p1_times_o = matrix_mul_o_p1(*p1_i, o);
        let mult_add_p2 = matrix_add_array(p1_times_o, *p2_i);


        p3[i] = upper_array_p3(matrix_mul_array_p2(transposed_o, mult_add_p2)); // Upper triangular part of the result
    }

    let encoded_p3: [u8 ; P3_BYTES] = encode_p3_bit_sliced_matrices_array(p3, true);


    // Public and secret keys
    let mut cpk: [u8; PK_SEED_BYTES + P3_BYTES] = [0u8 ; PK_SEED_BYTES + P3_BYTES]; // contains pk_seed and encoded_p3
    let csk: [u8 ; CSK_BYTES] = sk_seed;

    cpk[..PK_SEED_BYTES].copy_from_slice(&pk_seed);

    cpk[PK_SEED_BYTES..].copy_from_slice(&encoded_p3);


    return (cpk.to_vec(), csk.to_vec());
}

// MAYO algorithm 6.
// Expands a secret key from its compact representation
pub fn expand_sk(csk: &Vec<u8>) -> Vec<u8> {
    let sk_seed: &Vec<u8> = csk;

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
    let o = decode_o_bytestring_to_matrix_array(o_bytes);


    //Derive P_{i}^(1) and P_{i}^(2) from pk_seed
    let mut p: Vec<u8> = vec![0u8; P1_BYTES + P2_BYTES];
    safe_aes_128_ctr(
        &mut p,
        (P1_BYTES + P2_BYTES) as u64,
        &pk_seed,
        PK_SEED_BYTES as u64,
    );
    let p1_bytes = &p[0..P1_BYTES];
    let p2_bytes = &p[P1_BYTES..];

    // m p1 matrices are of size (n−o) × (n−o)
    let p1 = decode_p1_bit_sliced_matrices_array(p1_bytes, true);

    // m p2 matrices are of size (n−o) × o (not upper triangular matrices)
    let p2 = decode_p2_bit_sliced_matrices_array(p2_bytes, false);


    // Allocate space for L_i in [m]. Size is (n−o) × o per matrix
    let mut l = [[[0u8; O]; V]; M];

    // Compute L matrices
    for i in 0..M {
        let p1_i = &p1[i];
        let p2_i = &p2[i];

        let transposed_p1_i = transpose_p1_matrix_array(*p1_i);
        let added_p1 = matrix_v_add_array(*p1_i, transposed_p1_i);

        let left_term = matrix_mul_o_p1(added_p1, o);

        l[i] = matrix_add_array(left_term, *p2_i);
    }

    let encoded_l = encode_l_bit_sliced_matrices_array(l, false);

    // To follow the refference implementation append O_bytestring at the end
    // Do not add sk_seed to the expanded secret key
    let mut expanded_sk = [0u8 ; ESK_BYTES - SK_SEED_BYTES];

    expanded_sk[..P1_BYTES].copy_from_slice(&p1_bytes);
    expanded_sk[P1_BYTES..P1_BYTES+L_BYTES].copy_from_slice(&encoded_l);
    expanded_sk[P1_BYTES+L_BYTES..].copy_from_slice(&o_bytes);

    return expanded_sk.to_vec();
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

    let mut expanded_pk = [0u8 ; EPK_BYTES];
    let cpk_bytes = &cpk[PK_SEED_BYTES..];
    expanded_pk[..P1_BYTES + P2_BYTES].copy_from_slice(&aes_bytes);
    expanded_pk[P1_BYTES + P2_BYTES..].copy_from_slice(&cpk_bytes);

    return expanded_pk.to_vec();
}

// MAYO algorithm 8
// Signs a message using an expanded secret key
pub fn sign(compact_secret_key: &Vec<u8>, message: &Vec<u8>) -> Vec<u8> {

    let mut x = [0u8; K * O]; // Initialize x to zero
    let mut v = [[0u8; V]; K]; // Initialize v to zero

    let mut expanded_sk = expand_sk(&compact_secret_key);

    // Decode expanded secret key
    let p1_bytestring = &expanded_sk[..P1_BYTES];
    let l_bytestring = &expanded_sk[P1_BYTES..L_BYTES + P1_BYTES];
    let o_bytestring = &expanded_sk[P1_BYTES + L_BYTES..];

    // Assign matrices with decoded information
    let o = decode_o_bytestring_to_matrix_array(o_bytestring);
    let p1 = decode_p1_bit_sliced_matrices_array(p1_bytestring, true);
    let l = decode_p2_bit_sliced_matrices_array(l_bytestring, false);

    // Hash message and derive salt
    let mut m_digest = [0u8; DIGEST_BYTES];
    safe_shake256(
        &mut m_digest,
        DIGEST_BYTES as u64,
        &message,
        message.len() as u64,
    );

    let mut r = [0u8; R_BYTES]; 
    safe_randomBytes(&mut r, R_BYTES as u64);


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

    let t = decode_t_bytestring_to_array( &t_output);
    

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
            v[i] = decode_v_bytestring_to_array(v_bytestring_slice)
        }

        // Derive r (Notice r is redefined and have nothing to do with previous r)
        let v_bytestring_remainder = &v_bytestring[K * V_BYTES..];
        let r = decode_r_bytestring_to_array(v_bytestring_remainder); // Remainding part of v_bytestring.

        // Build the linear system Ax = y
        let mut a = [[0u8; K * O]; M + SHIFTS];
        let mut y = [0u8 ; M + SHIFTS];
        y[..M].copy_from_slice(&t);
        let mut ell = 0;
        let mut m_matrices = [[[0u8; O]; M]; K]; // Vector of size m x o of zeroes

        // Build K matrices of size M x O
        for i in 0..K {
            let v_i_transpose = transpose_v_array(v[i]);

            for j in 0..M {
                let res = matrix_mul_v_l(v_i_transpose, l[j]);
                m_matrices[i][j] = res.clone(); // Set the j-th row of m_i (unpack (o x 1) to row vector of size o)
            }
        }

        for i in 0..K {
            for j in (i..K).rev() {
                let v_i_transpose = transpose_v_array(v[i]);
                let mut u = [0u8; M];

                if i == j {
                    for a in 0..M {
                        let trans_mult = matrix_mul_v_p1(v_i_transpose, p1[a]);

                        // Size (1 x (n-o)) * ((n-o) x (n-o)) * ((n-o)) x 1) gives size 1 x 1.
                        u[a] = p1_matrix_v_mul(trans_mult, v[i]);
                    }
                } else {
                    for a in 0..M {
                        let trans_mult = matrix_mul_v_p1(v_i_transpose, p1[a]);
                        let left_term = p1_matrix_v_mul(trans_mult, v[j]);

                        let v_j_transpose = transpose_v_array(v[j]);
                        let trans_mult = matrix_mul_v_p1(v_j_transpose, p1[a]);
                        let right_term = p1_matrix_v_mul(trans_mult, v[i]);

                        u[a] = add(left_term, right_term);
                    }
                }

                // y = y - u * z^ell - Instead of subtracting with shifted u,
                // we just sub with shifted y for easier loop structre since
                // XOR (sub) and shift are both linear operations.
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
            Err(e) => {
                continue; // If Err (no solution found), continue to the next iteration of the loop
            }
        };
        break; // If Ok, break the loop
    } // ctr loop ends

    // Finish and output signature
    let mut signature = [0u8; K * N];


    for i in 0..K {
        let x_idx = &x[i * O..(i + 1) * O];
        let ox = o_matrix_x_idx_mul(o, x_idx);
        let v_i = v[i];
        let mut vi_plus_ox: [u8 ; V] = [0u8 ; V];

        for j in 0..V {
            vi_plus_ox[j] = add(ox[j], v_i[j]);
        }

        // OPTIMIZE THIS!
        let mut temp = [0u8; N];
        temp[..V].copy_from_slice(&vi_plus_ox);
        temp[V..].copy_from_slice(&x_idx);

        signature[i * N..(i + 1) * N].copy_from_slice(&temp);
    }

    let mut sig_con_salt = [0u8 ; SIG_BYTES];
    let signature_encoded = encode_signature_to_bytestring(signature);

    sig_con_salt[..SIG_BYTES-SALT_BYTES].copy_from_slice(&signature_encoded);
    sig_con_salt[SIG_BYTES-SALT_BYTES..].copy_from_slice(&salt);

    return sig_con_salt.to_vec();
}

// MAYO algorithm 9
// Verifies the signature of a message using an expanded public key
pub fn verify(expanded_pk: Vec<u8>, signature: Vec<u8>, message: &Vec<u8>) -> bool {


    // retrieves the public information from the expanded public key
    let p1_bytestring = expanded_pk[0..P1_BYTES].to_vec();
    let p2_bytestring = expanded_pk[P1_BYTES..P1_BYTES + P2_BYTES].to_vec();
    let p3_bytestring = expanded_pk[P1_BYTES + P2_BYTES..].to_vec();

    // decodes the public information into matrices
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

    // Hash message and derive salt
    let mut m_digest: Vec<u8> = vec![0u8; DIGEST_BYTES];
    safe_shake256(
        &mut m_digest,
        DIGEST_BYTES as u64,
        &message,
        message.len() as u64,
    );

    let mut t_shake_input = Vec::with_capacity(DIGEST_BYTES + SALT_BYTES);
    t_shake_input.extend(&m_digest); // Extend to prevent emptying original
    t_shake_input.extend(&salt); // Extend to prevent emptying original
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
    
    let mut y = vec![0u8; M + SHIFTS];
    let mut ell = 0;

    // Construct the M matrices of size N x N s.t. (P^1_a P^2_a)
    // for every matrix a ∈ [M]                    (0     P^3_a)
    let big_p = create_large_matrices(p1, p2, p3);

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

            for d in 0..M {
                y[d + ell] ^= u[d];
            }

            ell = ell + 1;
        }
    }

    y = reduce_mod_f_vec(y);

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

        let mut zero_rows = vec![vec![0u8; V]; O]; // O rows of zeroes of len N-O.

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


pub fn reduce_mod_f_vec(mut polynomial: Vec<u8>) -> Vec<u8> {
    // Perform the reduction of with f(z)
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

pub fn reduce_mod_f(mut polynomial: [u8 ; M + SHIFTS]) -> [u8 ; M] {
    // Perform the reduction of with f(z)

    for i in (M..polynomial.len()).rev() {
        for (shift, coef) in F_Z {
            let mul_res = mul(polynomial[i], coef);
            polynomial[i - M + shift] = sub(polynomial[i - M + shift], mul_res);
        }
        polynomial[i] = 0; // set original term to 0 After distributing coefficient
    }

    let mut reduced_polynomial = [0u8 ; M];
    reduced_polynomial.copy_from_slice(&polynomial[..M]);
    

    return reduced_polynomial;
}

pub fn reduce_a_mod_f(mut a: [[u8 ; K*O]; M+SHIFTS]) -> [[u8 ; K*O]; M] {
    for col in 0..K * O {
        for row in (M..M + SHIFTS).rev() {
            for (shift, coef) in F_Z {
                let mul_res = mul(a[row][col], coef);

                a[row - M + shift][col] = sub(a[row - M + shift][col], mul_res);
            }
            a[row][col] = 0;
        }
    }

    let mut reduced_a = [[0u8; K*O]; M];
    for i in 0..M {
        reduced_a[i].copy_from_slice(&a[i][..]);
    }

    return reduced_a
}

// MAYO algorithm 10
// Expands a secret key from its compact representation and signs a message input
pub fn api_sign(mut message: Vec<u8>, csk: Vec<u8>) -> Vec<u8> {
    //Expands the secret key

    //creates the signature based on expanded secret key and message
    let mut signature = sign(&csk, &message);

    //concatenates the signature and the message
    let mut sign_con_mes = Vec::with_capacity(SIG_BYTES + message.len());
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

                    if j < O {
                        p2[m][i][j] = rng.gen_range(00..=15);
                    }

                    if i < O && j < O {
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
