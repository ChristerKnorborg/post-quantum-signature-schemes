#include <stdio.h>
#include <stdlib.h>

#ifdef __ARM_NEON
#include <arm_neon.h>
#endif /* __ARM_NEON */

#define __STDC_FORMAT_MACROS


void mul_add_128_bitsliced_m_vec(u_int32_t *input, u_int32_t input_start, u_int8_t nibble, u_int32_t *acc, u_int32_t acc_start) {

        uint32x4_t in0_1 = vld1q_u32(&input[input_start]);
        uint32x4_t in0_2 = vld1q_u32(&input[input_start+4]);
        uint32x4_t in1_1 = vld1q_u32(&input[input_start+8]);
        uint32x4_t in1_2 = vld1q_u32(&input[input_start+12]);

        uint32x4_t acc0_1 = vld1q_u32(&acc[acc_start]);
        uint32x4_t acc0_2 = vld1q_u32(&acc[acc_start+4]);
        uint32x4_t acc1_1 = vld1q_u32(&acc[acc_start+8]);
        uint32x4_t acc1_2 = vld1q_u32(&acc[acc_start+12]);

        uint8x16_t tbl_a0 = {0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255};
        uint8x16_t tbl_a1 = {0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255};
        uint8x16_t tbl_a2 = {0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 0, 255, 255, 255, 255};
        uint8x16_t tbl_a3 = {0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255};

        uint8x16_t aa = vdupq_n_u8(nibble);
        uint8x16_t a0 = vqtbl1q_u8(tbl_a0, aa);
        uint8x16_t a1 = vqtbl1q_u8(tbl_a1, aa);
        uint8x16_t a2 = vqtbl1q_u8(tbl_a2, aa);
        uint8x16_t a3 = vqtbl1q_u8(tbl_a3, aa);


        uint32x4_t xz_1 = in0_1 ^ in1_2;
        uint32x4_t xz_2 = in0_2 ^ in1_1;

        uint32x4_t yy_1 = in1_1 ^ in1_2;
        uint32x4_t yy_2 = in1_2 ^ in1_1;

        // Degree 0 term of a 
        acc0_1 ^= a0 & in0_1;
        acc0_2 ^= a0 & in0_2;

        acc1_1 ^= a0 & in1_1;
        acc1_2 ^= a0 & in1_2;

        // Degree 1 term of a
        acc0_1 ^= a1 & in1_2;
        acc0_2 ^= a1 & xz_1;

        acc1_1 ^= a1 & in0_2;
        acc1_2 ^= a1 & in1_1;

        // Degree 2 term of a
        acc0_1 ^= a2 & in1_1;
        acc0_2 ^= a2 & yy_2;

        acc1_1 ^= a2 & xz_1;
        acc1_2 ^= a2 & in0_2;

        // Degree 3 term of a
        acc0_1 ^= a3 & in0_2;
        acc0_2 ^= a3 & xz_2;

        acc1_1 ^= a3 & yy_1;
        acc1_2 ^= a3 & xz_1;

        vst1q_u32(&acc[acc_start], acc0_1);
        vst1q_u32(&acc[acc_start+4], acc0_2);
        vst1q_u32(&acc[acc_start+8], acc1_1);
        vst1q_u32(&acc[acc_start+12], acc1_2);
} 