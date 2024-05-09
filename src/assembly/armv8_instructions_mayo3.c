#include <stdio.h>
#include <stdlib.h>

#ifdef __ARM_NEON
#include <arm_neon.h>
#endif /* __ARM_NEON */

#define __STDC_FORMAT_MACROS
#define O_MAYO_3 8
#define V_MAYO_3 58
#define K_MAYO_3 9


void mul_add_96_bitsliced_m_vec(u_int32_t *input, u_int32_t input_start, u_int8_t nibble, u_int32_t *acc, u_int32_t acc_start) {

        uint32x4_t in0 = vld1q_u32(&input[input_start]);
        uint32x4_t in1 = vld1q_u32(&input[input_start+4]);
        uint32x4_t in2 = vld1q_u32(&input[input_start+8]);

        uint32x4_t acc0 = vld1q_u32(&acc[acc_start]);
        uint32x4_t acc1 = vld1q_u32(&acc[acc_start+4]);
        uint32x4_t acc2 = vld1q_u32(&acc[acc_start+8]);

        uint8x16_t tbl_a0 = {0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255};
        uint8x16_t tbl_a1 = {0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255};
        uint8x16_t tbl_a2 = {0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 0, 255, 255, 255, 255};
        uint8x16_t tbl_a3 = {0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255};

        uint8x16_t aa = vdupq_n_u8(nibble);
        uint8x16_t a0 = vqtbl1q_u8(tbl_a0, aa);
        uint8x16_t a1 = vqtbl1q_u8(tbl_a1, aa);
        uint8x16_t a2 = vqtbl1q_u8(tbl_a2, aa);
        uint8x16_t a3 = vqtbl1q_u8(tbl_a3, aa);

        uint8x16_t inrot[3];
        inrot[0] = vextq_u32(in0, in1, 3);
        inrot[1] = vextq_u32(in1, in2, 3);
        inrot[2] = vextq_u32(in2, in0, 3);

        uint8x16_t inrot2[3];
        inrot2[0] = vextq_u32(in1, in2, 2);
        inrot2[1] = vextq_u32(in2, in0, 2);
        inrot2[2] = vextq_u32(in0, in1, 2);

        uint8x16_t inrot3[3];
        inrot3[0] = vextq_u32(in2, in0, 1);
        inrot3[1] = vextq_u32(in0, in1, 1);
        inrot3[2] = vextq_u32(in1, in2, 1);

        acc0 ^= a0 & in0;
        acc1 ^= a0 & in1;
        acc2 ^= a0 & in2;

        const uint32x4_t mask1 = {0, 0, 0, -1};
        const uint32x4_t mask2 = {-1, -1, 0, 0};
        const uint32x4_t mask3 = {-1, 0, 0, 0};


        acc0 ^= a1 & (inrot3[0] ^ (inrot2[0] & mask1));
        acc1 ^= a1 & (inrot3[1] ^ (inrot2[1] & mask2));
        acc2 ^= a1 & (inrot3[2]);

        acc0 ^= a2 & (inrot2[0] ^ (inrot[0] & mask1));
        acc1 ^= a2 & (inrot2[1] ^ (inrot[1]));
        acc2 ^= a2 & (inrot2[2] ^ (inrot[2] & mask3));

        acc0 ^= a3 & (inrot[0] ^ (in0 & mask1));
        acc1 ^= a3 & (inrot[1] ^ (in1));
        acc2 ^= a3 & (inrot[2] ^ (in2));

        vst1q_u32(&acc[acc_start], acc0);
        vst1q_u32(&acc[acc_start+4], acc1);
        vst1q_u32(&acc[acc_start+8], acc2);
} 