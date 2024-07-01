#include <stdio.h>
#include <stdlib.h>

#ifdef __ARM_NEON
#include <arm_neon.h>
#endif /* __ARM_NEON */

#define __STDC_FORMAT_MACROS



void mul_add_bitsliced_m_vec_mayo12(u_int32_t *input, u_int32_t input_start, u_int8_t nibble, u_int32_t *acc, u_int32_t acc_start) {

        // In0 is x^0 and x^1 and in1 is x^2 and x^3
        uint32x4_t in0 = vld1q_u32(&input[input_start]);
        uint32x4_t in1 = vld1q_u32(&input[input_start+4]);

        uint32x4_t acc0 = vld1q_u32(&acc[acc_start]);
        uint32x4_t acc1 = vld1q_u32(&acc[acc_start+4]);

        // The lookup vector to be used for the multiplications
        uint8x16_t mul_tbl = {0x0, 0x13, 0x26, 0x35, 0x4c, 0x5f, 0x6a, 0x79, 0x98, 0x8b, 0xbe, 0xad, 0xd4, 0xc7, 0xf2, 0xe1};
        

        uint32x4_t inshuf2_1 = vcombine_u32(vget_low_u32(in1), vget_high_u32(in1));
        uint32x4_t inshuf2_2 = vcombine_u32(vget_low_u32(in0), vget_high_u32(in0));

        uint32x4_t inshuf1_1 = vcombine_u32(vget_high_u32(in0), vget_low_u32(in0));
        uint32x4_t inshuf1_2 = vcombine_u32(vget_high_u32(in1), vget_low_u32(in1));
        
        uint32x4_t inshuf3_1 = vcombine_u32(vget_low_u32(inshuf1_2), vget_high_u32(inshuf1_2));
        uint32x4_t inshuf3_2 = vcombine_u32(vget_low_u32(inshuf1_1), vget_high_u32(inshuf1_1));


        const uint64x2_t mask1_1 = vcombine_u64(vdup_n_u64(1), vdup_n_u64(16));
        const uint64x2_t mask1_2 = vcombine_u64(vdup_n_u64(16), vdup_n_u64(16));

        const uint64x2_t mask2_1 = vcombine_u64(vdup_n_u64(128), vdup_n_u64(32));
        const uint64x2_t mask2_2 = vcombine_u64(vdup_n_u64(8), vdup_n_u64(32));

        const uint64x2_t mask3_1 = vcombine_u64(vdup_n_u64(64), vdup_n_u64(4));
        const uint64x2_t mask3_2 = vcombine_u64(vdup_n_u64(64), vdup_n_u64(64));

        const uint64x2_t mask4_1 = vcombine_u64(vdup_n_u64(32), vdup_n_u64(8));
        const uint64x2_t mask4_2 = vcombine_u64(vdup_n_u64(32), vdup_n_u64(128));

        uint64x2_t aaaa = vreinterpretq_u64_u8(vqtbl1q_u8(mul_tbl, vdupq_n_u8(nibble)));

        acc0 ^= in0 & vreinterpretq_u32_u64(vceqq_u64(aaaa & mask1_1, mask1_1));
        acc1 ^= in1 & vreinterpretq_u32_u64(vceqq_u64(aaaa & mask1_2, mask1_2));

        acc0 ^= inshuf1_1 & vreinterpretq_u32_u64(vceqq_u64(aaaa & mask2_1, mask2_1));
        acc1 ^= inshuf1_2 & vreinterpretq_u32_u64(vceqq_u64(aaaa & mask2_2, mask2_2));

        acc0 ^= inshuf2_1 & vreinterpretq_u32_u64(vceqq_u64(aaaa & mask3_1, mask3_1));
        acc1 ^= inshuf2_2 & vreinterpretq_u32_u64(vceqq_u64(aaaa & mask3_2, mask3_2));

        acc0 ^= inshuf3_1 & vreinterpretq_u32_u64(vceqq_u64(aaaa & mask4_1, mask4_1));
        acc1 ^= inshuf3_2 & vreinterpretq_u32_u64(vceqq_u64(aaaa & mask4_2, mask4_2));

        vst1q_u32(&acc[acc_start], acc0);
        vst1q_u32(&acc[acc_start+4], acc1);
}







void mul_add_bitsliced_m_vec_mayo3(u_int32_t *input, u_int32_t input_start, u_int8_t nibble, u_int32_t *acc, u_int32_t acc_start) {
    
        int u_32_per_term = 3;

        uint32x4_t in0 = vld1q_u32(&input[input_start]);
        uint32x4_t in1 = vld1q_u32(&input[input_start + u_32_per_term]);
        uint32x4_t in2 = vld1q_u32(&input[input_start + 2 * u_32_per_term]);
        uint32x4_t in3 = vld1q_u32(&input[input_start + 3 * u_32_per_term]);


        // Load accumulators as uint32x2_t
        uint32x4_t acc0 = vld1q_u32(&acc[acc_start]);
        uint32x4_t acc1 = vld1q_u32(&acc[acc_start + u_32_per_term]);
        uint32x4_t acc2 = vld1q_u32(&acc[acc_start + 2 * u_32_per_term]);
        uint32x4_t acc3 = vld1q_u32(&acc[acc_start + 3 * u_32_per_term]);


        // Nibble mask extraction
        uint32x4_t n0 = vdupq_n_u32((nibble & 1) ? 0xFFFFFFFF : 0);
        uint32x4_t n1 = vdupq_n_u32((nibble & 2) ? 0xFFFFFFFF : 0);
        uint32x4_t n2 = vdupq_n_u32((nibble & 4) ? 0xFFFFFFFF : 0);
        uint32x4_t n3 = vdupq_n_u32((nibble & 8) ? 0xFFFFFFFF : 0);

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

        // Store results back to acc array by using 128 bit entries and overwriting the last 32 bits of the previous store.
        vst1q_u32(&acc[acc_start], acc0);
        vst1q_u32(&acc[acc_start + u_32_per_term], acc1);
        vst1q_u32(&acc[acc_start + 2 * u_32_per_term], acc2);

        // Handle last 96 individually to prevent overstoring with 32 bits.
        acc[acc_start + 9] = vgetq_lane_u32(acc3, 0);
        acc[acc_start + 10] = vgetq_lane_u32(acc3, 1);
        acc[acc_start + 11] = vgetq_lane_u32(acc3, 2);
}






 
void mul_add_bitsliced_m_vec_mayo5(u_int32_t *input, u_int32_t input_start, u_int8_t nibble, u_int32_t *acc, u_int32_t acc_start) {
    
        int u_32_per_term = 4;

        uint32x4_t in0 = vld1q_u32(&input[input_start]);
        uint32x4_t in1 = vld1q_u32(&input[input_start + u_32_per_term]);
        uint32x4_t in2 = vld1q_u32(&input[input_start + 2 * u_32_per_term]);
        uint32x4_t in3 = vld1q_u32(&input[input_start + 3 * u_32_per_term]);

        // Load accumulators as uint32x2_t
        uint32x4_t acc0 = vld1q_u32(&acc[acc_start]);
        uint32x4_t acc1 = vld1q_u32(&acc[acc_start + u_32_per_term]);
        uint32x4_t acc2 = vld1q_u32(&acc[acc_start + 2 * u_32_per_term]);
        uint32x4_t acc3 = vld1q_u32(&acc[acc_start + 3 * u_32_per_term]);



        // Nibble mask extraction
        uint32x4_t n0 = vdupq_n_u32((nibble & 1) ? 0xFFFFFFFF : 0);
        uint32x4_t n1 = vdupq_n_u32((nibble & 2) ? 0xFFFFFFFF : 0);
        uint32x4_t n2 = vdupq_n_u32((nibble & 4) ? 0xFFFFFFFF : 0);
        uint32x4_t n3 = vdupq_n_u32((nibble & 8) ? 0xFFFFFFFF : 0);


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
        vst1q_u32(&acc[acc_start], acc0);
        vst1q_u32(&acc[acc_start + u_32_per_term], acc1);
        vst1q_u32(&acc[acc_start + 2 * u_32_per_term], acc2);
        vst1q_u32(&acc[acc_start + 3 * u_32_per_term], acc3); 
}
