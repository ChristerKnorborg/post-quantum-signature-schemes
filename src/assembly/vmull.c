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


void upper(u_int8_t *matrix, int o) {
    // Iterate over everything above the diagonal
    for (int i = 0 ; i < o ; i++)  {
        for (int j = i + 1 ; j < o ; j++) {  // Start from i + 1
            int idx_ij = i * o + j;  // Calculate the flat index for the element at [i][j]
            int idx_ji = j * o + i;  // Calculate the flat index for the element at [j][i]

            // GF(16) addition is the same as XOR
            matrix[idx_ij] ^= matrix[idx_ji];
            matrix[idx_ji] = 0; // Set the lower part to zero
        }
    }
}

void xor(u_int8_t *result, u_int8_t *a, u_int8_t *b, int n) {
    int i;

    for (i = 0; i < ((n) & ~7); i+=8) {
        uint8x8_t va = vld1_u8(&a[i]); // Load 8 elements from array a
        uint8x8_t vb = vld1_u8(&b[i]); // Load 8 elements from array b

        uint8x8_t res = veor_u8(va, vb); // Perform widening multiplication

        vst1_u8(&result[i], res);
    }    

     // Handle remaining elements (if any)
    for (; i < n; ++i) {
        result[i] ^= (a[i] ^ b[i]);
    }
}


void xor_registers(u_int8_t *a, u_int8_t *b, int n) {
    int i;

    for (i = 0; i < (n & ~7); i += 8) {
        uint8x8_t va = vld1_u8(&a[i]); // Load 8 elements from array a
        uint8x8_t vb = vld1_u8(&b[i]); // Load 8 elements from array b

        uint8x8_t res = veor_u8(va, vb); // Perform XOR

        vst1_u8(&a[i], res); // Store the result back into array a
    }

    // Handle remaining elements (if any)
    for (; i < n; ++i) {
        a[i] ^= b[i];
    }
}

void innter_product_P1_O(u_int8_t *result, u_int8_t *a, u_int8_t *b, int param_v, int param_o) {
    int res_idx = 0;
    int i;
    int j;

    uint8x8_t constant_vec_xor = vdup_n_u8((u_int8_t) 0xf0); 
    uint8x8_t constant_vec_and = vdup_n_u8((u_int8_t) 0x0f); 

    for( j = 0 ; j < param_o ; j++) {

        for (i = 0; i < ((param_v) & ~7); i+=8) {
            uint8x8_t va = vld1_u8(&a[i]); 
            poly8x8_t poly_va = vreinterpret_u8_p8(va); 



            uint8x8_t vb = vld1_u8(&b[(j*param_v)+i]); 
            poly8x8_t poly_vb = vreinterpret_u8_p8(vb);
            
            // Perform polynomial multiplication, widening result to 16x8 (every 2. bytes is 0 as 2^4 polynomials does flow onto next byte)
            poly16x8_t res = vmull_p8(poly_va, poly_vb); 

            // Remove every second byte where no coefficients are stored
            uint8x8_t narrowed_res = vqmovn_u16(res); 
                    
            // Perform reduction of x^6, x^5 and x^4 using irreducible polynomial x^4 = x + 1.
            // In the field, x^6 ≡ x^3 + x^2, x^5 ≡ x^2 + x and x^4 ≡ x + 1. Therefore shift with 3 and 4. 
            uint8x8_t high_coeffs = vand_u8(narrowed_res, constant_vec_xor);
            uint8x8_t high_coeffs_shift_3 = vshr_n_u8(high_coeffs, 3); 
            uint8x8_t high_coeffs_shift_4 = vshr_n_u8(high_coeffs, 4); 

            // Add the reduced coefficients to the lower coefficients.
            uint8x8_t reduced_coeffs_vec = veor_u8(high_coeffs_shift_3, high_coeffs_shift_4);  
            uint8x8_t res_before_and = veor_u8(narrowed_res, reduced_coeffs_vec);
            uint8x8_t final_res = vand_u8(res_before_and, constant_vec_and);

            // Add all 8 terms to a single byte to get the final inner product by horizontal XOR reduction
            uint8x8_t xor_fold1 = veor_u8(final_res, vext_u8(final_res, final_res, 4));
            uint8x8_t xor_fold2 = veor_u8(xor_fold1, vext_u8(xor_fold1, xor_fold1, 2));
            uint8x8_t xor_fold3 = veor_u8(xor_fold2, vext_u8(xor_fold2, xor_fold2, 1));

            // Extract the reduced byte and XOR into the result
            uint8_t reduced_byte = vget_lane_u8(xor_fold3, 0);
            result[res_idx] ^= reduced_byte;

            // Handle remaining elements (if any)
            for (; i < param_v; ++i) {
                u_int8_t res;
                res =  (a[i] & 1)*b[i]; // Multiply by x^0
                res ^= (a[i] & 2)*b[i]; // Multiply by x^1
                res ^= (a[i] & 4)*b[i]; // Multiply by x^2
                res ^= (a[i] & 8)*b[i]; // Multiply by x^3
                
                u_int8_t first_4_bits = res & 0xf0;
                u_int8_t overflow_bits = (first_4_bits >> 4) ^ (first_4_bits >> 3);  
                result[res_idx] ^= (res ^ overflow_bits) & 0x0f;
            }
            res_idx++;
        }
    }
}

void inner_product(u_int8_t *result, u_int8_t *a, u_int8_t *b, int n) {
    int i;

    uint8x8_t constant_vec_xor = vdup_n_u8((u_int8_t) 0xf0); 
    uint8x8_t constant_vec_and = vdup_n_u8((u_int8_t) 0x0f); 

    for (i = 0; i < (n & ~7); i+=8) {

        // Load 8 elements from array a and b respectively
        uint8x8_t va = vld1_u8(&a[i]); 
        uint8x8_t vb = vld1_u8(&b[i]); 

        // Change to polynomial multiplication
        poly8x8_t poly_va = vreinterpret_u8_p8(va); 
        poly8x8_t poly_vb = vreinterpret_u8_p8(vb);
        
        // Perform polynomial multiplication, widening result to 16x8 (every 2. bytes is 0 as 2^4 polynomials does flow onto next byte)
        poly16x8_t res = vmull_p8(poly_va, poly_vb); 

        // Remove every second byte where no coefficients are stored
        uint8x8_t narrowed_res = vqmovn_u16(res); 
                
                
        // Perform reduction of x^6, x^5 and x^4 using irreducible polynomial x^4 = x + 1.
        // In the field, x^6 ≡ x^3 + x^2, x^5 ≡ x^2 + x and x^4 ≡ x + 1. Therefore shift with 3 and 4. 
        uint8x8_t high_coeffs = vand_u8(narrowed_res, constant_vec_xor);
        uint8x8_t high_coeffs_shift_3 = vshr_n_u8(high_coeffs, 3); 
        uint8x8_t high_coeffs_shift_4 = vshr_n_u8(high_coeffs, 4); 

        // Add the reduced coefficients to the lower coefficients.
        uint8x8_t reduced_coeffs_vec = veor_u8(high_coeffs_shift_3, high_coeffs_shift_4);  
        uint8x8_t res_before_and = veor_u8(narrowed_res, reduced_coeffs_vec);
        uint8x8_t final_res = vand_u8(res_before_and, constant_vec_and);

        // Add all 8 terms to a single byte to get the final inner product by horizontal XOR reduction
        uint8x8_t xor_fold1 = veor_u8(final_res, vext_u8(final_res, final_res, 4));
        uint8x8_t xor_fold2 = veor_u8(xor_fold1, vext_u8(xor_fold1, xor_fold1, 2));
        uint8x8_t xor_fold3 = veor_u8(xor_fold2, vext_u8(xor_fold2, xor_fold2, 1));

        // Extract the reduced byte and XOR into the result
        uint8_t reduced_byte = vget_lane_u8(xor_fold3, 0);
        *result ^= reduced_byte;
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
        *result ^= (res ^ overflow_bits) & 0x0f;
    }
}

void vmull_values_test(u_int8_t *result, u_int8_t *a, u_int8_t *b, int n, int cols_b) {
    int i;

    uint8x8_t constant_vec_xor = vdup_n_u8((u_int8_t) 0xf0); 
    uint8x8_t constant_vec_and = vdup_n_u8((u_int8_t) 0x0f); 
    for (int k = 0; k < cols_b; k++) { 

        for (i = 0; i < ((n) & ~7); i+=8) {
            uint8x8_t va = vld1_u8(&a[i*n]); // Load 8 elements from array a
            poly8x8_t poly_va = vreinterpret_u8_p8(va);
            uint8x8_t vb = vld1_u8(&b[(k*n)+i]); // Load 8 elements from array b
            poly8x8_t poly_vb = vreinterpret_u8_p8(vb);

            poly16x8_t res = vmull_p8(poly_va, poly_vb); // Perform widening multiplication

            uint8x8_t narrowed_res = vqmovn_u16(res); // Narrowing conversion to uint8x8_t
                    
            uint8x8_t high_coeffs = vand_u8(narrowed_res, constant_vec_xor);

            uint8x8_t high_coeffs_shift_3 = vshr_n_u8(high_coeffs, 3);
            uint8x8_t high_coeffs_shift_4 = vshr_n_u8(high_coeffs, 4);

            uint8x8_t reduced_coeffs_vec = veor_u8(high_coeffs_shift_3, high_coeffs_shift_4);
            uint8x8_t res_before_and = veor_u8(narrowed_res, reduced_coeffs_vec);

            uint8x8_t final_res = vand_u8(res_before_and, constant_vec_and);


            // Horizontal XOR reduction of 'final_res' to a single byte
            uint8x8_t xor_fold1 = veor_u8(final_res, vext_u8(final_res, final_res, 4));
            uint8x8_t xor_fold2 = veor_u8(xor_fold1, vext_u8(xor_fold1, xor_fold1, 2));
            uint8x8_t xor_fold3 = veor_u8(xor_fold2, vext_u8(xor_fold2, xor_fold2, 1));

            // Extract the reduced byte
            uint8_t reduced_byte = vget_lane_u8(xor_fold3, 0);

            // XOR the reduced byte into the result
            *&result[k] ^= reduced_byte;

        }

        // Handle remaining elements (if any)
        for (; i < n; ++i) {
            u_int8_t res;
            res =  (a[i*n] & 1)*b[i+(k*n)]; // Multiply by x^0
            res ^= (a[i*n] & 2)*b[i+(k*n)]; // Multiply by x^1
            res ^= (a[i*n] & 4)*b[i+(k*n)]; // Multiply by x^2
            res ^= (a[i*n] & 8)*b[i+(k*n)]; // Multiply by x^3
            
            u_int8_t first_4_bits = res & 0xf0;
            u_int8_t overflow_bits = (first_4_bits >> 4) ^ (first_4_bits >> 3);  
            *&result[k] ^= (res ^ overflow_bits) & 0x0f;
        }
    }
}


void p1_o(u_int8_t *result, u_int8_t *a, u_int8_t *b, int rows_a, int cols_a, int cols_b) {

    for (int r = 0; r < rows_a; r++) {
        for (int k = 0; k < cols_b; k++) { 
            inner_product(&result[r * cols_b + k], &a[r*rows_a], &b[k*rows_a], rows_a);
        } 
    }
}

// Compute 
void o_transposed_mul_p2(u_int8_t *result, u_int8_t *o, u_int8_t *p2, int o_rows, int o_cols, int p2_rows, int p2_cols){

    // Temporary vector to hold one column of P2
    u_int8_t *p2_col = (u_int8_t *)malloc(p2_rows * sizeof(u_int8_t));

    // Iterate over each row of O (each representing a column of original O)
    for (int r = 0; r < o_rows; r++) {
        for (int c = 0; c < p2_cols; c++) {

            // Extract the column vector from P2
            for (int i = 0; i < p2_rows; i++) {
                p2_col[i] = p2[i * p2_cols + c];
            }
            
            // Perform vector multiplication using vmull_values
            inner_product(&result[r * p2_cols + c], &o[r * o_cols], p2_col, o_cols); // o_cols is equivalent to p2_rows
        }
    }
}

void calculate_p3(u_int8_t *result, u_int8_t *o, u_int8_t *p1, u_int8_t *p2, int param_v, int param_o, int param_m) {

    u_int8_t *temp =  (u_int8_t *)malloc(param_v * param_o * param_m * sizeof(u_int8_t));
    
    for (int matrix = 0; matrix < param_m; matrix++) {

        // Calculate P1*O
        p1_o(&temp[matrix * param_o*param_v], &p1[matrix * param_v*param_v], o, param_v, param_v, param_o); // Change to o_cols (now rows are given)
        
        // for (int i = 0 ; i < param_v ; i++) {
        //     innter_product_P1_O(&temp[matrix * (i*param_o)], &p1[matrix * (i*param_v)], o, param_v, param_o);
        // } 
    }

    
    // Calculate P1*O + P2
    u_int8_t *temp2 =  (u_int8_t *)malloc(param_v * param_o * param_m * sizeof(u_int8_t));
    xor(temp2, temp, p2, param_v*param_o*param_m);
    // XOR_registers(temp, p2, param_v*param_o);
 
    for (int matrix = 0; matrix < param_m; matrix++) {
        // Calculate O^T * (P1*O + P2)
        o_transposed_mul_p2(&result[matrix * param_o * param_o], o, &temp2[matrix * param_o*param_v], param_o, param_v, param_v, param_o);
        upper(&result[matrix * param_o*param_o], param_o);
    
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
}*/ 

