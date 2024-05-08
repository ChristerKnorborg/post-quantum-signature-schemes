#include <stdio.h>
#include <stdlib.h>

#ifdef __ARM_NEON
#include <arm_neon.h>
#endif /* __ARM_NEON */

#define __STDC_FORMAT_MACROS
#define O_MAYO_1 8
#define V_MAYO_1 58
#define K_MAYO_1 9

#define O_MAYO_2 18
#define V_MAYO_2 60


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