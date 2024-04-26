// Methods that define arithmetic over GF(16), with irreducible polynomial of degree 4 over GF(2).
// Concretely, f(x) = x^4 + x + 1 is used. 
use std::u8;

use crate::{constants::{M, O, V}, crypto_primitives::safe_calculate_p3};



// Negation in GF(16) of any element is the element itself because a is it's own additive inverse (where 0 is the additive identity).
// Hence, -a = a in binary fields (GF(2^n)).
#[inline]  
pub fn neg(x: u8) -> u8 {
    return x; // Negation in GF(2^n) has no effect as a + a = 0.
}

// GF(16) addition is equivalent to XOR because we do bitwise addition modulo 2 (no carry)
#[inline]
pub fn add(x: u8, y: u8) -> u8 {
    return x ^ y;
}

// GF(16) subtraction is equivalent to XOR because we do bitwise subtraction modulo 2 (no carry)
#[inline]
pub fn sub(x: u8, y: u8) -> u8 {
    return x ^ y;
}

// GF(16) multiplication is equivalent to multiplying the polynomials and then reducing modulo the
// irreducible polynomial f(x) = x^4 + x + 1. 
pub fn mul(x: u8, y: u8) -> u8 {

    // Carryless multiplication of polynomials in GF(2^4)
    let mut res: u8;
    res =  (x & 1)*y; // Multiply by x^0
    res ^= (x & 2)*y; // Multiply by x^1
    res ^= (x & 4)*y; // Multiply by x^2
    res ^= (x & 8)*y; // Multiply by x^3
    
    // Reduce modulo by the irreducible polynomial x^4 + x + 1 
    let first_4_bits: u8 = res & 0xf0; // First 4 bits of res (x^7 to x^4. Notice, the first bit is always 0, cause we can't get more than x^6)
    

    // Replace x^4 with x + 1 as x^4 (e.g. 16) = x + 1 (under the irreducible polynomial).
    // Replace x^5 with x^2 + x as x^5 (e.g. 16) = x^2 + x (under the irreducible polynomial).
    // Replace x^6 with x^3 + x^2 as x^6 (e.g. 16) = x^3 + x^2 (under the irreducible polynomial).
    let overflow_bits: u8 = (first_4_bits >> 4) ^ (first_4_bits >> 3);  
    let res : u8 = (res ^ overflow_bits) & 0x0f; // XOR res with the mod reduction of the overflow bits. Then remove first 4 bits from res.
    return res;
}

// From Euler's theorem, we know that an element x in a finite field F satisfies x^{p^{n}-1} = 1,
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




#[macro_export]
macro_rules! matrix_add {
    ($a:expr, $b:expr, $rows:expr, $cols:expr) => {{

        let mut result = [0u8; $cols * $rows];
        safe_matrix_add(&mut result, $a, $b, ($rows * $cols).try_into().unwrap());
        result
    }};
}




#[macro_export]
macro_rules! vector_matrix_mul {
    ($vec:expr, $mat:expr, $vec_len:expr, $mat_cols:expr) => {{

        let mut result = [0u8; $mat_cols];

        for k in 0..$mat_cols {
            let mut column = [0u8 ; $vec_len]; // Mat col is 
            for j in 0..$vec_len {
                column[j] = $mat[j * $mat_cols + k];
            }

            safe_inner_product(&mut result[k], $vec, &column, $vec_len.try_into().unwrap());
        }
        result
    }};
}



#[macro_export]
macro_rules! vector_transposed_matrix_mul {
    ($vec:expr, $mat:expr, $mat_rows:expr, $mat_cols:expr) => {{
        let mut result = [0u8; $mat_cols];

        for j in 0..$mat_cols {
            let row_as_column = &$mat[j * $mat_rows..(j + 1) * $mat_rows];
            safe_inner_product(&mut result[j], $vec, row_as_column, $mat_rows.try_into().unwrap());
        }
        result
    }};
}


#[macro_export]
macro_rules! vector_mul {
    ($a:expr, $b:expr, $LEN:expr) => {{
        let mut result = 0u8;

        safe_inner_product(&mut result, $a, $b, $LEN.try_into().unwrap());

        result
    }};
}



#[macro_export]
macro_rules! transpose_matrix_array {
    ($matrix:expr, $rows:expr, $cols:expr) => {{
        // Initialize the transposed matrix with zeroes. The dimensions are swapped.
        let mut transposed = [[0u8; $rows]; $cols];

        for i in 0..$rows {
            for j in 0..$cols {
                transposed[j][i] = $matrix[i][j]; // Swap elements
            }
        }
        transposed
    }};
}

#[macro_export]
macro_rules! transpose_matrix_array_single {
    ($matrix:expr, $rows:expr, $cols:expr) => {{
        let mut output = [0u8; $rows * $cols]; // Initialize the output array

        for y in 0..$rows {
            for x in 0..$cols {
                let input_index: usize = y * $cols + x; // Calculate index in the input flat array
                let output_index: usize = x * $rows + y; // Calculate index in the output flat array
                output[output_index] = $matrix[input_index];
            }
        }

        output
    }};
}



#[macro_export]
macro_rules! matrix_vec_mul {
    ($matrix:expr, $array:expr, $MAT_ROWS:expr, $MAT_COLS:expr) => {{
        let mut result = [0u8; $MAT_ROWS]; 

        for i in 0..$MAT_ROWS {
            safe_inner_product(&mut result[i], &$matrix[i*$MAT_COLS..], $array, $MAT_COLS.try_into().unwrap());
        }

        result
    }};
}



#[macro_export]
macro_rules! vec_add {
    ($a:expr, $b:expr, $LEN:expr) => {{
        let mut result = [0u8; $LEN];
        safe_matrix_add(&mut result, &$a, &$b, ($LEN).try_into().unwrap());
        result
    }};
}


pub fn calculate_p3(o: [u8; O * V], p1: [u8; V*V*M], p2: [u8; V*O*M]) -> [u8; O*O*M] {

    let mut res = [0u8; O * O * M];

    safe_calculate_p3(
        &mut res,
        &o,
        &p1,
        &p2,
        V.try_into().unwrap(),
        O.try_into().unwrap(),
        M.try_into().unwrap());

        return res;

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
        assert_eq!(add(0x0, 0x0), 0x0); 
        assert_eq!(add(0x1, 0x1), 0x0); 
        assert_eq!(add(0x1, 0x2), 0x3); 
        assert_eq!(add(0x3, 0x1), 0x2); 
        assert_eq!(add(0x6, 0x6), 0x0); 
    }

    #[test]
    fn test_sub() {
        // Subtraction is the same as addition in GF(16)
        assert_eq!(sub(0x0, 0x0), 0x0);
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
        assert_eq!(mul(0xC, 0x7), 0x2); 
        assert_eq!(mul(0xf, 0xf), 0xa); 
    }

    #[test]
    fn test_inv() {
        assert_eq!(inv(0x0), 0x0); // 0 acts as its own inverse, but theorethically it's undefined
        assert_eq!(inv(0x1), 0x1); // 1 is its own inverse
        // For non-trivial inverses, mul(x, inv(x)) = 1
        assert_eq!(inv(0x2), 0x9); // x's inverse is x^3 + 1 
        assert_eq!(inv(0x3), 0xe); // (x + 1)'s inverse is x^3 + x^2 + x
        assert_eq!(inv(0x4), 0xd); // x^2's inverse is x^3 + x^2 + 1
    }


}