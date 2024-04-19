#include <stdio.h>
#include <stdlib.h>

#ifdef __ARM_NEON
#include <arm_neon.h>
#endif /* __ARM_NEON */

#define __STDC_FORMAT_MACROS


/*void mul_add_bitsliced_m_vec(uint64_t *input, uint32_t input_start, uint8_t nibble, uint64_t *acc, uint32_t acc_start) {
    // Assume each operation now uses 64-bit vectors to hold the data

    int u_32_per_term = 2;

    uint64x1_t in0 = vld1_u64(&input[input_start]);
    uint64x1_t in1 = vld1_u64(&input[input_start + u_32_per_term]);
    uint64x1_t in2 = vld1_u64(&input[input_start + 2 * u_32_per_term]);
    uint64x1_t in3 = vld1_u64(&input[input_start + 3 * u_32_per_term]);

    uint64x1_t acc0 = vld1_u64(&acc[acc_start]);
    uint64x1_t acc1 = vld1_u64(&acc[acc_start + u_32_per_term]);
    uint64x1_t acc2 = vld1_u64(&acc[acc_start + 2 * u_32_per_term]);
    uint64x1_t acc3 = vld1_u64(&acc[acc_start + 3 * u_32_per_term]);

    // Correctly initializing using vdup_n_u64
    uint64x1_t n0 = vdup_n_u64((nibble & 1) ? 0xFFFFFFFFFFFFFFFF : 0);
    uint64x1_t n1 = vdup_n_u64((nibble & 2) ? 0xFFFFFFFFFFFFFFFF : 0);
    uint64x1_t n2 = vdup_n_u64((nibble & 4) ? 0xFFFFFFFFFFFFFFFF : 0);
    uint64x1_t n3 = vdup_n_u64((nibble & 8) ? 0xFFFFFFFFFFFFFFFF : 0);

    // Compute bitwise XORs directly in 64-bit registers
    uint64x1_t a = veor_u64(in0, in3);
    uint64x1_t b = veor_u64(in3, in2);
    uint64x1_t c = veor_u64(in2, in1);

    // Apply bitwise operations and store back to acc
    acc0 = veor_u64(acc0, vand_u64(n0, in0));
    acc1 = veor_u64(acc1, vand_u64(n0, in1));
    acc2 = veor_u64(acc2, vand_u64(n0, in2));
    acc3 = veor_u64(acc3, vand_u64(n0, in3));

    acc0 = veor_u64(acc0, vand_u64(n1, in3));
    acc1 = veor_u64(acc1, vand_u64(n1, a));
    acc2 = veor_u64(acc2, vand_u64(n1, in1));
    acc3 = veor_u64(acc3, vand_u64(n1, in2));

    acc0 = veor_u64(acc0, vand_u64(n2, in2));
    acc1 = veor_u64(acc1, vand_u64(n2, b));
    acc2 = veor_u64(acc2, vand_u64(n2, a));
    acc3 = veor_u64(acc3, vand_u64(n2, in1));

    acc0 = veor_u64(acc0, vand_u64(n3, in1));
    acc1 = veor_u64(acc1, vand_u64(n3, c));
    acc2 = veor_u64(acc2, vand_u64(n3, b));
    acc3 = veor_u64(acc3, vand_u64(n3, a));

    // Store results back to the acc array using 64-bit stores
    vst1_u64(&acc[acc_start], acc0);
    vst1_u64(&acc[acc_start + u_32_per_term], acc1);
    vst1_u64(&acc[acc_start + 2 * u_32_per_term], acc2);
    vst1_u64(&acc[acc_start + 3 * u_32_per_term], acc3);
}
 */
void mul_add_bitsliced_m_vec_mayo1(u_int32_t *input, u_int32_t input_start, u_int8_t nibble1, u_int8_t nibble2, u_int32_t *acc, u_int32_t acc_start) {
    
        int u_32_per_term = 2;

        uint32x2_t in0_0 = vld1_u32(&input[input_start]);
        uint32x2_t in1_0 = vld1_u32(&input[input_start + u_32_per_term]);
        uint32x2_t in2_0 = vld1_u32(&input[input_start + 2 * u_32_per_term]);
        uint32x2_t in3_0 = vld1_u32(&input[input_start + 3 * u_32_per_term]);

        uint32x2_t in0_1 = vld1_u32(&input[8 + input_start]);
        uint32x2_t in1_1 = vld1_u32(&input[8 + (input_start + u_32_per_term)]);
        uint32x2_t in2_1 = vld1_u32(&input[8 + (input_start + 2 * u_32_per_term)]);
        uint32x2_t in3_1 = vld1_u32(&input[8 + (input_start + 3 * u_32_per_term)]);

         // Convert to uint32x4_t for processing
        uint32x4_t in0 = vcombine_u32(in0_0, in0_1);
        uint32x4_t in1 = vcombine_u32(in1_0, in1_1);
        uint32x4_t in2 = vcombine_u32(in2_0, in2_1);
        uint32x4_t in3 = vcombine_u32(in3_0, in3_1);

        // Load accumulators as uint32x2_t
        uint32x2_t acc0_low = vld1_u32(&acc[acc_start]);
        uint32x2_t acc1_low = vld1_u32(&acc[acc_start + u_32_per_term]);
        uint32x2_t acc2_low = vld1_u32(&acc[acc_start + 2 * u_32_per_term]);
        uint32x2_t acc3_low = vld1_u32(&acc[acc_start + 3 * u_32_per_term]);

        // Convert to uint32x4_t for processing
        uint32x4_t acc0 = vcombine_u32(acc0_low, acc0_low);
        uint32x4_t acc1 = vcombine_u32(acc1_low, acc1_low);
        uint32x4_t acc2 = vcombine_u32(acc2_low, acc2_low);
        uint32x4_t acc3 = vcombine_u32(acc3_low, acc3_low);


        // Nibble mask extraction
        uint32x2_t n0_1 = vdup_n_u32((nibble1 & 1) ? 0xFFFFFFFF : 0);
        uint32x2_t n1_1 = vdup_n_u32((nibble1 & 2) ? 0xFFFFFFFF : 0);
        uint32x2_t n2_1 = vdup_n_u32((nibble1 & 4) ? 0xFFFFFFFF : 0);
        uint32x2_t n3_1 = vdup_n_u32((nibble1 & 8) ? 0xFFFFFFFF : 0);

        uint32x2_t n0_2 = vdup_n_u32((nibble2 & 1) ? 0xFFFFFFFF : 0);
        uint32x2_t n1_2 = vdup_n_u32((nibble2 & 2) ? 0xFFFFFFFF : 0);
        uint32x2_t n2_2 = vdup_n_u32((nibble2 & 4) ? 0xFFFFFFFF : 0);
        uint32x2_t n3_2 = vdup_n_u32((nibble2 & 8) ? 0xFFFFFFFF : 0);

        // Combining vectors
        uint32x4_t n0 = vcombine_u32(n0_1, n0_2); // combines n0 and n1
        uint32x4_t n1 = vcombine_u32(n1_1, n1_2); // combines n2 and n3
        uint32x4_t n2 = vcombine_u32(n2_1, n2_2); // combines n0_2 and n1_2
        uint32x4_t n3 = vcombine_u32(n3_1, n3_2); // combines n2_2 and n3_2

        // Compute bitwise XORs
        uint32x4_t a = veorq_u32(in0, in3);
        uint32x4_t b = veorq_u32(in3, in2);
        uint32x4_t c = veorq_u32(in2, in1);

        // x^0 terms
        acc0 = veorq_u32(acc0, vandq_u32(n0, in0));
        acc1 = veorq_u32(acc1, vandq_u32(n0, in1));
        acc2 = veorq_u32(acc2, vandq_u32(n0, in2));
        acc3 = veorq_u32(acc3, vandq_u32(n0, in3));

        // x^1 terms
        acc0 = veorq_u32(acc0, vandq_u32(n1, in3));
        acc1 = veorq_u32(acc1, vandq_u32(n1, a));
        acc2 = veorq_u32(acc2, vandq_u32(n1, in1));
        acc3 = veorq_u32(acc3, vandq_u32(n1, in2));

        // x^2 terms
        acc0 = veorq_u32(acc0, vandq_u32(n2, in2));
        acc1 = veorq_u32(acc1, vandq_u32(n2, b));
        acc2 = veorq_u32(acc2, vandq_u32(n2, a));
        acc3 = veorq_u32(acc3, vandq_u32(n2, in1));

        //x^3 terms
        acc0 = veorq_u32(acc0, vandq_u32(n3, in1));
        acc1 = veorq_u32(acc1, vandq_u32(n3, c));
        acc2 = veorq_u32(acc2, vandq_u32(n3, b));
        acc3 = veorq_u32(acc3, vandq_u32(n3, a));

        // Store results back to acc array as uint32x2_t
        vst1_u32(&acc[acc_start], veor_u32(vget_low_u32(acc0), vget_high_u32(acc0)));
        vst1_u32(&acc[acc_start + u_32_per_term], veor_u32(vget_low_u32(acc1), vget_high_u32(acc1)));
        vst1_u32(&acc[acc_start + 2 * u_32_per_term], veor_u32(vget_low_u32(acc2), vget_high_u32(acc2)));
        vst1_u32(&acc[acc_start + 3 * u_32_per_term], veor_u32(vget_low_u32(acc3), vget_high_u32(acc3)));
}

void mul_add_bitsliced_m_vec(u_int32_t *input, u_int32_t input_start, u_int8_t nibble, u_int32_t *acc, u_int32_t acc_start) {
    
        int u_32_per_term = 2;

        uint32x2_t in0 = vld1_u32(&input[input_start]);
        uint32x2_t in1 = vld1_u32(&input[input_start + u_32_per_term]);
        uint32x2_t in2 = vld1_u32(&input[input_start + 2 * u_32_per_term]);
        uint32x2_t in3 = vld1_u32(&input[input_start + 3 * u_32_per_term]);

        uint32x2_t acc0 = vld1_u32(&acc[acc_start]);
        uint32x2_t acc1 = vld1_u32(&acc[acc_start + u_32_per_term]);
        uint32x2_t acc2 = vld1_u32(&acc[acc_start + 2 * u_32_per_term]);
        uint32x2_t acc3 = vld1_u32(&acc[acc_start + 3 * u_32_per_term]);


        // Nibble mask extraction
        uint32x2_t n0 = vdup_n_u32((nibble & 1) ? 0xFFFFFFFF : 0);
        uint32x2_t n1 = vdup_n_u32((nibble & 2) ? 0xFFFFFFFF : 0);
        uint32x2_t n2 = vdup_n_u32((nibble & 4) ? 0xFFFFFFFF : 0);
        uint32x2_t n3 = vdup_n_u32((nibble & 8) ? 0xFFFFFFFF : 0);


        // Compute bitwise XORs
        uint32x2_t a = veor_u32(in0, in3);
        uint32x2_t b = veor_u32(in3, in2);
        uint32x2_t c = veor_u32(in2, in1);

        // x^0 terms
        acc0 = veor_u32(acc0, vand_u32(n0, in0));
        acc1 = veor_u32(acc1, vand_u32(n0, in1));
        acc2 = veor_u32(acc2, vand_u32(n0, in2));
        acc3 = veor_u32(acc3, vand_u32(n0, in3));

        // x^1 terms
        acc0 = veor_u32(acc0, vand_u32(n1, in3));
        acc1 = veor_u32(acc1, vand_u32(n1, a));
        acc2 = veor_u32(acc2, vand_u32(n1, in1));
        acc3 = veor_u32(acc3, vand_u32(n1, in2));

        // x^2 terms
        acc0 = veor_u32(acc0, vand_u32(n2, in2));
        acc1 = veor_u32(acc1, vand_u32(n2, b));
        acc2 = veor_u32(acc2, vand_u32(n2, a));
        acc3 = veor_u32(acc3, vand_u32(n2, in1));

        //x^3 terms
        acc0 = veor_u32(acc0, vand_u32(n3, in1));
        acc1 = veor_u32(acc1, vand_u32(n3, c));
        acc2 = veor_u32(acc2, vand_u32(n3, b));
        acc3 = veor_u32(acc3, vand_u32(n3, a));

        // Store results back to acc array
        vst1_u32(&acc[acc_start], acc0);  // Stores at index acc_start
        vst1_u32(&acc[acc_start + u_32_per_term], acc1); // Stores at index acc_start + 2
        vst1_u32(&acc[acc_start + 2 * u_32_per_term], acc2); // Stores at index acc_start + 4
        vst1_u32(&acc[acc_start + 3 * u_32_per_term], acc3); // Stores at index acc_start + 6
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

