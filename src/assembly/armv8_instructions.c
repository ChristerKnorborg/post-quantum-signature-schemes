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


        uint8x8_t aaaa = vqtbl1q_u8(mul_tbl, vdup_n_u8(nibble));

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


void mayo_P1_times_O_mayo2(uint32_t *P1, unsigned char *O, uint32_t *acc) {

        const uint8x16_t mul_tbl = {0x0, 0x13, 0x26, 0x35, 0x4c, 0x5f, 0x6a, 0x79, 0x98, 0x8b, 0xbe, 0xad, 0xd4, 0xc7, 0xf2, 0xe1};

        const uint64x2_t mask1_1 = {1, 16};
        const uint64x2_t mask1_2 = {16, 16};

        const uint64x2_t mask2_1 = {128, 32};
        const uint64x2_t mask2_2 = {8, 32};

        const uint64x2_t mask3_1 = {64, 4};
        const uint64x2_t mask3_2 = {64, 64};

        const uint64x2_t mask4_1 = {32, 8};
        const uint64x2_t mask4_2 = {32, 128};

        #define MAYO_POS (r*V_MAYO_2+ c - (r)*(r+1)/2 )

        for (int c = 0; c < V_MAYO_2; c++) {
        int k;

                for (k = 0; k < (O_MAYO_2); k += 4) {

                        uint8x8_t aaaa_small = vqtbl1_u8(mul_tbl, vdup_n_u8(O[c * O_MAYO_2 + k]));
                        uint8x16_t aaaa = vcombine_u8(aaaa_small, aaaa_small);

                        uint64x2_t cmask1_1 = vceqq_u64(aaaa & mask1_1, mask1_1);
                        uint64x2_t cmask1_2 = vceqq_u64(aaaa & mask1_2, mask1_2);

                        uint64x2_t cmask2_1 = vceqq_u64(aaaa & mask2_1, mask2_1);
                        uint64x2_t cmask2_2 = vceqq_u64(aaaa & mask2_2, mask2_2);
                        
                        uint64x2_t cmask3_1 = vceqq_u64(aaaa & mask3_1, mask3_1);
                        uint64x2_t cmask3_2 = vceqq_u64(aaaa & mask3_2, mask3_2);

                        uint64x2_t cmask4_1 = vceqq_u64(aaaa & mask4_1, mask4_1);
                        uint64x2_t cmask4_2 = vceqq_u64(aaaa & mask4_2, mask4_2);

                        uint8x8_t aaaa_small2 = vqtbl1_u8(mul_tbl, vdup_n_u8(O[c * O_MAYO_2 + k + 1]));
                        uint8x16_t aaaa2 = vcombine_u8(aaaa_small2, aaaa_small2);
                        uint64x2_t cmask12_1 = vceqq_u64(aaaa2 & mask1_1, mask1_1);
                        uint64x2_t cmask12_2 = vceqq_u64(aaaa2 & mask1_2, mask1_2);
                        uint64x2_t cmask22_1 = vceqq_u64(aaaa2 & mask2_1, mask2_1);
                        uint64x2_t cmask22_2 = vceqq_u64(aaaa2 & mask2_2, mask2_2);
                        uint64x2_t cmask32_1 = vceqq_u64(aaaa2 & mask3_1, mask3_1);
                        uint64x2_t cmask32_2 = vceqq_u64(aaaa2 & mask3_2, mask3_2);
                        uint64x2_t cmask42_1 = vceqq_u64(aaaa2 & mask4_1, mask4_1);
                        uint64x2_t cmask42_2 = vceqq_u64(aaaa2 & mask4_2, mask4_2);

                        uint8x8_t aaaa_small3 = vqtbl1_u8(mul_tbl, vdup_n_u8(O[c * O_MAYO_2 + k + 2]));
                        uint8x16_t aaaa3 = vcombine_u8(aaaa_small3, aaaa_small3);
                        uint64x2_t cmask13_1 = vceqq_u64(aaaa3 & mask1_1, mask1_1);
                        uint64x2_t cmask13_2 = vceqq_u64(aaaa3 & mask1_2, mask1_2);
                        uint64x2_t cmask23_1 = vceqq_u64(aaaa3 & mask2_1, mask2_1);
                        uint64x2_t cmask23_2 = vceqq_u64(aaaa3 & mask2_2, mask2_2);
                        uint64x2_t cmask33_1 = vceqq_u64(aaaa3 & mask3_1, mask3_1);
                        uint64x2_t cmask33_2 = vceqq_u64(aaaa3 & mask3_2, mask3_2);
                        uint64x2_t cmask43_1 = vceqq_u64(aaaa3 & mask4_1, mask4_1);
                        uint64x2_t cmask43_2 = vceqq_u64(aaaa3 & mask4_2, mask4_2);

                        uint8x8_t aaaa_small4 = vqtbl1_u8(mul_tbl, vdup_n_u8(O[c * O_MAYO_2 + k + 3]));
                        uint8x16_t aaaa4 = vcombine_u8(aaaa_small4, aaaa_small4);
                        uint64x2_t cmask14_1 = vceqq_u64(aaaa4 & mask1_1, mask1_1);
                        uint64x2_t cmask14_2 = vceqq_u64(aaaa4 & mask1_2, mask1_2);
                        uint64x2_t cmask24_1 = vceqq_u64(aaaa4 & mask2_1, mask2_1);
                        uint64x2_t cmask24_2 = vceqq_u64(aaaa4 & mask2_2, mask2_2);
                        uint64x2_t cmask34_1 = vceqq_u64(aaaa4 & mask3_1, mask3_1);
                        uint64x2_t cmask34_2 = vceqq_u64(aaaa4 & mask3_2, mask3_2);
                        uint64x2_t cmask44_1 = vceqq_u64(aaaa4 & mask4_1, mask4_1);
                        uint64x2_t cmask44_2 = vceqq_u64(aaaa4 & mask4_2, mask4_2);


                        for (int r = 0; r <= c; r++) {

                                int mp = MAYO_POS;

                                int acc_index_1 = (r * O_MAYO_2 + k) * 8;
                                int acc_index_2 = (r * O_MAYO_2 + k + 1) * 8;
                                int acc_index_3 = (r * O_MAYO_2 + k + 2) * 8;
                                int acc_index_4 = (r * O_MAYO_2 + k + 3) * 8;

                                uint32x4_t in0 = vld1q_u64(&P1[mp*8]);
                                uint32x4_t in1 = vld1q_u64(&P1[mp*8+4]);

                                uint64x2_t acc0_1 = vld1q_u64(&acc[acc_index_1]);
                                uint64x2_t acc1_1 = vld1q_u64(&acc[acc_index_1 + 4]);
                                uint64x2_t acc0_2 = vld1q_u64(&acc[acc_index_2]);
                                uint64x2_t acc1_2 = vld1q_u64(&acc[acc_index_2 + 4]);
                                uint64x2_t acc0_3 = vld1q_u64(&acc[acc_index_3]);
                                uint64x2_t acc1_3 = vld1q_u64(&acc[acc_index_3 + 4]);
                                uint64x2_t acc0_4 = vld1q_u64(&acc[acc_index_4]);
                                uint64x2_t acc1_4 = vld1q_u64(&acc[acc_index_4 + 4]);


                                uint32x4_t inshuf2_1 = vcombine_u32(vget_low_u64(in1), vget_high_u64(in1));
                                uint32x4_t inshuf2_2 = vcombine_u32(vget_low_u64(in0), vget_high_u64(in0));

                                uint32x4_t inshuf1_1 = vcombine_u32(vget_high_u32(in0), vget_low_u32(in0));
                                uint32x4_t inshuf1_2 = vcombine_u32(vget_high_u32(in1), vget_low_u32(in1));
                                
                                uint32x4_t inshuf3_1 = vcombine_u32(vget_low_u64(inshuf1_2), vget_high_u64(inshuf1_2));
                                uint32x4_t inshuf3_2 = vcombine_u32(vget_low_u64(inshuf1_1), vget_high_u64(inshuf1_1));
                
                                        
                                acc0_1 ^= in0 & cmask1_1;
                                acc1_1 ^= in1 & cmask1_2;

                                acc0_1 ^= inshuf1_1 & cmask2_1;
                                acc1_1 ^= inshuf1_2 & cmask2_2;

                                acc0_1 ^= inshuf2_1 & cmask3_1;
                                acc1_1 ^= inshuf2_2 & cmask3_2;

                                acc0_1 ^= inshuf3_1 & cmask4_1;
                                acc1_1 ^= inshuf3_2 & cmask4_2;

                                acc0_2 ^= in0 & cmask12_1;
                                acc1_2 ^= in1 & cmask12_2;

                                acc0_2 ^= inshuf1_1 & cmask22_1;
                                acc1_2 ^= inshuf1_2 & cmask22_2;

                                acc0_2 ^= inshuf2_1 & cmask32_1;
                                acc1_2 ^= inshuf2_2 & cmask32_2;

                                acc0_2 ^= inshuf3_1 & cmask42_1;
                                acc1_2 ^= inshuf3_2 & cmask42_2;

                                acc0_3 ^= in0 & cmask13_1;
                                acc1_3 ^= in1 & cmask13_2;

                                acc0_3 ^= inshuf1_1 & cmask23_1;
                                acc1_3 ^= inshuf1_2 & cmask23_2;

                                acc0_3 ^= inshuf2_1 & cmask33_1;
                                acc1_3 ^= inshuf2_2 & cmask33_2;

                                acc0_3 ^= inshuf3_1 & cmask43_1;
                                acc1_3 ^= inshuf3_2 & cmask43_2;

                                acc0_4 ^= in0 & cmask14_1;
                                acc1_4 ^= in1 & cmask14_2;

                                acc0_4 ^= inshuf1_1 & cmask24_1;
                                acc1_4 ^= inshuf1_2 & cmask24_2;

                                acc0_4 ^= inshuf2_1 & cmask34_1;
                                acc1_4 ^= inshuf2_2 & cmask34_2;

                                acc0_4 ^= inshuf3_1 & cmask44_1;
                                acc1_4 ^= inshuf3_2 & cmask44_2;

                                vst1q_u32(&acc[acc_index_1], acc0_1);
                                vst1q_u32(&acc[acc_index_1 + 4], acc1_1);
                                vst1q_u32(&acc[acc_index_2], acc0_2);
                                vst1q_u32(&acc[acc_index_2 + 4], acc1_2);
                                vst1q_u32(&acc[acc_index_3], acc0_3);
                                vst1q_u32(&acc[acc_index_3 + 4], acc1_3);
                                vst1q_u32(&acc[acc_index_4], acc0_4);
                                vst1q_u32(&acc[acc_index_4 + 4], acc1_4);
                        }

                } 

                // These loops are for mayo2
                for (;k < (O_MAYO_2); k += 2) {

                        uint8x8_t aaaa_small = vqtbl1_u8(mul_tbl, vdup_n_u8(O[c * O_MAYO_2 + k]));
                        uint8x16_t aaaa = vcombine_u8(aaaa_small, aaaa_small);

                        uint64x2_t cmask1_1 = vceqq_u64(aaaa & mask1_1, mask1_1);
                        uint64x2_t cmask1_2 = vceqq_u64(aaaa & mask1_2, mask1_2);

                        uint64x2_t cmask2_1 = vceqq_u64(aaaa & mask2_1, mask2_1);
                        uint64x2_t cmask2_2 = vceqq_u64(aaaa & mask2_2, mask2_2);
                        
                        uint64x2_t cmask3_1 = vceqq_u64(aaaa & mask3_1, mask3_1);
                        uint64x2_t cmask3_2 = vceqq_u64(aaaa & mask3_2, mask3_2);

                        uint64x2_t cmask4_1 = vceqq_u64(aaaa & mask4_1, mask4_1);
                        uint64x2_t cmask4_2 = vceqq_u64(aaaa & mask4_2, mask4_2);

                        uint8x8_t aaaa_small2 = vqtbl1_u8(mul_tbl, vdup_n_u8(O[c * O_MAYO_2 + k + 1]));
                        uint8x16_t aaaa2 = vcombine_u8(aaaa_small2, aaaa_small2);
                        uint64x2_t cmask12_1 = vceqq_u64(aaaa2 & mask1_1, mask1_1);
                        uint64x2_t cmask12_2 = vceqq_u64(aaaa2 & mask1_2, mask1_2);
                        uint64x2_t cmask22_1 = vceqq_u64(aaaa2 & mask2_1, mask2_1);
                        uint64x2_t cmask22_2 = vceqq_u64(aaaa2 & mask2_2, mask2_2);
                        uint64x2_t cmask32_1 = vceqq_u64(aaaa2 & mask3_1, mask3_1);
                        uint64x2_t cmask32_2 = vceqq_u64(aaaa2 & mask3_2, mask3_2);
                        uint64x2_t cmask42_1 = vceqq_u64(aaaa2 & mask4_1, mask4_1);
                        uint64x2_t cmask42_2 = vceqq_u64(aaaa2 & mask4_2, mask4_2);

                        for (int r = 0; r <= c; r++) {

                                int mp = MAYO_POS;

                                int acc_index_1 = (r * O_MAYO_2 + k) * 8;
                                int acc_index_2 = (r * O_MAYO_2 + k + 1) * 8;

                                uint32x4_t in0 = vld1q_u64(&P1[mp*8]);
                                uint32x4_t in1 = vld1q_u64(&P1[mp*8+4]);

                                uint64x2_t acc0_1 = vld1q_u64(&acc[acc_index_1]);
                                uint64x2_t acc1_1 = vld1q_u64(&acc[acc_index_1 + 4]);
                                uint64x2_t acc0_2 = vld1q_u64(&acc[acc_index_2]);
                                uint64x2_t acc1_2 = vld1q_u64(&acc[acc_index_2 + 4]);

                                uint32x4_t inshuf2_1 = vcombine_u32(vget_low_u64(in1), vget_high_u64(in1));
                                uint32x4_t inshuf2_2 = vcombine_u32(vget_low_u64(in0), vget_high_u64(in0));

                                uint32x4_t inshuf1_1 = vcombine_u32(vget_high_u32(in0), vget_low_u32(in0));
                                uint32x4_t inshuf1_2 = vcombine_u32(vget_high_u32(in1), vget_low_u32(in1));
                                
                                uint32x4_t inshuf3_1 = vcombine_u32(vget_low_u64(inshuf1_2), vget_high_u64(inshuf1_2));
                                uint32x4_t inshuf3_2 = vcombine_u32(vget_low_u64(inshuf1_1), vget_high_u64(inshuf1_1));
                
                                        
                                acc0_1 ^= in0 & cmask1_1;
                                acc1_1 ^= in1 & cmask1_2;

                                acc0_1 ^= inshuf1_1 & cmask2_1;
                                acc1_1 ^= inshuf1_2 & cmask2_2;

                                acc0_1 ^= inshuf2_1 & cmask3_1;
                                acc1_1 ^= inshuf2_2 & cmask3_2;

                                acc0_1 ^= inshuf3_1 & cmask4_1;
                                acc1_1 ^= inshuf3_2 & cmask4_2;


                                acc0_2 ^= in0 & cmask12_1;
                                acc1_2 ^= in1 & cmask12_2;
                                acc0_2 ^= inshuf1_1 & cmask22_1;
                                acc1_2 ^= inshuf1_2 & cmask22_2;
                                acc0_2 ^= inshuf2_1 & cmask32_1;
                                acc1_2 ^= inshuf2_2 & cmask32_2;
                                acc0_2 ^= inshuf3_1 & cmask42_1;
                                acc1_2 ^= inshuf3_2 & cmask42_2;

                                vst1q_u32(&acc[acc_index_1], acc0_1);
                                vst1q_u32(&acc[acc_index_1 + 4], acc1_1);
                                vst1q_u32(&acc[acc_index_2], acc0_2);
                                vst1q_u32(&acc[acc_index_2 + 4], acc1_2);
                        }
                }

        }

        #undef MAYO_POS
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


void mul_add_bitsliced_m_vec_mayo1(u_int32_t *input, u_int32_t input_start, u_int32_t input_offset, u_int8_t nibble1, u_int8_t nibble2, u_int32_t *acc, u_int32_t acc_start) {
    
        int u_32_per_term = 2;

        uint32x2_t in0_0 = vld1_u32(&input[input_start]);
        uint32x2_t in1_0 = vld1_u32(&input[input_start + u_32_per_term]);
        uint32x2_t in2_0 = vld1_u32(&input[input_start + 2 * u_32_per_term]);
        uint32x2_t in3_0 = vld1_u32(&input[input_start + 3 * u_32_per_term]);

        uint32x2_t in0_1 = vld1_u32(&input[input_offset + input_start]);
        uint32x2_t in1_1 = vld1_u32(&input[input_offset + (input_start + u_32_per_term)]);
        uint32x2_t in2_1 = vld1_u32(&input[input_offset + (input_start + 2 * u_32_per_term)]);
        uint32x2_t in3_1 = vld1_u32(&input[input_offset + (input_start + 3 * u_32_per_term)]);

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

        // Convert to uint32x4_t for processing (Only add acc once or it will cancel for XOR)
        int32x2_t zero = vdup_n_u32(0);
        uint32x4_t acc0 = vcombine_u32(acc0_low, zero);
        uint32x4_t acc1 = vcombine_u32(acc1_low, zero);
        uint32x4_t acc2 = vcombine_u32(acc2_low, zero);
        uint32x4_t acc3 = vcombine_u32(acc3_low, zero);


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



// First method for MAYO1 and MAYO2 - works similarly to thoes of MAYO3 and MAYO5
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





void encode_bit_sliced_array_mayo12(u_int8_t *input, u_int8_t *output, int matrices) {
        
        uint32_t* input_u32 = (uint32_t*) input;

        // Load 64 bits directly into 64-bit NEON registers
        uint32x2_t A0 = vld1_u32(&input_u32[0]); // Load the first 64 bits
        uint32x2_t A1 = vld1_u32(&input_u32[1]); // Load the second 64 bits
        uint32x2_t A2 = vld1_u32(&input_u32[2]); // Load the third 64 bits
        uint32x2_t A3 = vld1_u32(&input_u32[3]); // Load the fourth 64 bits


        // uint8_t* output_u8 = (uint8_t*) output;
        // vst1_u8(&output_u8[0], vreinterpret_u8_u64(A0));    // Store 8 bytes of A0_prime
        // vst1_u8(&output_u8[4], vreinterpret_u8_u64(A1));    // Store 8 bytes of A1_prime
        // vst1_u8(&output_u8[8], vreinterpret_u8_u64(A2));   // Store 8 bytes of A2_prime
        // vst1_u8(&output_u8[12], vreinterpret_u8_u64(A3));   // Store 8 bytes of A3_prime
    

        // Create a 64-bit NEON register with the constant pattern
        uint32x2_t mask1  = vdup_n_u32(0x11111111);
        uint32x2_t mask2  = vdup_n_u32(0x22222222);
        uint32x2_t mask4  = vdup_n_u32(0x44444444);
        uint32x2_t mask8  = vdup_n_u32(0x88888888);



        // Bitwise AND and OR operations
        uint32x2_t A0_prime = vand_u32(A0, mask1);
        uint32x2_t A3_prime = vand_u32(A1, mask1);
        A0_prime = vorr_u32(A0_prime, vshl_n_u32(A3_prime, 1));
        A3_prime = vand_u32(A2, mask1);
        A0_prime = vorr_u32(A0_prime, vshl_n_u32(A3_prime, 2));
        A3_prime = vand_u32(A3, mask1);
        A0_prime = vorr_u32(A0_prime, vshl_n_u32(A3_prime, 3));

        uint32x2_t A1_prime = vand_u32(A1, mask2);
        A3_prime = vand_u32(A0, mask2);
        A1_prime = vorr_u32(A1_prime, vshr_n_u32(A3_prime, 1));
        A3_prime = vand_u32(A2, mask2);
        A1_prime = vorr_u32(A1_prime, vshl_n_u32(A3_prime, 1));
        A3_prime = vand_u32(A3, mask2);
        A1_prime = vorr_u32(A1_prime, vshl_n_u32(A3_prime, 2));

        uint32x2_t A2_prime = vand_u32(A2, mask4);
        A3_prime = vand_u32(A0, mask4);
        A2_prime = vorr_u32(A2_prime, vshr_n_u32(A3_prime, 2));
        A3_prime = vand_u32(A1, mask4);
        A2_prime = vorr_u32(A2_prime, vshr_n_u32(A3_prime, 1));
        A3_prime = vand_u32(A3, mask4);
        A2_prime = vorr_u32(A2_prime, vshl_n_u32(A3_prime, 1));

        A3_prime = vand_u32(A3, mask8);
        A3       = vand_u32(A0, mask8);
        A3_prime = vorr_u32(A3_prime, vshr_n_u32(A3, 3));
        A3       = vand_u32(A1, mask8);
        A3_prime = vorr_u32(A3_prime, vshr_n_u32(A3, 2));
        A3       = vand_u32(A2, mask8);
        A3_prime = vorr_u32(A3_prime, vshr_n_u32(A3, 1));


        // Store results back to the output array
        // uint8_t* output_u8 = (uint8_t*) output;
        // vst1_u8(&output_u8[0], vreinterpret_u8_u32(A0_prime));    // Store 8 bytes of A0_prime
        // vst1_u8(&output_u8[4], vreinterpret_u8_u32(A1_prime));    // Store 8 bytes of A1_prime
        // vst1_u8(&output_u8[8], vreinterpret_u8_u32(A2_prime));   // Store 8 bytes of A2_prime
        // vst1_u8(&output_u8[12], vreinterpret_u8_u32(A3_prime));   // Store 8 bytes of A3_prime

       
       // Store results back to the output array
        uint8_t* output_u8 = (uint8_t*) output;
        *(uint32_t*)&output_u8[0] = vget_lane_u32(A0_prime, 0);
        *(uint32_t*)&output_u8[4] = vget_lane_u32(A1_prime, 0);
        *(uint32_t*)&output_u8[8] = vget_lane_u32(A2_prime, 0);
        *(uint32_t*)&output_u8[12] = vget_lane_u32(A3_prime, 0);
}




void decode_bit_sliced_array (u_int8_t *input, u_int8_t *output, int matrices) {

}




