#include <stdio.h>
#include <stdlib.h>

#ifdef __ARM_NEON
#include <arm_neon.h>
#endif /* __ARM_NEON */

#define __STDC_FORMAT_MACROS


/* void print_array(u_int8_t *arr, int n) {
    printf("[");
    for (int i = 0; i < n; i++) {
        printf("%c", arr[i]);
        if (i < n - 1) {
            printf(", ");
        }
    }
    printf("]\n");
} *///188 227


void vmull_values(u_int8_t *result, u_int8_t *a, u_int8_t *b, int n) {
    int i;


    for (i = 0; i < (n & ~7); i+=8) {
        uint8x8_t va = vld1_u8(&a[i]); // Load 8 elements from array a
        poly8x8_t poly_va = vreinterpret_u8_p8(va);
        uint8x8_t vb = vld1_u8(&b[i]); // Load 8 elements from array b
        poly8x8_t poly_vb = vreinterpret_u8_p8(vb);

        poly16x8_t res = vmull_p8(poly_va, poly_vb); // Perform widening multiplication

        uint8x8_t narrowed_res = vqmovn_u16(res); // Narrowing conversion to uint8x8_t
        
        

        // Find the highest bits to reduce.
        uint8x8_t constant_vec_xor = vdup_n_u8((u_int8_t) 0xf0); 
        uint8x8_t high_coeffs = vand_u8(narrowed_res, constant_vec_xor);

        uint8x8_t high_coeffs_shift_3 = vshr_n_u8(high_coeffs, 3);
        uint8x8_t high_coeffs_shift_4 = vshr_n_u8(high_coeffs, 4);

        uint8x8_t reduced_coeffs_vec = veor_u8(high_coeffs_shift_3, high_coeffs_shift_4);
        uint8x8_t res_before_and = veor_u8(narrowed_res, reduced_coeffs_vec);

        uint8x8_t constant_vec_and = vdup_n_u8((u_int8_t) 0x0f); 
        uint8x8_t final_res = vand_u8(res_before_and, constant_vec_and);


        /* uint64_t high_coeffs = narrowed_res & 0x7070707070707070; // Extract coefficients (x^6, x^5, x^4) from every 8 polynomial. E.g. use and with 01110000...01110000

        uint64_t reduced_coeffs = (high_coeffs >> 3) ^ (high_coeffs >> 4);



        // Add reduced coefficients to lower 4 bits in each 8 and remove pre reduced coefficients
        result = (narrowed_res ^ reduced_coeffs) & 0x0F0F0F0F0F0F0F0F; */


         // Load the result[i] into a uint8x8_t vector
        uint8x8_t result_vec = vld1_u8(&result[i]);

        // Perform XOR operation between result[i] and final_res
        uint8x8_t xor_result = veor_u8(result_vec, final_res);

        // Store the result back into result[i]
        vst1_u8(&result[i], xor_result);


        



    }
    // Handle remaining elements (if any)
    for (; i < n; ++i) {
        u_int8_t res;
        res =  (a[i] & 1)*b[i]; // Multiply by x^0
        res ^= (a[i] & 2)*b[i]; // Multiply by x^1
        res ^= (a[i] & 4)*b[i]; // Multiply by x^2
        res ^= (a[i] & 8)*b[i]; // Multiply by x^3
        
        u_int8_t first_4_bits = res & 0xf0;
        u_int8_t overflow_bits = (first_4_bits >> 4) ^ (first_4_bits >> 3);  
        result[i] ^= (res ^ overflow_bits) & 0x0f;
    }
}

/* void test_neon() {
    // Test cases
    u_int8_t a[] = {2, 3, 3, 4, 10};
    u_int8_t b[] = {5, 6, 10, 8, 5};
    u_int8_t result[5];
    int n = 5;

    printf("Array a: ");
    print_array(a, n);
    printf("Array b: ");
    print_array(b, n);

    vmull_values(result, a, b, n);

    printf("Result: ");
    print_f(result, n);
} */

