use std::vec;
use cipher::consts::U32;

use crate::crypto_primitives::{safe_aes_128_ctr, safe_random_bytes, safe_shake256};
use crate::finite_field::{add, mul};
use crate::sample::sample_solution;
use crate::bitsliced_arithmetic::{create_big_p_bitsliced, ot_times_p2, p1_p1t_times_o_plus_p2, p1_times_o_add_p2, st_times_big_p, st_times_big_p_times_s, upper_big_p, upper_p3, vt_times_l};
use crate::constants::{
    CPK_BYTES, CSK_BYTES, DIGEST_BYTES, EPK_BYTES, ESK_BYTES, F_Z, K, L_BYTES, M, N, V, O, O_BYTES, P1_BYTES, P2_BYTES, P3_BYTES,
    PK_SEED_BYTES, R_BYTES, SALT_BYTES, SIG_BYTES, SK_SEED_BYTES, V_BYTES, SHIFTS
};
use crate::utils::{bytes_to_hex_string, write_u32_array_to_file_byte, write_u8_array_to_file_byte};
use crate::{
    decode_bit_sliced_array, decode_bit_sliced_matrices, decode_bytestring_matrix_array, decode_bytestring_to_array, 
    encode_bit_sliced_array, encode_bit_sliced_matrices, encode_to_bytestring_array, matrix_add, matrix_mul, matrix_vec_mul,
    transpose_matrix_array, vector_matrix_mul, vector_mul, vector_transposed_matrix_mul
};

pub struct ExpandedSecretKey {
    p1: [u32 ; P1_BYTES/4],
    l:  [u32 ; L_BYTES/4],
    o:  [u8 ; O_BYTES]
}



const U32_PER_IDX: usize = M / 4 / 2;




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
    
    let (p1_bytes, p2_bytes) = p.split_at_mut(P1_BYTES);


    let mut p1_u32 = [0u32 ; P1_BYTES/4];
    for (i, chunk) in p1_bytes.chunks(4).enumerate() {
        let mut array = [0u8; 4];
        array.copy_from_slice(chunk);
        p1_u32[i] = u32::from_le_bytes(array); // Use from_be_bytes for big endian
    }


    let mut p2_u32 = [0u32 ; P2_BYTES/4];
    for (i, chunk) in p2_bytes.chunks(4).enumerate() {
        let mut array = [0u8; 4];
        array.copy_from_slice(chunk);
        p2_u32[i] = u32::from_le_bytes(array); // Use from_be_bytes for big endian
    }

    
    // m p1 matrices are of size (n−o) × (n−o)
    // m p2 matrices are of size (n−o) × o (not upper triangular matrices)
    // Allocate space for P3_i. Size is o × o
    let mut p3 = [0u32 ; O*O*M/8];


    // Compute P3 = (−O^T * P1 * O ) − (−O^T * P2) as P3 = O^t * (P1*O + P2)



    // Compute (P1*O + P2) in p2
    p1_times_o_add_p2(&p1_u32, o, &mut p2_u32);

    // Compute P3 = O^t * (P1*O + P2) in accumulated in p3
    ot_times_p2(o, &p2_u32, &mut p3);

    // Compute upper of p3
    let mut p3_upper = [0u32 ; P3_BYTES/4];
    upper_p3(&mut p3, &mut p3_upper); // OPTIMIZE THIS!


    let mut p3_u8 = [0u8 ; P3_BYTES];
    for (i, &num) in p3_upper.iter().enumerate() {
        let byte_slice = num.to_le_bytes(); // Convert each u32 to 4 u8s. Use to_be_bytes for big endian.
        let start_index = i * 4;
        p3_u8[start_index..start_index + 4].copy_from_slice(&byte_slice);
    }


    // Public and secret keys
    let mut cpk = [0u8 ; PK_SEED_BYTES + P3_BYTES]; // contains pk_seed and encoded_p3
    let csk: [u8 ; CSK_BYTES] = sk_seed;

    cpk[..PK_SEED_BYTES].copy_from_slice(&pk_seed);
    cpk[PK_SEED_BYTES..].copy_from_slice(&p3_u8); 

    
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
    let pk_seed: [u8; PK_SEED_BYTES] = pk_seed_slice
        .try_into()
        .expect("Slice has incorrect length");

    // Make Oil space from o_bytes. Only a single is yielded from decode_bit_sliced_matrices in this case
    let o_bytes = &s[PK_SEED_BYTES..PK_SEED_BYTES+O_BYTES];
    let o = decode_bytestring_matrix_array!(o_bytes, V, O);


    // Derive P1_i and P2_i from pk_seed
    let mut p = [0u8; (P1_BYTES + P2_BYTES)];
    safe_aes_128_ctr(
        &mut p,
        (P1_BYTES + P2_BYTES) as u64,
        &pk_seed,
        PK_SEED_BYTES as u64,
    );

    
    let (p1_bytes, p2_bytes) = p.split_at_mut(P1_BYTES);

    let mut p1_u32 = [0u32 ; P1_BYTES/4];
    for (i, chunk) in p1_bytes.chunks(4).enumerate() {
        let mut array = [0u8; 4];
        array.copy_from_slice(chunk);
        p1_u32[i] = u32::from_le_bytes(array); // Use from_be_bytes for big endian
    }


    let mut p2_u32 = [0u32 ; P2_BYTES/4];
    for (i, chunk) in p2_bytes.chunks(4).enumerate() {
        let mut array = [0u8; 4];
        array.copy_from_slice(chunk);
        p2_u32[i] = u32::from_le_bytes(array); // Use from_be_bytes for big endian
    }
    
    // Compute L matrices BITSLICED and put them inside P_2
    p1_p1t_times_o_plus_p2(&p1_u32, o,  &mut p2_u32); // m matrices of size (n−o) × o

    // To follow the refference implementation append O_bytestring at the end
    // Do not add sk_seed to the expanded secret key

    let mut esk =  ExpandedSecretKey { p1: [0u32 ; P1_BYTES/4],
        l:  [0u32 ; P2_BYTES/4],
        o:  [0u8 ; O_BYTES]};

    esk.p1.copy_from_slice(&p1_u32);
    esk.l.copy_from_slice(&p2_u32);
    esk.o.copy_from_slice(&o_bytes);

    return esk;
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
    let mut aes_bytes = [0u8; (P1_BYTES + P2_BYTES)];
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
pub fn sign(compact_secret_key: [u8 ; CSK_BYTES], message: Vec<u8>) -> [u8 ; SIG_BYTES] {

    let mut x = [0u8; K*O]; // Initialize x to zero
    let mut v = [[0u8; V]; K]; // Initialize v to zero

    // Unlike specifcation, sk_seed is NOT included ESK
    let expanded_sk: ExpandedSecretKey = expand_sk(compact_secret_key);

    // Decode expanded secret key
    let p1 = &expanded_sk.p1;
    let l = &expanded_sk.l;
    let o_bytestring = &expanded_sk.o;

    
    

    let mut p1_bytestring = [0u8 ; P1_BYTES];
    for (i, &num) in expanded_sk.p1.iter().enumerate() {
        let byte_slice = num.to_le_bytes(); // Convert each u32 to 4 u8s. Use to_be_bytes for big endian.
        let start_index = i * 4;
        p1_bytestring[start_index..start_index + 4].copy_from_slice(&byte_slice);
    }


    let mut l_bytestring = [0u8 ; L_BYTES];
    for (i, &num) in expanded_sk.l.iter().enumerate() {
        let byte_slice = num.to_le_bytes(); // Convert each u32 to 4 u8s. Use to_be_bytes for big endian.
        let start_index = i * 4;
        l_bytestring[start_index..start_index + 4].copy_from_slice(&byte_slice);
    }


    let o = decode_bytestring_matrix_array!(o_bytestring, V, O);



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

        let mut m_matrices_array = [0u32 ; K*O*M / 8];

        // Build K matrices of size M x O
        vt_times_l(v, l, &mut m_matrices_array);


        let mut m_matrices = [[[0u8; O]; M]; K]; 
        
        
        // Assign the indexes of m matrices to columns of m
        for i in 0..K {
            for j in 0..O {

                let curr_idx = (M/8) * (i * O + j);
                let curr_vec = &m_matrices_array[curr_idx..curr_idx+U32_PER_IDX];

                let mut encoded_m_row_u8 = [0u8 ; M/2];
                for (d, &num) in curr_vec.iter().enumerate() {
                    let byte_slice = num.to_le_bytes(); // Convert each u32 to 4 u8s. Use to_be_bytes for big endian.
                    let start_index = d * 4;
                    encoded_m_row_u8[start_index..start_index + 4].copy_from_slice(&byte_slice);
                }   
                
                let decoded_m_row = decode_bit_sliced_array!(encoded_m_row_u8, M);
                for col in 0..M {
                    m_matrices[i][col][j] = decoded_m_row[col];
                }
            }
        }

    





        break; // If Ok, break the loop
    } // ctr loop ends

    // dummy return
    return [0u8 ; SIG_BYTES];
}





// MAYO algorithm 9
// Verifi the signature of a message using the expanded public key
pub fn verify(expanded_pk: [u8 ; EPK_BYTES], signature: &[u8], message: &Vec<u8>) -> bool {

    // Retrieve the public information from the expanded public key
    let p1_bytes = &expanded_pk[0..P1_BYTES];
    let p2_bytes = &expanded_pk[P1_BYTES..P1_BYTES + P2_BYTES];
    let p3_bytes = &expanded_pk[P1_BYTES + P2_BYTES..];



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





    
    




    let mut p1_u32 = [0u32 ; P1_BYTES/4];
    for (i, chunk) in p1_bytes.chunks(4).enumerate() {
        let mut array = [0u8; 4];
        array.copy_from_slice(chunk);
        p1_u32[i] = u32::from_le_bytes(array); // Use from_be_bytes for big endian
    }

    let mut p2_u32 = [0u32 ; P2_BYTES/4];
    for (i, chunk) in p2_bytes.chunks(4).enumerate() {
        let mut array = [0u8; 4];
        array.copy_from_slice(chunk);
        p2_u32[i] = u32::from_le_bytes(array); // Use from_be_bytes for big endian
    }

    let mut p3_u32 = [0u32 ; P3_BYTES/4];
    for (i, chunk) in p3_bytes.chunks(4).enumerate() {
        let mut array = [0u8; 4];
        array.copy_from_slice(chunk);
        p3_u32[i] = u32::from_le_bytes(array); // Use from_be_bytes for big endian
    }



    let mut big_p = [0u32 ; (P1_BYTES + P2_BYTES + P3_BYTES)/4];

    // Construct matrices P*_i of size N x N s.t. (P^1_a P^2_a)
    // for every matrix a ∈ [m]                   (0     P^3_a)
    create_big_p_bitsliced(&p1_u32, &p2_u32, &p3_u32, &mut big_p); // OPTIMIZE THIS!




    // Compute s^t * P*
    let mut s_p = [0u32; K*N*M/8];
    st_times_big_p(s_matrix, &big_p, &mut s_p);
    
    // Compute s^t * P* * s
    let mut temp = [0u32; K*K*M/8];
    st_times_big_p_times_s(&s_p, s_matrix, &mut temp);

    const SIZE: usize = K * (K + 1) / 2; // Size of upper triangular part of matrix of size K x K
    let mut upper_temp = [0u32; SIZE*M/8];
    upper_big_p(&temp, &mut upper_temp); 

    for i in 0..K {
        for j in (i..K).rev() {


            // // Calculate position of in upper triangular part of matrix
            let pos = i * K + j - (i * (i + 1) / 2);
            let mut encoded_u = &upper_temp[pos * U32_PER_IDX .. (pos * U32_PER_IDX) + U32_PER_IDX];


            let mut encoded_u_u8 = [0u8 ; M/2];
            for (d, &num) in encoded_u.iter().enumerate() {
                let byte_slice = num.to_le_bytes(); // Convert each u32 to 4 u8s. Use to_be_bytes for big endian.
                let start_index = d * 4;
                encoded_u_u8[start_index..start_index + 4].copy_from_slice(&byte_slice);
            }




            let u = decode_bit_sliced_array!(encoded_u_u8, M);





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

