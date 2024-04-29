#include <stdio.h>
#include <stdlib.h>

#ifdef __ARM_NEON
#include <arm_neon.h>
#endif /* __ARM_NEON */

#define __STDC_FORMAT_MACROS



void matrix_add(u_int8_t *result, u_int8_t *a, u_int8_t *b, int n) {
    int i;

    for (i = 0; i < ((n) & ~15); i+=16) {
        uint8x16_t va = vld1q_u8(&a[i]); // Load 16 elements from array a
        uint8x16_t vb = vld1q_u8(&b[i]); // Load 16 elements from array b

        uint8x16_t res = veorq_u8(va, vb); // Perform addition

        vst1q_u8(&result[i], res);
    }  

    for (i = 0; i < ((n) & ~7); i+=8) {
        uint8x8_t va = vld1_u8(&a[i]); // Load 8 elements from array a
        uint8x8_t vb = vld1_u8(&b[i]); // Load 8 elements from array b

        uint8x8_t res = veor_u8(va, vb); // Perform addition

        vst1_u8(&result[i], res);
    }    

     // Handle remaining elements (if any)
    for (; i < n; ++i) {
        result[i] ^= (a[i] ^ b[i]);
    }
}



void inner_product(u_int8_t *result, u_int8_t *a, u_int8_t *b, int n) {
    int i;

    uint8x8_t upper_extract = vdup_n_u8((u_int8_t) 0xf0); 
    uint8x8_t lower_extract = vdup_n_u8((u_int8_t) 0x0f); 

    for (i = 0; i < (n & ~7); i+=8) {

        // Load 8 elements from array a and b respectively
        uint8x8_t va = vld1_u8(&a[i]); 
        uint8x8_t vb = vld1_u8(&b[i]); 
        // Change to polynomial multiplication
        poly8x8_t poly_va = vreinterpret_u8_p8(va); 
        poly8x8_t poly_vb = vreinterpret_u8_p8(vb);
        
        // Perform polynomial multiplication, widening result to 16x8 (every 2. bytes is 0 as 2^4 polynomials does flow onto next byte)
        poly16x8_t poly_res = vmull_p8(poly_va, poly_vb); 

        // Remove every second byte where no coefficients are stored
        uint8x8_t narrowed = vqmovn_u16(poly_res); 
                
                
        // Perform reduction of x^6, x^5 and x^4 using irreducible polynomial x^4 = x + 1.
        // In the field, x^6 ≡ x^3 + x^2, x^5 ≡ x^2 + x and x^4 ≡ x + 1. Therefore shift with 3 and 4. 
        uint8x8_t high_coeffs = vand_u8(narrowed, upper_extract);
        uint8x8_t shifted_3 = vshr_n_u8(high_coeffs, 3); 
        uint8x8_t shifted_4 = vshr_n_u8(high_coeffs, 4); 

        // Add the reduced coefficients to the lower coefficients.
        uint8x8_t reduced_coeffs = veor_u8(shifted_3, shifted_4);  
        uint8x8_t added_coeffs = veor_u8(narrowed, reduced_coeffs);
        uint8x8_t res = vand_u8(added_coeffs, lower_extract);

        // Add all 8 terms to a single byte to get the final inner product by horizontal XOR sum
        uint8x8_t fold1 = veor_u8(res, vext_u8(res, res, 4));
        uint8x8_t fold2 = veor_u8(fold1, vext_u8(fold1, fold1, 2));
        uint8x8_t fold3 = veor_u8(fold2, vext_u8(fold2, fold2, 1));

        // Extract the reduced byte and XOR into the result
        uint8_t reduced_byte = vget_lane_u8(fold3, 0);
        *result ^= reduced_byte;
        }

    // Handle remaining elements (if any) using standard mul approch
    for (; i < n; ++i) {
        u_int8_t res;
        res =  (a[i] & 1)*b[i]; // Multiply by x^0
        res ^= (a[i] & 2)*b[i]; // Multiply by x^1
        res ^= (a[i] & 4)*b[i]; // Multiply by x^2
        res ^= (a[i] & 8)*b[i]; // Multiply by x^3
        
        u_int8_t first_4_bits = res & 0xf0;
        u_int8_t overflow_bits = (first_4_bits >> 4) ^ (first_4_bits >> 3);  
        *result ^= (res ^ overflow_bits) & 0x0f;
    }
}

