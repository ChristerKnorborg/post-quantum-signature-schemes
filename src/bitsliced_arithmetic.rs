use std::vec;
use crate::constants::{
    CSK_BYTES, DIGEST_BYTES, EPK_BYTES, ESK_BYTES, F_Z, K, L_BYTES, M, N, O, O_BYTES, P1_BYTES,
    P2_BYTES, P3_BYTES, PK_SEED_BYTES, R_BYTES, SALT_BYTES, SIG_BYTES, SK_SEED_BYTES, V_BYTES,
};

// Multiply m matrices with a single matrix and return res
// * For bitslcied matrices *
pub fn m_matrices_mult_matrix_m(matrices: Vec<u8>, matrix: Vec<u8>) -> Vec<u8> {
    let p = N - O;

    for i in 0..p {
        for j in 0..p {
            for k in 0..O {
        
            }
        }
    }

    return vec![0u8 ; M]
}