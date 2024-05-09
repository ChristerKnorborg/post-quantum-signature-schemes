#include <stdio.h>
#include <stdlib.h>

#ifdef __ARM_NEON
#include <arm_neon.h>
#endif /* __ARM_NEON */

#define __STDC_FORMAT_MACROS


void mul_add_64_bitsliced_m_vec(u_int32_t *input, u_int32_t input_start, u_int8_t nibble, u_int32_t *acc, u_int32_t acc_start) {

        // In0 is x^0 and x^1 and in1 is x^2 and x^3
        uint32x4_t in0 = vld1q_u32(&input[input_start]);
        uint32x4_t in1 = vld1q_u32(&input[input_start+4]);

        uint32x4_t acc0 = vld1q_u32(&acc[acc_start]);
        uint32x4_t acc1 = vld1q_u32(&acc[acc_start+4]);

        // The lookup vector to be used for the multiplications
        uint8x16_t mul_tbl = {0x0, 0x13, 0x26, 0x35, 0x4c, 0x5f, 0x6a, 0x79, 0x98, 0x8b, 0xbe, 0xad, 0xd4, 0xc7, 0xf2, 0xe1};

        uint32x4_t inshuf2_1 = vcombine_u32(vget_low_u64(in1), vget_high_u64(in1));
        uint32x4_t inshuf2_2 = vcombine_u32(vget_low_u64(in0), vget_high_u64(in0));

        uint32x4_t inshuf1_1 = vcombine_u32(vget_high_u32(in0), vget_low_u32(in0));
        uint32x4_t inshuf1_2 = vcombine_u32(vget_high_u32(in1), vget_low_u32(in1));
        
        uint32x4_t inshuf3_1 = vcombine_u32(vget_low_u64(inshuf1_2), vget_high_u64(inshuf1_2));
        uint32x4_t inshuf3_2 = vcombine_u32(vget_low_u64(inshuf1_1), vget_high_u64(inshuf1_1));


        const uint64x2_t mask1_1 = {1, 16};
        const uint64x2_t mask1_2 = {16, 16};

        const uint64x2_t mask2_1 = {128, 32};
        const uint64x2_t mask2_2 = {8, 32};

        const uint64x2_t mask3_1 = {64, 4};
        const uint64x2_t mask3_2 = {64, 64};

        const uint64x2_t mask4_1 = {32, 8};
        const uint64x2_t mask4_2 = {32, 128};

        uint8x16_t aaaa = vqtbl1q_u8(mul_tbl, vdupq_n_u8(nibble));

        acc0 ^= in0 & vceqq_u64(aaaa & mask1_1, mask1_1);
        acc1 ^= in1 & vceqq_u64(aaaa & mask1_2, mask1_2);

        acc0 ^= inshuf1_1 & vceqq_u64(aaaa & mask2_1, mask2_1);
        acc1 ^= inshuf1_2 & vceqq_u64(aaaa & mask2_2, mask2_2);

        acc0 ^= inshuf2_1 & vceqq_u64(aaaa & mask3_1, mask3_1);
        acc1 ^= inshuf2_2 & vceqq_u64(aaaa & mask3_2, mask3_2);

        acc0 ^= inshuf3_1 & vceqq_u64(aaaa & mask4_1, mask4_1);
        acc1 ^= inshuf3_2 & vceqq_u64(aaaa & mask4_2, mask4_2);

        vst1q_u32(&acc[acc_start], acc0);
        vst1q_u32(&acc[acc_start + 4], acc1);
}

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