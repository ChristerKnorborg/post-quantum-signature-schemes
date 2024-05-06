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


/* void mul_add_64_bitsliced_m_vec(u_int32_t *input, u_int32_t input_start, u_int8_t nibble, u_int32_t *acc, u_int32_t acc_start) {

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


        uint8x8_t aaaa_small = vqtbl1_u8(mul_tbl, vdup_n_u8(nibble));
        uint8x16_t aaaa = vcombine_u8(aaaa_small, aaaa_small);


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
} */

void mayo_P1_times_O_mayo1(uint32_t *P1, unsigned char *O, uint32_t *acc) {

        const uint8x16_t mul_tbl = {0x0, 0x13, 0x26, 0x35, 0x4c, 0x5f, 0x6a, 0x79, 0x98, 0x8b, 0xbe, 0xad, 0xd4, 0xc7, 0xf2, 0xe1};

        const uint64x2_t mask1_1 = {1, 16};
        const uint64x2_t mask1_2 = {16, 16};

        const uint64x2_t mask2_1 = {128, 32};
        const uint64x2_t mask2_2 = {8, 32};

        const uint64x2_t mask3_1 = {64, 4};
        const uint64x2_t mask3_2 = {64, 64};

        const uint64x2_t mask4_1 = {32, 8};
        const uint64x2_t mask4_2 = {32, 128};

        #define MAYO_POS (r*V_MAYO_1+ c - (r)*(r+1)/2 )

        for (int c = 0; c < V_MAYO_1; c++) {
        int k;

                for (k = 0; k < (O_MAYO_1); k += 4) {

                        uint8x8_t aaaa_small = vqtbl1_u8(mul_tbl, vdup_n_u8(O[c * O_MAYO_1 + k]));
                        uint8x16_t aaaa = vcombine_u8(aaaa_small, aaaa_small);

                        uint64x2_t cmask1_1 = vceqq_u64(aaaa & mask1_1, mask1_1);
                        uint64x2_t cmask1_2 = vceqq_u64(aaaa & mask1_2, mask1_2);

                        uint64x2_t cmask2_1 = vceqq_u64(aaaa & mask2_1, mask2_1);
                        uint64x2_t cmask2_2 = vceqq_u64(aaaa & mask2_2, mask2_2);
                        
                        uint64x2_t cmask3_1 = vceqq_u64(aaaa & mask3_1, mask3_1);
                        uint64x2_t cmask3_2 = vceqq_u64(aaaa & mask3_2, mask3_2);

                        uint64x2_t cmask4_1 = vceqq_u64(aaaa & mask4_1, mask4_1);
                        uint64x2_t cmask4_2 = vceqq_u64(aaaa & mask4_2, mask4_2);

                        uint8x8_t aaaa_small2 = vqtbl1_u8(mul_tbl, vdup_n_u8(O[c * O_MAYO_1 + k + 1]));
                        uint8x16_t aaaa2 = vcombine_u8(aaaa_small2, aaaa_small2);
                        uint64x2_t cmask12_1 = vceqq_u64(aaaa2 & mask1_1, mask1_1);
                        uint64x2_t cmask12_2 = vceqq_u64(aaaa2 & mask1_2, mask1_2);
                        uint64x2_t cmask22_1 = vceqq_u64(aaaa2 & mask2_1, mask2_1);
                        uint64x2_t cmask22_2 = vceqq_u64(aaaa2 & mask2_2, mask2_2);
                        uint64x2_t cmask32_1 = vceqq_u64(aaaa2 & mask3_1, mask3_1);
                        uint64x2_t cmask32_2 = vceqq_u64(aaaa2 & mask3_2, mask3_2);
                        uint64x2_t cmask42_1 = vceqq_u64(aaaa2 & mask4_1, mask4_1);
                        uint64x2_t cmask42_2 = vceqq_u64(aaaa2 & mask4_2, mask4_2);

                        uint8x8_t aaaa_small3 = vqtbl1_u8(mul_tbl, vdup_n_u8(O[c * O_MAYO_1 + k + 2]));
                        uint8x16_t aaaa3 = vcombine_u8(aaaa_small3, aaaa_small3);
                        uint64x2_t cmask13_1 = vceqq_u64(aaaa3 & mask1_1, mask1_1);
                        uint64x2_t cmask13_2 = vceqq_u64(aaaa3 & mask1_2, mask1_2);
                        uint64x2_t cmask23_1 = vceqq_u64(aaaa3 & mask2_1, mask2_1);
                        uint64x2_t cmask23_2 = vceqq_u64(aaaa3 & mask2_2, mask2_2);
                        uint64x2_t cmask33_1 = vceqq_u64(aaaa3 & mask3_1, mask3_1);
                        uint64x2_t cmask33_2 = vceqq_u64(aaaa3 & mask3_2, mask3_2);
                        uint64x2_t cmask43_1 = vceqq_u64(aaaa3 & mask4_1, mask4_1);
                        uint64x2_t cmask43_2 = vceqq_u64(aaaa3 & mask4_2, mask4_2);

                        uint8x8_t aaaa_small4 = vqtbl1_u8(mul_tbl, vdup_n_u8(O[c * O_MAYO_1 + k + 3]));
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

                                int acc_index_1 = (r * O_MAYO_1 + k) * 8;
                                int acc_index_2 = (r * O_MAYO_1 + k + 1) * 8;
                                int acc_index_3 = (r * O_MAYO_1 + k + 2) * 8;
                                int acc_index_4 = (r * O_MAYO_1 + k + 3) * 8;

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
        }

        #undef MAYO_POS
}

void mayo_P1_times_Vt_mayo1(uint32_t *P1, unsigned char *V, uint32_t *acc) {

        const uint8x16_t mul_tbl = {0x0, 0x13, 0x26, 0x35, 0x4c, 0x5f, 0x6a, 0x79, 0x98, 0x8b, 0xbe, 0xad, 0xd4, 0xc7, 0xf2, 0xe1};

        const uint64x2_t mask1_1 = {1, 16};
        const uint64x2_t mask1_2 = {16, 16};

        const uint64x2_t mask2_1 = {128, 32};
        const uint64x2_t mask2_2 = {8, 32};

        const uint64x2_t mask3_1 = {64, 4};
        const uint64x2_t mask3_2 = {64, 64};

        const uint64x2_t mask4_1 = {32, 8};
        const uint64x2_t mask4_2 = {32, 128};

        #define MAYO_POS (r*V_MAYO_1+ c - (r)*(r+1)/2 )

        for (int c = 0; c < V_MAYO_1; c++) {
   
                uint8x8_t aaaa_small = vqtbl1_u8(mul_tbl, vdup_n_u8(V[c]));
                uint8x16_t aaaa = vcombine_u8(aaaa_small, aaaa_small);

                uint64x2_t cmask1_1 = vceqq_u64(aaaa & mask1_1, mask1_1);
                uint64x2_t cmask1_2 = vceqq_u64(aaaa & mask1_2, mask1_2);

                uint64x2_t cmask2_1 = vceqq_u64(aaaa & mask2_1, mask2_1);
                uint64x2_t cmask2_2 = vceqq_u64(aaaa & mask2_2, mask2_2);
                
                uint64x2_t cmask3_1 = vceqq_u64(aaaa & mask3_1, mask3_1);
                uint64x2_t cmask3_2 = vceqq_u64(aaaa & mask3_2, mask3_2);

                uint64x2_t cmask4_1 = vceqq_u64(aaaa & mask4_1, mask4_1);
                uint64x2_t cmask4_2 = vceqq_u64(aaaa & mask4_2, mask4_2);

                uint8x8_t aaaa_small2 = vqtbl1_u8(mul_tbl, vdup_n_u8(V[V_MAYO_1 + c]));
                uint8x16_t aaaa2 = vcombine_u8(aaaa_small2, aaaa_small2);
                uint64x2_t cmask12_1 = vceqq_u64(aaaa2 & mask1_1, mask1_1);
                uint64x2_t cmask12_2 = vceqq_u64(aaaa2 & mask1_2, mask1_2);
                uint64x2_t cmask22_1 = vceqq_u64(aaaa2 & mask2_1, mask2_1);
                uint64x2_t cmask22_2 = vceqq_u64(aaaa2 & mask2_2, mask2_2);
                uint64x2_t cmask32_1 = vceqq_u64(aaaa2 & mask3_1, mask3_1);
                uint64x2_t cmask32_2 = vceqq_u64(aaaa2 & mask3_2, mask3_2);
                uint64x2_t cmask42_1 = vceqq_u64(aaaa2 & mask4_1, mask4_1);
                uint64x2_t cmask42_2 = vceqq_u64(aaaa2 & mask4_2, mask4_2);

                uint8x8_t aaaa_small3 = vqtbl1_u8(mul_tbl, vdup_n_u8(V[2 * V_MAYO_1 + c]));
                uint8x16_t aaaa3 = vcombine_u8(aaaa_small3, aaaa_small3);
                uint64x2_t cmask13_1 = vceqq_u64(aaaa3 & mask1_1, mask1_1);
                uint64x2_t cmask13_2 = vceqq_u64(aaaa3 & mask1_2, mask1_2);
                uint64x2_t cmask23_1 = vceqq_u64(aaaa3 & mask2_1, mask2_1);
                uint64x2_t cmask23_2 = vceqq_u64(aaaa3 & mask2_2, mask2_2);
                uint64x2_t cmask33_1 = vceqq_u64(aaaa3 & mask3_1, mask3_1);
                uint64x2_t cmask33_2 = vceqq_u64(aaaa3 & mask3_2, mask3_2);
                uint64x2_t cmask43_1 = vceqq_u64(aaaa3 & mask4_1, mask4_1);
                uint64x2_t cmask43_2 = vceqq_u64(aaaa3 & mask4_2, mask4_2);

                uint8x8_t aaaa_small4 = vqtbl1_u8(mul_tbl, vdup_n_u8(V[3 * V_MAYO_1 + c]));
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

                        int acc_index_1 = (r * K_MAYO_1) * 8;
                        int acc_index_2 = (r * K_MAYO_1 + 1) * 8;
                        int acc_index_3 = (r * K_MAYO_1 + 2) * 8;
                        int acc_index_4 = (r * K_MAYO_1 + 3) * 8;

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

                aaaa_small = vqtbl1_u8(mul_tbl, vdup_n_u8(V[4* V_MAYO_1 + c]));
                aaaa = vcombine_u8(aaaa_small, aaaa_small);

                cmask1_1 = vceqq_u64(aaaa & mask1_1, mask1_1);
                 cmask1_2 = vceqq_u64(aaaa & mask1_2, mask1_2);

                 cmask2_1 = vceqq_u64(aaaa & mask2_1, mask2_1);
                 cmask2_2 = vceqq_u64(aaaa & mask2_2, mask2_2);
                
                 cmask3_1 = vceqq_u64(aaaa & mask3_1, mask3_1);
                 cmask3_2 = vceqq_u64(aaaa & mask3_2, mask3_2);

                 cmask4_1 = vceqq_u64(aaaa & mask4_1, mask4_1);
                 cmask4_2 = vceqq_u64(aaaa & mask4_2, mask4_2);

                aaaa_small2 = vqtbl1_u8(mul_tbl, vdup_n_u8(V[5 * V_MAYO_1 + c]));
                 aaaa2 = vcombine_u8(aaaa_small2, aaaa_small2);
                 cmask12_1 = vceqq_u64(aaaa2 & mask1_1, mask1_1);
                 cmask12_2 = vceqq_u64(aaaa2 & mask1_2, mask1_2);
                 cmask22_1 = vceqq_u64(aaaa2 & mask2_1, mask2_1);
                 cmask22_2 = vceqq_u64(aaaa2 & mask2_2, mask2_2);
                 cmask32_1 = vceqq_u64(aaaa2 & mask3_1, mask3_1);
                 cmask32_2 = vceqq_u64(aaaa2 & mask3_2, mask3_2);
                 cmask42_1 = vceqq_u64(aaaa2 & mask4_1, mask4_1);
                 cmask42_2 = vceqq_u64(aaaa2 & mask4_2, mask4_2);

                aaaa_small3 = vqtbl1_u8(mul_tbl, vdup_n_u8(V[6 * V_MAYO_1 + c]));
                 aaaa3 = vcombine_u8(aaaa_small3, aaaa_small3);
                 cmask13_1 = vceqq_u64(aaaa3 & mask1_1, mask1_1);
                 cmask13_2 = vceqq_u64(aaaa3 & mask1_2, mask1_2);
                 cmask23_1 = vceqq_u64(aaaa3 & mask2_1, mask2_1);
                 cmask23_2 = vceqq_u64(aaaa3 & mask2_2, mask2_2);
                 cmask33_1 = vceqq_u64(aaaa3 & mask3_1, mask3_1);
                 cmask33_2 = vceqq_u64(aaaa3 & mask3_2, mask3_2);
                 cmask43_1 = vceqq_u64(aaaa3 & mask4_1, mask4_1);
                 cmask43_2 = vceqq_u64(aaaa3 & mask4_2, mask4_2);

                aaaa_small4 = vqtbl1_u8(mul_tbl, vdup_n_u8(V[7 * V_MAYO_1 + c]));
                 aaaa4 = vcombine_u8(aaaa_small4, aaaa_small4);
                 cmask14_1 = vceqq_u64(aaaa4 & mask1_1, mask1_1);
                 cmask14_2 = vceqq_u64(aaaa4 & mask1_2, mask1_2);
                 cmask24_1 = vceqq_u64(aaaa4 & mask2_1, mask2_1);
                 cmask24_2 = vceqq_u64(aaaa4 & mask2_2, mask2_2);
                 cmask34_1 = vceqq_u64(aaaa4 & mask3_1, mask3_1);
                 cmask34_2 = vceqq_u64(aaaa4 & mask3_2, mask3_2);
                 cmask44_1 = vceqq_u64(aaaa4 & mask4_1, mask4_1);
                 cmask44_2 = vceqq_u64(aaaa4 & mask4_2, mask4_2);

                uint8x8_t aaaa_small5 = vqtbl1_u8(mul_tbl, vdup_n_u8(V[8 * V_MAYO_1 + c]));
                uint8x16_t aaaa5 = vcombine_u8(aaaa_small5, aaaa_small5);
                uint64x2_t cmask15_1 = vceqq_u64(aaaa5 & mask1_1, mask1_1);
                uint64x2_t cmask15_2 = vceqq_u64(aaaa5 & mask1_2, mask1_2);
                uint64x2_t cmask25_1 = vceqq_u64(aaaa5 & mask2_1, mask2_1);
                uint64x2_t cmask25_2 = vceqq_u64(aaaa5 & mask2_2, mask2_2);
                uint64x2_t cmask35_1 = vceqq_u64(aaaa5 & mask3_1, mask3_1);
                uint64x2_t cmask35_2 = vceqq_u64(aaaa5 & mask3_2, mask3_2);
                uint64x2_t cmask45_1 = vceqq_u64(aaaa5 & mask4_1, mask4_1);
                uint64x2_t cmask45_2 = vceqq_u64(aaaa5 & mask4_2, mask4_2);

                for (int r = 0; r <= c; r++) {

                        int mp = MAYO_POS;

                        int acc_index_1 = (r * K_MAYO_1 + 4) * 8;
                        int acc_index_2 = (r * K_MAYO_1 + 5) * 8;
                        int acc_index_3 = (r * K_MAYO_1 + 6) * 8;
                        int acc_index_4 = (r * K_MAYO_1 + 7) * 8;
                        int acc_index_5 = (r * K_MAYO_1 + 8) * 8;

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
                        uint64x2_t acc0_5 = vld1q_u64(&acc[acc_index_5]);
                        uint64x2_t acc1_5 = vld1q_u64(&acc[acc_index_5 + 4]);


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

                        acc0_5 ^= in0 & cmask15_1;
                        acc1_5 ^= in1 & cmask15_2;

                        acc0_5 ^= inshuf1_1 & cmask25_1;
                        acc1_5 ^= inshuf1_2 & cmask25_2;

                        acc0_5 ^= inshuf2_1 & cmask35_1;
                        acc1_5 ^= inshuf2_2 & cmask35_2;

                        acc0_5 ^= inshuf3_1 & cmask45_1;
                        acc1_5 ^= inshuf3_2 & cmask45_2;

                        vst1q_u32(&acc[acc_index_1], acc0_1);
                        vst1q_u32(&acc[acc_index_1 + 4], acc1_1);
                        vst1q_u32(&acc[acc_index_2], acc0_2);
                        vst1q_u32(&acc[acc_index_2 + 4], acc1_2);
                        vst1q_u32(&acc[acc_index_3], acc0_3);
                        vst1q_u32(&acc[acc_index_3 + 4], acc1_3);
                        vst1q_u32(&acc[acc_index_4], acc0_4);
                        vst1q_u32(&acc[acc_index_4 + 4], acc1_4);
                        vst1q_u32(&acc[acc_index_5], acc0_5);
                        vst1q_u32(&acc[acc_index_5 + 4], acc1_5);
                }

        }

        #undef MAYO_POS
}