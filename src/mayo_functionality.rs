use std::vec;

use crate::crypto_primitives::{safe_aes_128_ctr, safe_random_bytes, safe_shake256}; 
use crate::finite_field::{add, mul};
use crate::sample::sample_solution;
use crate::bitsliced_arithmetic::{calculate_st_p, mul_add_bitsliced_m_vec, p1_add_p1t};
use crate::constants::{
    CSK_BYTES, DIGEST_BYTES, F_Z, K, L_BYTES, M, N, V, O, O_BYTES, P1_BYTES, P2_BYTES, P3_BYTES,
    PK_SEED_BYTES, R_BYTES, SALT_BYTES, SIG_BYTES, SK_SEED_BYTES, V_BYTES, SHIFTS
};

use crate::{
    bitsliced_mat_mul_mat_add, decode_bit_sliced_array, decode_bytestring_matrix_array, decode_bytestring_to_array, encode_to_bytestring_array,
    mat_mul_bitsliced_mat_add, matrix_vec_mul, transposed_mat_mul_bitsliced_mat_add, upper, bitsliced_mat_mul_transposed_mat_add, vec_add
};



const U32_PER_IDX: usize = M / 4 / 2;

pub struct ExpandedSecretKey {
    p1: [u32 ; P1_BYTES/4],
    l:  [u32 ; L_BYTES/4],
    o:  [u8 ; O_BYTES]
}

pub struct ExpandedPublicKey {
    p1: [u32 ; P1_BYTES/4],
    p2: [u32 ; P2_BYTES/4],
    p3: [u32 ; P3_BYTES/4],
}


pub struct CompactPublicKey {
    pub seed: [u8 ; 16],
    pub p3: [u32 ; P3_BYTES/4], 
}



// MAYO algorithm 5:
pub fn compact_key_gen() -> (CompactPublicKey, [u8 ; CSK_BYTES]) {
    
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
    let pk_seed: [u8; PK_SEED_BYTES] = pk_seed_slice.try_into().expect("Slice has incorrect length");

    // Make Oil space from o_bytes. Only a single is yielded from decode_bit_sliced_matrices in this case
    let o_bytes = &s[PK_SEED_BYTES..PK_SEED_BYTES+O_BYTES];
    let o = decode_bytestring_matrix_array!(o_bytes, V, O);

    // Derive P1_i and P2_i from pk_seed
    let mut p = [0u32; (P1_BYTES + P2_BYTES)/4];
    safe_aes_128_ctr(
        &mut p,
        (P1_BYTES + P2_BYTES) as u64,
        &pk_seed,
    );
    
    let (p1, mut p2) = p.split_at_mut(P1_BYTES/4);


    
    // m p1 matrices of size (n−o) × (n−o)
    // m p2 matrices of size (n−o) × o (not upper triangular matrices)
    // m P3 matrices of size is o × o


    // Compute P3 = (−O^T * P1 * O ) − (−O^T * P2) as P3 = O^t * (P1*O + P2)
    // Compute (P1*O + P2) stored in p2
    bitsliced_mat_mul_mat_add!(&p1, o, &mut p2, V, V, O, true); // upper_triangular = true
    

    // Compute P3 = O^t * (P1*O + P2) stored in p3
    let mut p3 = [0u32 ; O*O*M/8]; // m matrices of size o × o ( divide by 8 from bytes to u32 and 2 nibbles per byte)

    transposed_mat_mul_bitsliced_mat_add!(o, &p2, &mut p3, V, O, O);

    // Compute upper of p3
    let mut p3_upper = [0u32 ; P3_BYTES/4];
    upper!(&mut p3, &mut p3_upper, V, O); 


    // Public and secret keys
    let cpk = CompactPublicKey {
        seed: pk_seed,
        p3: p3_upper
    }; // contains pk_seed and encoded_p3

    let csk: [u8 ; CSK_BYTES] = sk_seed;

    return (cpk, csk);
}





// MAYO algorithm 6.
// Expands a secret key from its compact representation
pub fn expand_sk(csk: [u8 ; CSK_BYTES]) ->  ExpandedSecretKey{
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
    let pk_seed: [u8; PK_SEED_BYTES] = pk_seed_slice.try_into().expect("Slice has incorrect length");

    // Make Oil space from o_bytes
    let o_bytes = &s[PK_SEED_BYTES..PK_SEED_BYTES+O_BYTES];
    let o = decode_bytestring_matrix_array!(o_bytes, V, O);


    // Derive P1_i and P2_i from pk_seed
    let mut p = [0u32; (P1_BYTES + P2_BYTES)/4];
    safe_aes_128_ctr(
        &mut p,
        (P1_BYTES + P2_BYTES) as u64,
        &pk_seed,
    );

    let (p1, p2) = p.split_at_mut(P1_BYTES/4);

    
    // Compute L = (P1 + P1^t)*O + P2 

    // P1 + P1^t
    let mut p1_p1t_added = [0u32 ; V*V*M/8];
    p1_add_p1t(&p1, &mut p1_p1t_added);

    // Compute (P1 + P1^t)*O + P2 stored in p2
    bitsliced_mat_mul_mat_add!(p1_p1t_added, o, p2, V, V, O, false); // upper_triangular = false



    // To follow the refference implementation append O_bytestring at the end
    // Do not add sk_seed to the expanded secret key

    return ExpandedSecretKey {
        p1: p1.try_into().expect("Slice has incorrect length"),
        l: p2.try_into().expect("Slice has incorrect length"),
        o: o_bytes.try_into().expect("Slice has incorrect length"),
    }
}






// Mayo algorithm 7
// Expands a public key from its compact representation
pub fn expand_pk(cpk: CompactPublicKey) -> ExpandedPublicKey {

    // Expand seed_pk and return
    let mut aes_output = [0u32; (P1_BYTES + P2_BYTES)/4];
    safe_aes_128_ctr(
        &mut aes_output,
        (P1_BYTES + P2_BYTES) as u64,
        &cpk.seed,
    );

    let (p1_slice, p2_slice) = aes_output.split_at(P1_BYTES/4);

    return ExpandedPublicKey{ 
        p1: p1_slice.try_into().expect("Slice has incorrect length"),
        p2: p2_slice.try_into().expect("Slice has incorrect length"),
        p3: cpk.p3
    };
}



// MAYO algorithm 8
// Signs a message using an expanded secret key
pub fn sign(compact_secret_key: [u8 ; CSK_BYTES], message: &Vec<u8>) -> [u8 ; SIG_BYTES] {

    let mut x = [0u8; K*O]; // Initialize x to zero
    let mut v = [[0u8; V]; K]; // Initialize v to zero

    // Unlike specifcation, sk_seed is NOT included ESK
    let expanded_sk: ExpandedSecretKey = expand_sk(compact_secret_key);

    // Decode expanded secret key
    let p1: [u32; P1_BYTES/4] = expanded_sk.p1;
    let l: [u32; L_BYTES/4] = expanded_sk.l;
    let o_bytestring: [u8; O_BYTES] = expanded_sk.o;

    let o = decode_bytestring_matrix_array!(o_bytestring, V, O);

    // Hash message 
    let mut m_digest = [0u8; DIGEST_BYTES];
    safe_shake256(
        &mut m_digest,
        DIGEST_BYTES as u64,
        message,
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
    
    // Derive v_i and r
    let mut v_shake_input = [0u8 ; DIGEST_BYTES + SALT_BYTES + CSK_BYTES + 1];
    v_shake_input[..DIGEST_BYTES].copy_from_slice(&m_digest);
    v_shake_input[DIGEST_BYTES..DIGEST_BYTES + SALT_BYTES].copy_from_slice(&salt);
    v_shake_input[DIGEST_BYTES + SALT_BYTES..DIGEST_BYTES + SALT_BYTES + CSK_BYTES].copy_from_slice(&compact_secret_key);
        
    // Attempt to find a preimage for t
    for ctr in 0..=255 {

        // Derive v_i and r
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


        // Build K matrices of size M x O
        let mut m_matrices_array = [0u32 ; K*O*M / 8];
        mat_mul_bitsliced_mat_add!(v, l, &mut m_matrices_array, K, V, O); 

        let mut m_matrices = [[[0u8; O]; M]; K]; 
        
        // Assign the indexes of m matrices to columns of m
        for i in 0..K {
            for j in 0..O {

                let curr_idx = (M/8) * (i * O + j);
                let encoded_m = &m_matrices_array[curr_idx..curr_idx+U32_PER_IDX];
                
                let decoded_m_row = decode_bit_sliced_array!(encoded_m);
                for col in 0..M {
                    m_matrices[i][col][j] = decoded_m_row[col];
                }
            }
        }

        // v^t * P1
        let mut vt_p1 = [0u32 ; V*K*M/ 8]; 
        bitsliced_mat_mul_transposed_mat_add!(p1, v, &mut vt_p1, V, V, K, 0, true);

        // v^t * P1 * v
        let mut vt_p1_v = [0u32 ; K*K*M / 8]; 
        mat_mul_bitsliced_mat_add!(v, vt_p1, &mut vt_p1_v, K, V, K);


        const SIZE: usize = K * (K + 1) / 2; // Size of upper triangular part of matrix of size K x K
        let mut upper_vt_p1_v = [0u32 ; SIZE*M/8];
        upper!(vt_p1_v, &mut upper_vt_p1_v, K, K);


        for i in 0..K {
            for j in (i..K).rev() {

                // Calculate position of in upper triangular part of matrix
                let pos = i * K + j - (i * (i + 1) / 2);
                let encoded_u = &upper_vt_p1_v[pos * U32_PER_IDX .. (pos * U32_PER_IDX) + U32_PER_IDX];


                let u = decode_bit_sliced_array!(encoded_u);

                
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
        let x_idx: [u8 ; O] = x[i * O..(i + 1) * O].try_into().expect("Slice has incorrect length");
        let ox: [u8 ; V] = matrix_vec_mul!(o, x_idx, V, O); // (n−o) × o * o × 1 = (n−o) × 1

        vec_add!(v[i], ox, V);

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
pub fn verify(expanded_pk: ExpandedPublicKey, signature: &[u8], message: &[u8]) -> bool {

    // Retrieve the public information from the expanded public key
    let p1 = expanded_pk.p1;
    let p2 = expanded_pk.p2;
    let p3 = expanded_pk.p3;


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

    
    // Compute s^t * P
    let st_p = calculate_st_p(p1, p2, p3, s_matrix);
    

    // Compute s^t * P * s
    let mut st_p_s = [0u32; K*K*M/8];
    mat_mul_bitsliced_mat_add!(s_matrix, st_p, &mut st_p_s, K, N, K);


    const SIZE: usize = K * (K + 1) / 2; // Size of upper triangular part of matrix of size K x K
    let mut upper_st_p_s = [0u32; SIZE*M/8];
    upper!(&st_p_s, &mut upper_st_p_s, K, K); 

    for i in 0..K {
        for j in (i..K).rev() {

            // // Calculate position of in upper triangular part of matrix
            let pos = i * K + j - (i * (i + 1) / 2);
            let encoded_u = &upper_st_p_s[pos * U32_PER_IDX .. (pos * U32_PER_IDX) + U32_PER_IDX];

            let u = decode_bit_sliced_array!(encoded_u);

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
    let signature = sign(csk, &message);

    // Concatenate signature and message
    // Note the message length cannot be known at compile time (Hence vec is used instead of array)
    let mut sign_con_mes = Vec::with_capacity(SIG_BYTES + message.len());
    sign_con_mes.extend_from_slice(&signature);
    sign_con_mes.extend(message);

    return sign_con_mes;
}






// MAYO algorithm 11
// Expand a public key from its compact representation and verify a signature
pub fn api_sign_open(sign_con_mes: Vec<u8>, cpk: CompactPublicKey) -> (bool, Vec<u8>) {
    
    // Expand public key
    let expanded_pk = expand_pk(cpk);

    // Extract signature and message from input
    let signature = &sign_con_mes[0..SIG_BYTES];
    let message_slice = &sign_con_mes[SIG_BYTES..];

    // Verify the signature based on expanded public key and message
    let result = verify(expanded_pk, signature, message_slice);

    if result == false {
        return (result, vec![0u8]); // If the signature is invalid, the message is set to zero
    }
    return (result, message_slice.to_vec()); // Only convert if valid signature
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

