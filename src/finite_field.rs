#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldElement {
    value: usize,
    modulus: usize,
}

// Represents a vector in the finite field F_q^n
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiniteFieldVector {
    elements: Vec<FieldElement>,
}

impl FieldElement {
        // Constructs a new field element with the given value and modulus.
        fn new(value: usize, modulus: usize) -> Self {
            FieldElement {
                value: value % modulus,
                modulus,
            }
        }

    // Adds two field elements together.
    fn add(&mut self, other: FieldElement) {
        assert_eq!(self.modulus, other.modulus);
        self.value = (self.value + other.value) % self.modulus;
    }

    // Subtracts another field element from this field element.
    fn sub(&mut self, other: FieldElement) {
        assert_eq!(self.modulus, other.modulus);
        self.value = (self.value + self.modulus - other.value) % self.modulus;
    }

    // Multiplies two field elements together.
    fn mul(&mut self, other: FieldElement) {
        assert_eq!(self.modulus, other.modulus);
        self.value = (self.value * other.value) % self.modulus;
    }

    // Divides this field element by another field element (where the other field element is not zero).
    // (division in a finite field is equivalent to multiplying by the multiplicative inverse, which exists for all non-zero elements of the field)
    fn div(&mut self, other: FieldElement) {
        assert_eq!(self.modulus, other.modulus);
        assert!(other.value != 0, "Attempt to divide by zero");

        let inverse = other.inv(); 
        self.mul(inverse); 
    }

    // Calculates and returns the multiplicative inverse of the field element. 
    fn inv(&self) -> FieldElement { 
        assert!(self.value != 0, "Attempt to take inverse of zero");

        let mut exp = self.modulus - 2;
        let mut base = self.value;
        let mut result = 1;

        // Exponentiation by squaring
        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % self.modulus;
            }
            base = (base * base) % self.modulus;
            exp /= 2;
        }

        FieldElement::new(result, self.modulus)
    }
}




impl FiniteFieldVector {

    // Constructs a new vector given a list of elements
    fn new(elements: Vec<FieldElement>) -> Self {

        // Check that all elements have the same modulus
        let modulus = elements[0].modulus;
        for element in &elements {
            assert_eq!(element.modulus, modulus);
        }
        // Return new vector
        FiniteFieldVector { elements }
    }

    // Vector addition
    fn add(&mut self, other: &FiniteFieldVector) {
        assert_eq!(self.elements.len(), other.elements.len(), "Vectors must be of the same length");
        for (a, b) in self.elements.iter_mut().zip(&other.elements) {
            a.add(*b);
        }
    }

    // Scalar multiplication
    fn scalar_mul(&mut self, scalar: FieldElement) {
        for a in &mut self.elements {
            a.mul(scalar);
        }
    }


    // Dot product
    fn dot(&self, other: &FiniteFieldVector) -> FieldElement {
        assert_eq!(self.elements.len(), other.elements.len(), "Vectors must be of the same length");
        let mut result = FieldElement::new(0, self.elements[0].modulus);
        for (a, b) in self.elements.iter().zip(&other.elements) {
            let mut temp = *a;
            temp.mul(*b);
            result.add(temp);
        }
        result
    }

}










#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let mut a = FieldElement::new(2, 5);
        let b = FieldElement::new(3, 5);
        a.add(b);
        assert_eq!(a, FieldElement::new(0, 5)); // 2 + 3 mod 5 = 0
    }

    #[test]
    fn test_subtraction() {
        let mut a = FieldElement::new(3, 5);
        let b = FieldElement::new(2, 5);
        a.sub(b);
        assert_eq!(a, FieldElement::new(1, 5)); // 3 - 2 mod 5 = 1
    }

    #[test]
    fn test_multiplication() {
        let mut a = FieldElement::new(2, 5);
        let b = FieldElement::new(3, 5);
        a.mul(b);
        assert_eq!(a, FieldElement::new(1, 5)); // 2 * 3 mod 5 = 1
    }

    #[test]
    fn test_division() {
        let mut a = FieldElement::new(2, 7);
        let b = FieldElement::new(3, 7);
        a.div(b);
        assert_eq!(a, FieldElement::new(3, 7)); // 2 / 3 mod 7 = 3
    }

    #[test]
    fn test_inverse() {
        let a = FieldElement::new(3, 7);
        let b = a.inv();
        assert_eq!(b, FieldElement::new(5, 7)); // 3^-1 mod 7 = 5
    }
}



#[cfg(test)]
mod vector_tests {
    use super::*;

    #[test]
    fn test_vector_addition() {
        let mut v1 = FiniteFieldVector::new(vec![FieldElement::new(1, 5), FieldElement::new(2, 5)]);
        let v2 = FiniteFieldVector::new(vec![FieldElement::new(3, 5), FieldElement::new(4, 5)]);
        v1.add(&v2);
        assert_eq!(v1, FiniteFieldVector::new(vec![FieldElement::new(4, 5), FieldElement::new(1, 5)])); // 1+3 mod 5 = 4, 2+4 mod 5 = 1
    }

    #[test]
    fn test_scalar_multiplication() {
        let mut v = FiniteFieldVector::new(vec![FieldElement::new(4, 7), FieldElement::new(2, 7)]);
        let scalar = FieldElement::new(3, 7);
        v.scalar_mul(scalar);
        assert_eq!(v, FiniteFieldVector::new(vec![FieldElement::new(5, 7), FieldElement::new(6, 7)])); // 3*4 mod 7 = 5, 3*2 mod 7 = 6
    }

    #[test]
    fn test_dot_product() {
        let v1 = FiniteFieldVector::new(vec![FieldElement::new(1, 7), FieldElement::new(2, 7)]);
        let v2 = FiniteFieldVector::new(vec![FieldElement::new(3, 7), FieldElement::new(4, 7)]);
        let dot_product = v1.dot(&v2);
        assert_eq!(dot_product, FieldElement::new(11, 7)); // 1*3 + 2*4 mod 7 = 11 mod 7 = 4
    }
}