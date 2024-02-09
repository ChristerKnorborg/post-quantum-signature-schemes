// Methods that define arithmetic over GF(16), with irreducible polynomial of degree 4 over GF(2).
// Concretely, f(x) = x^4 + x + 1 is used. 
use std::u8;


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


pub fn matrix_add() {
    // TODO
}


pub fn matrix_mul() {
    // TODO
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neg() {
        // Negatrion is defined as the additive inverse of a number. 
        // E.g. how much we need to add to a number to get 0. (always the number itself in binary fields)
        assert_eq!(neg(0x0), 0xf); // 0 is its own negation
        assert_eq!(neg(0x1), 0xe); // 1 is its own negation 
        assert_eq!(neg(0xf), 0x0); // 0xf is its own negation
        assert_eq!(neg(0xe), 0x1); // 0xe is its own negation

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
}







