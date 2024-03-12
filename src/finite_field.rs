// Methods that define arithmetic over GF(16), with irreducible polynomial of degree 4 over GF(2).
// Concretely, f(x) = x^4 + x + 1 is used. 
use std::u8;

use crate::constants::{K, M, N, O, V};


// Negation in GF(16) of any element is the element itself because a is it's own additive inverse (where 0 is the additive identity).
// Hence, -a = a in binary fields (GF(2^n)).  
pub fn neg(x: u8) -> u8 {
    return x; // Negation in GF(2^n) has no effect as a + a = 0.
}

// GF(16) addition is equivalent to XOR because we do bitwise addition modulo 2 (no carry)
pub fn add(x: u8, y: u8) -> u8 {
    return x ^ y;
}

// GF(16) subtraction is equivalent to XOR because we do bitwise subtraction modulo 2 (no carry)
pub fn sub(x: u8, y: u8) -> u8 {
    return x ^ y;
}

// GF(16) multiplication is equivalent to multiplying the polynomials and then reducing modulo the irreducible polynomial. 
pub fn mul(x: u8, y: u8) -> u8 {

    // Carryless multiplication of polynomials in GF(2^4)
    let mut res: u8;
    res =  (x & 1)*y; // Multiply by x^0
    res ^= (x & 2)*y; // Multiply by x^1
    res ^= (x & 4)*y; // Multiply by x^2
    res ^= (x & 8)*y; // Multiply by x^3

    // Reduce modulo by the irreducible polynomial x^4 + x + 1 
    let first_4_bits: u8 = res & 0xf0; // First 4 bits of res (x^7 to x^4. Notice, the first bit is always 0, cause we can't get more than x^6)
    let overflow_bits: u8 = (first_4_bits >> 4) ^ (first_4_bits >> 3); // Replace x^4 with x + 1 as x^4 (e.g. 16) = x + 1 (under the irreducible polynomial). Notice, + is XOR in binary fields.
    let res : u8 = (res ^ overflow_bits) & 0x0f; // XOR res with the mod reduction of the overflow bits. Then remove first 4 bits from res.
    return res;
}

// From Fermat's little theorem, we know that an element x in a finite field F satisfies x^{p^{n}-1} = 1,
// where p is the characteristic of F and n is the degree of the extension. From this we can deduce that x^{14} * x = x^{-1} * x = 1.
// E.g. x^14 = x^-1 (the multiplicative inverse of x)      
pub fn inv(x: u8) -> u8{

    // u8 table[16] = {0, 1, 9, 14, 13, 11, 7, 6, 15, 2, 12, 5,
    // 10, 4, 3, 8}; return table[a & 15];

    // Calculate multiplicative inverse of x by exponentiation by squaring (x^14 = x^-1) 
    let x2: u8 = mul(x, x);
    let x4: u8 = mul(x2, x2);
    let x6: u8 = mul(x2, x4);
    let x8: u8 = mul(x4, x4);
    let x14: u8 = mul(x8, x6);

    return x14;
}


// GF(16) division is equivalent to multiplying the dividend by the multiplicative inverse of the divisor.
pub fn div(x: u8, y: u8) -> u8 {
    return mul(x, inv(y));
}




pub fn matrix_add(a: &Vec<Vec<u8>>, b: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {

    assert_eq!(a.len(), b.len(), "Matrices must have the same number of rows");
    assert_eq!(a[0].len(), b[0].len(), "Matrices must have the same number of columns");

    a.iter().zip(b.iter())
        .map(|(row_a, row_b)| {
            row_a.iter().zip(row_b.iter())
                .map(|(&val_a, &val_b)| add(val_a, val_b))
                .collect()
        })
        .collect()
}

// Matrix subtraction over GF(16)
pub fn matrix_sub(a: &Vec<Vec<u8>>, b: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {

    assert_eq!(a.len(), b.len(), "Matrices must have the same number of rows");
    assert_eq!(a[0].len(), b[0].len(), "Matrices must have the same number of columns");

    a.iter().zip(b.iter())
        .map(|(row_a, row_b)| {
            row_a.iter().zip(row_b.iter())
                .map(|(&val_a, &val_b)| sub(val_a, val_b))
                .collect()
        })
        .collect()
}

// Matrix subtraction over GF(16)
pub fn vector_sub(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {

    //assert_eq!(a.len(), b.len(), "Vectors must have the same length");

    return a.iter().zip(b.iter())
        .map(|(&val_a, &val_b)| sub(val_a, val_b))
        .collect()
}


// Matrix multiplication over GF(16) (also works for matrix-vector multiplication)
pub fn matrix_mul(a: &Vec<Vec<u8>>, b: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {

    assert_eq!(a[0].len(), b.len(), "Number of columns in A must equal number of rows in B");

    let rows_a: usize = a.len();
    let cols_a = a[0].len();
    let cols_b = b[0].len();

    let mut result = vec![vec![0; cols_b]; rows_a];

    for i in 0..rows_a {
        for j in 0..cols_b {
            for k in 0..cols_a {
                // Take the dot product of the i-th row of A and the j-th column of B
                result[i][j] = add(result[i][j], mul(a[i][k], b[k][j])); 

            }
        }
    }
    return result
}




pub fn matrix_add_array(a: [[u8; O]; N-O], b: [[u8; O]; N-O]) -> [[u8; O]; N-O] {

    let mut result = [[0; O]; N - O]; // Initialize the result array

    for i in 0..(N - O) {
        for j in 0..O {
            result[i][j] = add(a[i][j], b[i][j]); // Perform addition for corresponding elements
        }
    }
    result
}

pub fn matrix_v_add_array(a: [[u8; V]; V], b: [[u8; V]; V]) -> [[u8; V]; V] {

    let mut result = [[0; V]; V]; // Initialize the result array

    for i in 0..V {
        for j in 0..V {
            result[i][j] = add(a[i][j], b[i][j]); // Perform addition for corresponding elements
        }
    }
    result
}

pub fn matrix_mul_array_p2(a: [[u8; N-O]; O], b: [[u8; O]; N-O]) -> [[u8 ; O]; O] {


    let rows_a = O;
    let cols_a = N-O;
    let cols_b = O;

    let mut result = [[0; O]; O];

    for i in 0..rows_a {
        for j in 0..cols_b {
            for k in 0..cols_a {
                // Take the dot product of the i-th row of A and the j-th column of B
                result[i][j] = add(result[i][j], mul(a[i][k], b[k][j])); 

            }
        }
    }
    return result
}


pub fn matrix_mul_o_p1(a: [[u8; N-O]; N-O], b: [[u8; O]; N-O]) -> [[u8 ; O]; N-O] {


    let rows_a = N-O;
    let cols_a = N-O;
    let cols_b = O;

    let mut result = [[0; O]; N-O];

    for i in 0..rows_a {
        for j in 0..cols_b {
            for k in 0..cols_a {
                // Take the dot product of the i-th row of A and the j-th column of B
                result[i][j] = add(result[i][j], mul(a[i][k], b[k][j])); 

            }
        }
    }
    return result
}

pub fn matrix_mul_v_l(a: [[u8; V]; 1], b: [[u8; O]; V]) -> [u8 ; O] {

    let mut result = [0; O];

    for j in 0..V {
        for k in 0..O {
            // Take the dot product of the i-th row of A and the j-th column of B
            result[k] = add(result[k], mul(a[0][j], b[j][k])); 

        }
    }
    return result
}

pub fn matrix_mul_s_trans_big_p(s: [u8; N], big_p: [[u8; N]; N]) -> [u8 ; N] {

    let mut result = [0; N];

    for j in 0..N {
        for k in 0..N {
            // Take the dot product of the i-th row of A and the j-th column of B
            result[k] = add(result[k], mul(s[j], big_p[j][k])); 

        }
    }
    return result
}

pub fn array_mul_s_p(s: [u8; N], p: [u8; N]) -> u8 {

    let mut result = 0;

    for i in 0..N {
            // Take the dot product of the i-th row of A and the j-th column of B
            result = add(result, mul(s[i], p[i])); 
    }
    return result
}

pub fn matrix_mul_v_p1(a: [[u8; V]; 1], b: [[u8; V]; V]) -> [[u8 ; V]; 1] {

    let mut result = [[0; V]; 1];

    for j in 0..V {
        for k in 0..V {
            // Take the dot product of the i-th row of A and the j-th column of B
            result[0][j] = add(result[0][j], mul(a[0][k], b[k][j])); 

        }
    }
    return result
}


// Vector-matrix multiplication over GF(16).
// Returns a vector of size equal to the number of columns in the matrix.
pub fn vector_matrix_mul(vec: &Vec<u8>, matrix: &Vec<Vec<u8>>) -> Vec<u8> {
    
    assert_eq!(vec.len(), matrix.len(), "Length of vector must equal number of rows in matrix");

    let rows_matrix = matrix.len();
    let cols_matrix = matrix[0].len();

    let mut result = vec![0; cols_matrix]; // 1 x cols_matrix vector

    for j in 0..cols_matrix {
        for i in 0..rows_matrix {
            // Multiply each element of the vector by the corresponding element in the matrix column and sum the results
            result[j] = add(result[j], mul(vec[i], matrix[i][j])); 
        }
    }
    return result;
}


// Matrix-vector multiplication over GF(16)
// Returns a vector of size equal to the number of rows in the matrix.
pub fn matrix_vector_mul(matrix: &Vec<Vec<u8>>, vec: &Vec<u8>) -> Vec<u8> {
    assert_eq!(matrix[0].len(), vec.len(), "Number of columns in matrix must equal length of vector");

    let rows_matrix = matrix.len();
    let cols_matrix = matrix[0].len();

    let mut result = vec![0; rows_matrix]; // rows_matrix x 1 vector

    for i in 0..rows_matrix {
        for k in 0..cols_matrix {
            // Multiply each element of the i-th row of the matrix by the corresponding element in the vector and sum the results
            result[i] = add(result[i], mul(matrix[i][k], vec[k])); 
        }
    }

    return result;
}

pub fn a_mul_r(matrix: [[u8; K*O]; M], array: [u8 ; K*O]) -> [u8 ; M] {
    let mut result = [0; M]; // rows_matrix x 1 vector


    for i in 0..M {
        for j in 0..K*O {
            // Multiply each element of the i-th row of the matrix by the corresponding element in the vector and sum the results
            result[i] = add(result[i], mul(matrix[i][j], array[j])); 
        }
    }
    return result;
}


pub fn o_matrix_x_idx_mul(matrix: [[u8; O]; V], array: &[u8]) -> [u8 ; V] {
    let mut result = [0u8 ; V]; // V x 1 vector

    for i in 0..V {
        for j in 0..O {
            // Multiply each element of the i-th row of the matrix by the corresponding element in the vector and sum the results
            result[i] = add(result[i], mul(matrix[i][j], array[j])); 
        }
    }

    return result;
}



pub fn p1_matrix_v_mul(matrix: [[u8; V]; 1], array: [u8 ; V]) -> u8 {
    let mut result = 0; // rows_matrix x 1 vector


    for i in 0..V {
        // Multiply each element of the i-th row of the matrix by the corresponding element in the vector and sum the results
        result = add(result, mul(matrix[0][i], array[i])); 
    }

    return result;
}

// Matrix negation over GF(16)
pub fn matrix_neg(a: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    a.iter()
        .map(|row| row.iter().map(|&val| neg(val)).collect())
        .collect()
}







#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neg() {
        // Negatrion is defined as the additive inverse of a number. 
        // E.g. how much we need to add to a number to get 0. (always the number itself in binary fields)
        assert_eq!(neg(0x0), 0x0); // 0 is its own negation
        assert_eq!(neg(0x1), 0x1); // 1 is its own negation 
        assert_eq!(neg(0xf), 0xf); // 0xf is its own negation
        assert_eq!(neg(0xe), 0xe); // 0xe is its own negation

    }

    #[test]
    fn test_add() {
        assert_eq!(add(0x0, 0x0), 0x0); // 0 is the additive identity
        assert_eq!(add(0x1, 0x1), 0x0); // 1 is its own additive inverse
        assert_eq!(add(0x1, 0x2), 0x3); // 1 + 2 = 3
        assert_eq!(add(0x3, 0x1), 0x2); // 3 + 1 = 2
        assert_eq!(add(0x6, 0x6), 0x0); // 6 is its own additive inverse
    }

    #[test]
    fn test_sub() {
        // Subtraction is the same as addition in GF(16)
        assert_eq!(sub(0x0, 0x0), 0x0); // 0 is the additive identity
        assert_eq!(sub(0x3, 0x1), 0x2); // (x + 1) - 1 = x
        assert_eq!(sub(0x1, 0x2), 0x3); // 1 - x = x + 1
        assert_eq!(sub(0x6, 0xf), 0x9); // x^2 + x - (x^3 + x^2 + x + 1) = x^3 + 1
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(0x0, 0x0), 0x0); // 0 * 0 = 0
        assert_eq!(mul(0x1, 0x1), 0x1); // 1 * 1 = 1
        assert_eq!(mul(0x2, 0x2), 0x4); // x * x = x^2 = 4
        assert_eq!(mul(0x3, 0x3), 0x5); // (x + 1) * (x + 1) = x^2 + 2x + 1 = x^2 + 1 (as modulo 2 eats the 2x - no modular reduction needed)

        assert_eq!(mul(0xC, 0x3), 0x7); 
        // (x^3 + x^2) * (x + 1) = x^4 + 2x^3 + x^2 
        // = x^4 + x^2 (Term-wise modulo 2 reduction)
        // x^4 + x^2 + (x^4 + x + 1) = x^2 + x + 1 (By doing modular reduction on x^4 + x^2 with f(x) = x^4 + x + 1 - then modulo 2 term-wise)

        assert_eq!(mul(0xC, 0x7), 0x2); 
        // (x^3 + x^2) * (x^2 + x + 1) = x^5 + 2x^4 + 2x^3 + x^2 = x^5 + x^2 (Term-wise modulo 2 reduction)
        // x^5 + x^2 + (x^5 + x^2 + x) = 2x^5 + 2x^2 + x (By doing modular reduction on x^5 + x^2 with x * f(x) = x^5 + x^2 + x)
        // = x  (modulo 2 term-wise)

        assert_eq!(mul(0xf, 0xf), 0xa); 
        // (x^3 + x^2 + x + 1) * (x^3 + x^2 + x + 1) = x^6 + 2x^5 + 3x^4 + 4x^3 + 3x^2 + x + 1
        // = x^6 + x^4 + x^2 + 1 (Term-wise modulo 2 reduction)
        // x^6 + x^4 + x^3 + x^2 + 1 + (x^6 + x^3 + x^2) = 2x^6 + x^4 + 2x^3 + 2x^2 + 1 (By doing modular reduction on x^6 + x^4 + x^3 + x^2 + 1 with x^2 * f(x) = x^6 + x^3 + x^2)
        // = x^4 + x^3 + 1 (modulo 2 term-wise)
        // x^4 + x^3 + 1 + (x^4 + x + 1) = 2x^4 + x^3 + x + 2 (By doing modular reduction on x^4 + x^3 + 1 with f(x) = x^4 + x + 1)
        // = x^3 + x  (modulo 2 term-wise)
    }

    #[test]
    fn test_inv() {
        assert_eq!(inv(0x0), 0x0); // 0 acts as its own inverse, but theorethically it's undefined
        assert_eq!(inv(0x1), 0x1); // 1 is its own inverse
        // For non-trivial inverses, mul(x, inv(x)) = 1
        assert_eq!(inv(0x2), 0x9); // x's inverse is x^3 + 1 
        assert_eq!(inv(0x3), 0xe); // (x + 1)'s inverse is x^3 + x^2 + x
        assert_eq!(inv(0x4), 0xd); // x^2's inverse is x^3 + x^2 + 1
        assert_eq!(inv(0x5), 0xb); // (x^2 + 1)'s inverse is x^3 + x + 1
        assert_eq!(inv(0x6), 0x7); // (x^2 + x)'s inverse is x^2 + x + 1
        assert_eq!(inv(0x8), 0xf); // x^3's inverse is x^3 + x^2 + x + 1
    }


    #[test]
    fn test_matrix_add() {

        let a = vec![vec![0, 1, 5],
                                   vec![3, 4, 6]];

        let b = vec![vec![3, 4, 5],
                                   vec![0, 1, 7]];

        let result = matrix_add(&a, &b);

        // Notice (x^2 + 1) + (x^2 + 1) = 0. (5 + 5 = 0)
        // And (x^2 + x + 1) + (x^2 + x) = 1. (6 + 7 = 1)
        let expected = vec![vec![3, 5, 0], 
                                          vec![3, 5, 1]]; 
        
        assert_eq!(result, expected, "Matrix addition is not correct");
    }


    #[test]
    // GF(16) subtraction is the same as addition
    fn test_matrix_sub() {

        let a = vec![vec![0, 1, 5],
                                   vec![3, 4, 6]];

        let b = vec![vec![3, 4, 5],
                                   vec![0, 1, 7]];

        let result = matrix_sub(&a, &b);

        // Notice (x^2 + 1) + (x^2 + 1) = 0. (5 + 5 = 0)
        // And (x^2 + x + 1) + (x^2 + x) = 1. (6 + 7 = 1)
        let expected = vec![vec![3, 5, 0], 
                                          vec![3, 5, 1]]; 
        
        assert_eq!(result, expected, "Matrix addition is not correct");
    }


    #[test]
    fn test_matrix_neg() {

        let a = vec![vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
                                   vec![9, 10, 11, 12, 13, 14, 15,]];

        let expected = a.clone(); // Negation in GF(16) should be the element itself

        let result = matrix_neg(&a);
        
        assert_eq!(result, expected, "Matrix negation is not correct");
    }

    
    #[test]
    fn test_matrix_mul_simple() {

        let a = vec![vec![2],
                                   vec![8]];

        let b = vec![vec![3, 1]];

        let result = matrix_mul(&a, &b);

        let expected = vec![vec![6, 2],
                                          vec![11, 8]]; 
        
        assert_eq!(result, expected, "Matrix multiplication is not correct");
    }


    #[test]
    fn test_matrix_mul() {

        let a = vec![vec![2, 2, 2, 2, 2, 2, 2, 2],
                                   vec![4, 4, 5, 5, 5, 6, 6, 8]];

        let b = vec![vec![0, 1],
                                   vec![2, 3],
                                   vec![4, 5],
                                   vec![6, 7],
                                   vec![8, 9],
                                   vec![10, 11],
                                   vec![12, 13],
                                   vec![14, 15]];

        let result = matrix_mul(&a, &b);

        let expected = vec![vec![0, 0],
                                          vec![2, 15]]; 
        
        assert_eq!(result, expected, "Matrix multiplication is not correct");
    }


    #[test]
    fn test_matrix_mul_with_vector_wrapped_as_matrix() {

        // Row vector of size 1x2
        let a = vec![vec![0, 2]];

        // Matrix of size 2x2
        let b = vec![vec![0, 1],
                                   vec![2, 3],];

        let result = matrix_mul(&a, &b);

        // 0*0 + 2*2 = 4, 0*1 + 2*3 = 6 (still works with GF(16 for these small examples).)
        let expected = vec![vec![4, 6]]; 
        
        assert_eq!(result, expected, "Matrix-vector multiplication is not correct");
    }


    #[test]
    fn test_vector_matrix_mul() {

        let vec = vec![0, 2]; // 1x2 vector

        let matrix = vec![vec![0, 1], // 2x2 matrix
                                        vec![2, 3],];

        let result = vector_matrix_mul(&vec, &matrix);


        let cols_expected = 2; // Expected dimensions 1x2 vector


        assert_eq!(result.len(), cols_expected, "Result vector has wrong number of cols");


        // 0*0 + 2*2 = 4, 0*1 + 2*3 = 6 (still works with GF(16 for these small examples).)
        let expected = vec![4, 6]; 
        
        assert_eq!(result, expected, "Vector-matrix multiplication is not correct");
    }


    #[test]
    fn test_matrix_vector_mul() {

        let matrix = vec![vec![0, 1],
                                   vec![2, 3],];

        let vec = vec![0, 2];

        let result = matrix_vector_mul(&matrix, &vec);


        let cols_expected = vec.len(); // Matrix has 2 columns


        assert_eq!(result.len(), cols_expected, "Result vector has wrong number of columns");

        // 0*0 + 1*2 = 2, 2*0 + 3*2 = 6 (still works with GF(16 for these small examples).)
        let expected = vec![2, 6]; 
        
        assert_eq!(result, expected, "Matrix-vector multiplication is not correct");
    }

    

}







