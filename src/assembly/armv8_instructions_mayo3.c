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

        uint32x4_t in = vld1q_u32(&input[input_start]);

        uint32x4_t acc = vld1q_u32(&acc[acc_start]);

        const uint8x16_t tbl_a0 = {0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255};
        const uint8x16_t tbl_a1 = {0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255};
        const uint8x16_t tbl_a2 = {0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 0, 255, 255, 255, 255};
        const uint8x16_t tbl_a3 = {0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255};

        uint8x16_t aa = vdupq_n_u8(nibble);
        uint8x16_t a0 = vqtbl1q_u8(tbl_a0, aa);
        uint8x16_t a1 = vqtbl1q_u8(tbl_a1, aa);
        uint8x16_t a2 = vqtbl1q_u8(tbl_a2, aa);
        uint8x16_t a3 = vqtbl1q_u8(tbl_a3, aa);

        acc[0] ^= a0 & in[0];
        acc[1] ^= a0 & in[1];
        acc[2] ^= a0 & in[2];

        const uint32x4_t mask1 = {0, 0, 0, 255};
        const uint32x4_t mask2 = {255, 255, 0, 0};
        const uint32x4_t mask3 = {255, 0, 0, 0};


        uint8x16_t inrot[3];
        inrot[0] = vextq_u8(vreinterpretq_u8_m128i(vgetq_lane_u32(in, 1)), vreinterpretq_u8_m128i(vgetq_lane_u32(in, 0)), 3);
        inrot[1] = vextq_u8(vreinterpretq_u8_m128i(vgetq_lane_u32(in, 2)), vreinterpretq_u8_m128i(vgetq_lane_u32(in, 1)), 3);
        inrot[2] = vextq_u8(vreinterpretq_u8_m128i(vgetq_lane_u32(in, 0)), vreinterpretq_u8_m128i(vgetq_lane_u32(in, 2)), 3);

        uint8x16_t inrot2[3];
        inrot2[0] = vextq_u8(vreinterpretq_u8_m128i(in[2]), vreinterpretq_u8_m128i(in[1]), 2);
        inrot2[1] = vextq_u8(vreinterpretq_u8_m128i(in[0]), vreinterpretq_u8_m128i(in[2]), 2);
        inrot2[2] = vextq_u8(vreinterpretq_u8_m128i(in[1]), vreinterpretq_u8_m128i(in[0]), 2);

        uint8x16_t inrot3[3];
        inrot3[0] = vextq_u8(vreinterpretq_u8_m128i(in[0]), vreinterpretq_u8_m128i(in[2]), 1);
        inrot3[1] = vextq_u8(vreinterpretq_u8_m128i(in[1]), vreinterpretq_u8_m128i(in[0]), 1);
        inrot3[2] = vextq_u8(vreinterpretq_u8_m128i(in[2]), vreinterpretq_u8_m128i(in[1]), 1);


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