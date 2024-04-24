#include <stdio.h>
#include <stdlib.h>

#ifdef __ARM_NEON
#include <arm_neon.h>
#endif /* __ARM_NEON */

#define __STDC_FORMAT_MACROS

 
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
        // vst1_u8(&output_u8[8], vreinterpret_u8_u64(A1));    // Store 8 bytes of A1_prime
        // vst1_u8(&output_u8[16], vreinterpret_u8_u64(A2));   // Store 8 bytes of A2_prime
        // vst1_u8(&output_u8[24], vreinterpret_u8_u64(A3));   // Store 8 bytes of A3_prime
    

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
        uint8_t* output_u8 = (uint8_t*) output;
        vst1_u8(&output_u8[0], vreinterpret_u8_u32(A0_prime));    // Store 8 bytes of A0_prime
        vst1_u8(&output_u8[4], vreinterpret_u8_u32(A1_prime));    // Store 8 bytes of A1_prime
        vst1_u8(&output_u8[8], vreinterpret_u8_u32(A2_prime));   // Store 8 bytes of A2_prime
        vst1_u8(&output_u8[12], vreinterpret_u8_u32(A3_prime));   // Store 8 bytes of A3_prime

        // uint8_t* output_u8 = (uint8_t*) output;
        // vst1_u8(&output_u8[0], vreinterpret_u8_u32(vget_get_lane(A0_prime, 0)));    // Store 8 bytes of A0_prime
        // vst1_u8(&output_u8[4], vreinterpret_u8_u32(vget_get_lane(A1_prime, 0)));    // Store 8 bytes of A1_prime
        // vst1_u8(&output_u8[8], vreinterpret_u8_u32(vget_get_lane(A2_prime, 0)));   // Store 8 bytes of A2_prime
        // vst1_u8(&output_u8[12], vreinterpret_u8_u32(vget_get_lane(A3_prime, 0)));   // Store 8 bytes of A3_prime

        // output_u8[0] = (u_int8_t) vget_get_lane(A0_prime, 0) & 0xFF;
}




void decode_bit_sliced_array (u_int8_t *input, u_int8_t *output, int matrices) {

}




