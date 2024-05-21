/* aes-arm.c - ARMv8 AES extensions using C intrinsics         */
/*   Written and placed in public domain by Jeffrey Walton     */
/*   Based on code from ARM, and by Johannes Schneiders, Skip  */
/*   Hovsmith and Barry O'Rourke for the mbedTLS project.      */

/* gcc -std=c99 -march=armv8-a+crypto aes-arm.c -o aes-arm.exe */


#include "mem.h"
#include <string.h>
#include <stdlib.h>

#if defined(__arm__) || defined(__aarch32__) || defined(__arm64__) || defined(__aarch64__) || defined(_M_ARM) || defined(_M_ARM64)
# if defined(__GNUC__)
#  include <stdint.h>
# endif
# if defined(__ARM_NEON) || defined(_MSC_VER)
#  include <arm_neon.h>
# endif
/* GCC and LLVM Clang, but not Apple Clang */
# if defined(__GNUC__) && !defined(__apple_build_version__)
#  if defined(__ARM_ACLE) || defined(__ARM_FEATURE_CRYPTO)
#   include <arm_acle.h>
#  endif
# endif
#endif  /* ARM Headers */

// aes s-box
static const uint8_t sbox[256] = {
  //0     1    2      3     4    5     6     7      8    9     A      B    C     D     E     F
  0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
  0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
  0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
  0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
  0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
  0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
  0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
  0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
  0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
  0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
  0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
  0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
  0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
  0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
  0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
  0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16 };


// subword algorithm used in the aes key scheduling. 
uint32_t subword(uint32_t word) {
    return (uint32_t)sbox[(word >> 24) & 0xFF] << 24 |
           (uint32_t)sbox[(word >> 16) & 0xFF] << 16 |
           (uint32_t)sbox[(word >> 8) & 0xFF] << 8 |
           (uint32_t)sbox[word & 0xFF];
}

uint32x4_t aeskeygenassist(uint32x4_t a32, uint8_t rcon) {
    // Extract words X1 and X3
    uint32_t X1 = vgetq_lane_u32(a32, 1);
    uint32_t X3 = vgetq_lane_u32(a32, 3);

    // Apply SubWord (Manually implemented using AES S-box lookup table)
    uint32_t subX1 = subword(X1);  // Implement this function based on AES S-box
    uint32_t subX3 = subword(X3);  // Implement this function based on AES S-box

    // RotWord
    uint32_t rotX1 = (subX1 >> 8) | (subX1 << 24);
    uint32_t rotX3 = (subX3 >> 8) | (subX3 << 24);

    // Apply RCON
    rotX1 ^= (uint32_t)rcon;
    rotX3 ^= (uint32_t)rcon;

    // Assemble the final vector
    uint32x4_t result = a32;
    result = vsetq_lane_u32(subX1, result, 0);
    result = vsetq_lane_u32(rotX1, result, 1);
    result = vsetq_lane_u32(subX3, result, 2);
    result = vsetq_lane_u32(rotX3, result, 3);

    return result;
}

static void aes_setkey_encrypt(const unsigned char *key, uint8x16_t rkeys[]) {
    uint8x16_t key0 = vld1q_u8((const uint8_t *)(key));
    uint32x4_t temp0, temp1, temp4;
    int idx = 0;

    temp0 = vreinterpretq_u32_u8(key0);
    temp4 = vdupq_n_u32(0);

    #define BLOCK1(IMM)                                                     \
      temp1 = aeskeygenassist(temp0, IMM);                                  \
      rkeys[idx++] = vreinterpretq_u8_u32(temp0);                           \
      temp4 = vsetq_lane_u32(vgetq_lane_u32(temp0, 0), temp4, 1);           \
      temp4 = vsetq_lane_u32(vgetq_lane_u32(temp0, 1), temp4, 2);           \
      temp4 = vsetq_lane_u32(vgetq_lane_u32(temp0, 2), temp4, 3);           \
      temp0 = veorq_u32(temp0, temp4);                                      \
      temp0 = (vsetq_lane_u64(((uint64_t) veor_u32(vget_high_u32(temp0), vget_low_u32(temp0))), (temp0), 1));      \
      temp1 = vdupq_n_u32(vgetq_lane_u32(temp1, 3));                        \
      temp0 = veorq_u32(temp0, temp1);                                      \

    BLOCK1(0x01);
    BLOCK1(0x02);
    BLOCK1(0x04);
    BLOCK1(0x08);
    BLOCK1(0x10);
    BLOCK1(0x20);
    BLOCK1(0x40);
    BLOCK1(0x80);
    BLOCK1(0x1b);
    BLOCK1(0x36);
    rkeys[idx++] = vreinterpretq_u8_u32(temp0);
}

void arm_aes128_load_schedule(const uint8_t *key, void **_schedule) {
    *_schedule = malloc(11 * sizeof(uint8x16_t));
    // assert(*_schedule != NULL);
    uint8x16_t *schedule = (uint8x16_t *)*_schedule;
    aes_setkey_encrypt(key, schedule);
}


// AES encryption using NEON intrinsics. Round constants are 0 as the function mimics Intel AVX2 implementation,
// which applies in order: ShiftRows, SubBytes, MixColumns, AddRoundKey. 
// vaeseq_u8 applies SubBytes, ShiftRows, AddRoundKey. 
static void arm_aes128_encrypt(const uint8x16_t rkeys[11], uint8x16_t nv, unsigned char *out) {
    uint8x16_t temp = veorq_u8(nv, rkeys[0]);

    temp = vaeseq_u8(temp, vdupq_n_u8(0)); // Don't add round key here (therefore 0)
    temp = vaesmcq_u8(temp); // MixColumns
    temp = veorq_u8(temp, rkeys[1]); // AddRoundKey now

    temp = vaeseq_u8(temp, vdupq_n_u8(0));
    temp = vaesmcq_u8(temp);
    temp = veorq_u8(temp, rkeys[2]);

    temp = vaeseq_u8(temp, vdupq_n_u8(0));
    temp = vaesmcq_u8(temp);
    temp = veorq_u8(temp, rkeys[3]);

    temp = vaeseq_u8(temp, vdupq_n_u8(0));
    temp = vaesmcq_u8(temp);
    temp = veorq_u8(temp, rkeys[4]);

    temp = vaeseq_u8(temp, vdupq_n_u8(0));
    temp = vaesmcq_u8(temp);
    temp = veorq_u8(temp, rkeys[5]);

    temp = vaeseq_u8(temp, vdupq_n_u8(0));
    temp = vaesmcq_u8(temp);
    temp = veorq_u8(temp, rkeys[6]);

    temp = vaeseq_u8(temp, vdupq_n_u8(0));
    temp = vaesmcq_u8(temp);
    temp = veorq_u8(temp, rkeys[7]);

    temp = vaeseq_u8(temp, vdupq_n_u8(0));
    temp = vaesmcq_u8(temp);
    temp = veorq_u8(temp, rkeys[8]);

    temp = vaeseq_u8(temp, vdupq_n_u8(0));
    temp = vaesmcq_u8(temp);
    temp = veorq_u8(temp, rkeys[9]);

    temp = vaeseq_u8(temp, vdupq_n_u8(0));
    temp = veorq_u8(temp, rkeys[10]);
    vst1q_u8(out, temp);
}

void arm_aes128_free_schedule(void *schedule) {
    if (schedule != NULL) {
        mayo_secure_free(schedule, 11 * sizeof(uint16x8_t));
    }
}


static void arm_aes128_ctr_enc_sch(const void *schedule, uint8_t *out,
                                        size_t out_len) {
    uint8x16_t mask = {0, 1, 2, 3, 4, 5, 6, 7, 15, 14, 13, 12, 11, 10, 9, 8}; 
    uint8x16_t block = vdupq_n_u8(0); // Initialize block to zero
    while (out_len >= 16) {
        arm_aes128_encrypt(schedule, block, out);
        out += 16;
        out_len -= 16;
        block = vqtbl1q_u8(vreinterpretq_u8_u64(vaddq_u64(vreinterpretq_u64_u8(vqtbl1q_u8(block, mask)), (uint64x2_t) {0,1}) ), mask);
    }
    if (out_len > 0) {
        uint8_t tmp[16];
        arm_aes128_encrypt(schedule, block, tmp);
        memcpy(out, tmp, out_len);
    }
}

int AES_128_CTR(unsigned char *output, size_t outputByteLen,
                   const unsigned char *input) {
    void *schedule = NULL;
    arm_aes128_load_schedule(input, &schedule);
    arm_aes128_ctr_enc_sch(schedule, output, outputByteLen);
    arm_aes128_free_schedule(schedule);
    return (int)outputByteLen;
}

#define AES128_KEYBYTES 16
#define AES192_KEYBYTES 24
#define AES256_KEYBYTES 32
#define AESCTR_NONCEBYTES 12
#define AES_BLOCKBYTES 16

// We've put these states on the heap to make sure ctx_release is used.
#define PQC_AES128_STATESIZE 88
typedef struct
{
    uint64_t *sk_exp;
} aes128ctx;

#define PQC_AES192_STATESIZE 104
typedef struct
{
    uint64_t *sk_exp;
} aes192ctx;

#define PQC_AES256_STATESIZE 120
typedef struct
{
    uint64_t *sk_exp;
} aes256ctx;

/** Initializes the context **/
void aes128_ecb_keyexp(aes128ctx *r, const unsigned char *key);

void aes128_ctr_keyexp(aes128ctx *r, const unsigned char *key);

void aes128_ecb(unsigned char *out, const unsigned char *in, size_t nblocks, const aes128ctx *ctx);

void aes128_ctr(unsigned char *out, size_t outlen, const unsigned char *iv, const aes128ctx *ctx);

/** Frees the context **/
void aes128_ctx_release(aes128ctx *r);

/** Initializes the context **/
void aes192_ecb_keyexp(aes192ctx *r, const unsigned char *key);

void aes192_ctr_keyexp(aes192ctx *r, const unsigned char *key);

void aes192_ecb(unsigned char *out, const unsigned char *in, size_t nblocks, const aes192ctx *ctx);

void aes192_ctr(unsigned char *out, size_t outlen, const unsigned char *iv, const aes192ctx *ctx);

void aes192_ctx_release(aes192ctx *r);

/** Initializes the context **/
void aes256_ecb_keyexp(aes256ctx *r, const unsigned char *key);

void aes256_ctr_keyexp(aes256ctx *r, const unsigned char *key);

void aes256_ecb(unsigned char *out, const unsigned char *in, size_t nblocks, const aes256ctx *ctx);

void aes256_ctr(unsigned char *out, size_t outlen, const unsigned char *iv, const aes256ctx *ctx);

/** Frees the context **/
void aes256_ctx_release(aes256ctx *r);

static inline uint32_t br_dec32le(const unsigned char *src)
{
    return (uint32_t)src[0] | ((uint32_t)src[1] << 8) | ((uint32_t)src[2] << 16) | ((uint32_t)src[3] << 24);
}

static void br_range_dec32le(uint32_t *v, size_t num, const unsigned char *src)
{
    while (num-- > 0)
    {
        *v++ = br_dec32le(src);
        src += 4;
    }
}

static inline uint32_t br_swap32(uint32_t x)
{
    x = ((x & (uint32_t)0x00FF00FF) << 8) | ((x >> 8) & (uint32_t)0x00FF00FF);
    return (x << 16) | (x >> 16);
}

static inline void br_enc32le(unsigned char *dst, uint32_t x)
{
    dst[0] = (unsigned char)x;
    dst[1] = (unsigned char)(x >> 8);
    dst[2] = (unsigned char)(x >> 16);
    dst[3] = (unsigned char)(x >> 24);
}

static void br_range_enc32le(unsigned char *dst, const uint32_t *v, size_t num)
{
    while (num-- > 0)
    {
        br_enc32le(dst, *v++);
        dst += 4;
    }
}

static void br_aes_ct64_bitslice_Sbox(uint64_t *q)
{
    /*
     * This S-box implementation is a straightforward translation of
     * the circuit described by Boyar and Peralta in "A new
     * combinational logic minimization technique with applications
     * to cryptology" (https://eprint.iacr.org/2009/191.pdf).
     *
     * Note that variables x* (input) and s* (output) are numbered
     * in "reverse" order (x0 is the high bit, x7 is the low bit).
     */

    uint64_t x0, x1, x2, x3, x4, x5, x6, x7;
    uint64_t y1, y2, y3, y4, y5, y6, y7, y8, y9;
    uint64_t y10, y11, y12, y13, y14, y15, y16, y17, y18, y19;
    uint64_t y20, y21;
    uint64_t z0, z1, z2, z3, z4, z5, z6, z7, z8, z9;
    uint64_t z10, z11, z12, z13, z14, z15, z16, z17;
    uint64_t t0, t1, t2, t3, t4, t5, t6, t7, t8, t9;
    uint64_t t10, t11, t12, t13, t14, t15, t16, t17, t18, t19;
    uint64_t t20, t21, t22, t23, t24, t25, t26, t27, t28, t29;
    uint64_t t30, t31, t32, t33, t34, t35, t36, t37, t38, t39;
    uint64_t t40, t41, t42, t43, t44, t45, t46, t47, t48, t49;
    uint64_t t50, t51, t52, t53, t54, t55, t56, t57, t58, t59;
    uint64_t t60, t61, t62, t63, t64, t65, t66, t67;
    uint64_t s0, s1, s2, s3, s4, s5, s6, s7;

    x0 = q[7];
    x1 = q[6];
    x2 = q[5];
    x3 = q[4];
    x4 = q[3];
    x5 = q[2];
    x6 = q[1];
    x7 = q[0];

    /*
     * Top linear transformation.
     */
    y14 = x3 ^ x5;
    y13 = x0 ^ x6;
    y9 = x0 ^ x3;
    y8 = x0 ^ x5;
    t0 = x1 ^ x2;
    y1 = t0 ^ x7;
    y4 = y1 ^ x3;
    y12 = y13 ^ y14;
    y2 = y1 ^ x0;
    y5 = y1 ^ x6;
    y3 = y5 ^ y8;
    t1 = x4 ^ y12;
    y15 = t1 ^ x5;
    y20 = t1 ^ x1;
    y6 = y15 ^ x7;
    y10 = y15 ^ t0;
    y11 = y20 ^ y9;
    y7 = x7 ^ y11;
    y17 = y10 ^ y11;
    y19 = y10 ^ y8;
    y16 = t0 ^ y11;
    y21 = y13 ^ y16;
    y18 = x0 ^ y16;

    /*
     * Non-linear section.
     */
    t2 = y12 & y15;
    t3 = y3 & y6;
    t4 = t3 ^ t2;
    t5 = y4 & x7;
    t6 = t5 ^ t2;
    t7 = y13 & y16;
    t8 = y5 & y1;
    t9 = t8 ^ t7;
    t10 = y2 & y7;
    t11 = t10 ^ t7;
    t12 = y9 & y11;
    t13 = y14 & y17;
    t14 = t13 ^ t12;
    t15 = y8 & y10;
    t16 = t15 ^ t12;
    t17 = t4 ^ t14;
    t18 = t6 ^ t16;
    t19 = t9 ^ t14;
    t20 = t11 ^ t16;
    t21 = t17 ^ y20;
    t22 = t18 ^ y19;
    t23 = t19 ^ y21;
    t24 = t20 ^ y18;

    t25 = t21 ^ t22;
    t26 = t21 & t23;
    t27 = t24 ^ t26;
    t28 = t25 & t27;
    t29 = t28 ^ t22;
    t30 = t23 ^ t24;
    t31 = t22 ^ t26;
    t32 = t31 & t30;
    t33 = t32 ^ t24;
    t34 = t23 ^ t33;
    t35 = t27 ^ t33;
    t36 = t24 & t35;
    t37 = t36 ^ t34;
    t38 = t27 ^ t36;
    t39 = t29 & t38;
    t40 = t25 ^ t39;

    t41 = t40 ^ t37;
    t42 = t29 ^ t33;
    t43 = t29 ^ t40;
    t44 = t33 ^ t37;
    t45 = t42 ^ t41;
    z0 = t44 & y15;
    z1 = t37 & y6;
    z2 = t33 & x7;
    z3 = t43 & y16;
    z4 = t40 & y1;
    z5 = t29 & y7;
    z6 = t42 & y11;
    z7 = t45 & y17;
    z8 = t41 & y10;
    z9 = t44 & y12;
    z10 = t37 & y3;
    z11 = t33 & y4;
    z12 = t43 & y13;
    z13 = t40 & y5;
    z14 = t29 & y2;
    z15 = t42 & y9;
    z16 = t45 & y14;
    z17 = t41 & y8;

    /*
     * Bottom linear transformation.
     */
    t46 = z15 ^ z16;
    t47 = z10 ^ z11;
    t48 = z5 ^ z13;
    t49 = z9 ^ z10;
    t50 = z2 ^ z12;
    t51 = z2 ^ z5;
    t52 = z7 ^ z8;
    t53 = z0 ^ z3;
    t54 = z6 ^ z7;
    t55 = z16 ^ z17;
    t56 = z12 ^ t48;
    t57 = t50 ^ t53;
    t58 = z4 ^ t46;
    t59 = z3 ^ t54;
    t60 = t46 ^ t57;
    t61 = z14 ^ t57;
    t62 = t52 ^ t58;
    t63 = t49 ^ t58;
    t64 = z4 ^ t59;
    t65 = t61 ^ t62;
    t66 = z1 ^ t63;
    s0 = t59 ^ t63;
    s6 = t56 ^ ~t62;
    s7 = t48 ^ ~t60;
    t67 = t64 ^ t65;
    s3 = t53 ^ t66;
    s4 = t51 ^ t66;
    s5 = t47 ^ t65;
    s1 = t64 ^ ~s3;
    s2 = t55 ^ ~t67;

    q[7] = s0;
    q[6] = s1;
    q[5] = s2;
    q[4] = s3;
    q[3] = s4;
    q[2] = s5;
    q[1] = s6;
    q[0] = s7;
}

static void br_aes_ct64_ortho(uint64_t *q)
{
#define SWAPN(cl, ch, s, x, y)                                      \
    do                                                              \
    {                                                               \
        uint64_t a, b;                                              \
        a = (x);                                                    \
        b = (y);                                                    \
        (x) = (a & (uint64_t)(cl)) | ((b & (uint64_t)(cl)) << (s)); \
        (y) = ((a & (uint64_t)(ch)) >> (s)) | (b & (uint64_t)(ch)); \
    } while (0)

#define SWAP2(x, y) SWAPN(0x5555555555555555, 0xAAAAAAAAAAAAAAAA, 1, x, y)
#define SWAP4(x, y) SWAPN(0x3333333333333333, 0xCCCCCCCCCCCCCCCC, 2, x, y)
#define SWAP8(x, y) SWAPN(0x0F0F0F0F0F0F0F0F, 0xF0F0F0F0F0F0F0F0, 4, x, y)

    SWAP2(q[0], q[1]);
    SWAP2(q[2], q[3]);
    SWAP2(q[4], q[5]);
    SWAP2(q[6], q[7]);

    SWAP4(q[0], q[2]);
    SWAP4(q[1], q[3]);
    SWAP4(q[4], q[6]);
    SWAP4(q[5], q[7]);

    SWAP8(q[0], q[4]);
    SWAP8(q[1], q[5]);
    SWAP8(q[2], q[6]);
    SWAP8(q[3], q[7]);
}

static void br_aes_ct64_interleave_in(uint64_t *q0, uint64_t *q1, const uint32_t *w)
{
    uint64_t x0, x1, x2, x3;

    x0 = w[0];
    x1 = w[1];
    x2 = w[2];
    x3 = w[3];
    x0 |= (x0 << 16);
    x1 |= (x1 << 16);
    x2 |= (x2 << 16);
    x3 |= (x3 << 16);
    x0 &= (uint64_t)0x0000FFFF0000FFFF;
    x1 &= (uint64_t)0x0000FFFF0000FFFF;
    x2 &= (uint64_t)0x0000FFFF0000FFFF;
    x3 &= (uint64_t)0x0000FFFF0000FFFF;
    x0 |= (x0 << 8);
    x1 |= (x1 << 8);
    x2 |= (x2 << 8);
    x3 |= (x3 << 8);
    x0 &= (uint64_t)0x00FF00FF00FF00FF;
    x1 &= (uint64_t)0x00FF00FF00FF00FF;
    x2 &= (uint64_t)0x00FF00FF00FF00FF;
    x3 &= (uint64_t)0x00FF00FF00FF00FF;
    *q0 = x0 | (x2 << 8);
    *q1 = x1 | (x3 << 8);
}

static void br_aes_ct64_interleave_out(uint32_t *w, uint64_t q0, uint64_t q1)
{
    uint64_t x0, x1, x2, x3;

    x0 = q0 & (uint64_t)0x00FF00FF00FF00FF;
    x1 = q1 & (uint64_t)0x00FF00FF00FF00FF;
    x2 = (q0 >> 8) & (uint64_t)0x00FF00FF00FF00FF;
    x3 = (q1 >> 8) & (uint64_t)0x00FF00FF00FF00FF;
    x0 |= (x0 >> 8);
    x1 |= (x1 >> 8);
    x2 |= (x2 >> 8);
    x3 |= (x3 >> 8);
    x0 &= (uint64_t)0x0000FFFF0000FFFF;
    x1 &= (uint64_t)0x0000FFFF0000FFFF;
    x2 &= (uint64_t)0x0000FFFF0000FFFF;
    x3 &= (uint64_t)0x0000FFFF0000FFFF;
    w[0] = (uint32_t)x0 | (uint32_t)(x0 >> 16);
    w[1] = (uint32_t)x1 | (uint32_t)(x1 >> 16);
    w[2] = (uint32_t)x2 | (uint32_t)(x2 >> 16);
    w[3] = (uint32_t)x3 | (uint32_t)(x3 >> 16);
}

static const unsigned char Rcon[] = {
    0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36};

static uint32_t sub_word(uint32_t x)
{
    uint64_t q[8];

    memset(q, 0, sizeof q);
    q[0] = x;
    br_aes_ct64_ortho(q);
    br_aes_ct64_bitslice_Sbox(q);
    br_aes_ct64_ortho(q);
    return (uint32_t)q[0];
}

static void br_aes_ct64_keysched(uint64_t *comp_skey, const unsigned char *key, unsigned int key_len)
{
    unsigned int i, j, k, nk, nkf;
    uint32_t tmp;
    uint32_t skey[60];
    unsigned nrounds = 10 + ((key_len - 16) >> 2);

    nk = (key_len >> 2);
    nkf = ((nrounds + 1) << 2);
    br_range_dec32le(skey, (key_len >> 2), key);
    tmp = skey[(key_len >> 2) - 1];
    for (i = nk, j = 0, k = 0; i < nkf; i++)
    {
        if (j == 0)
        {
            tmp = (tmp << 24) | (tmp >> 8);
            tmp = sub_word(tmp) ^ Rcon[k];
        }
        else if (nk > 6 && j == 4)
        {
            tmp = sub_word(tmp);
        }
        tmp ^= skey[i - nk];
        skey[i] = tmp;
        if (++j == nk)
        {
            j = 0;
            k++;
        }
    }

    for (i = 0, j = 0; i < nkf; i += 4, j += 2)
    {
        uint64_t q[8];

        br_aes_ct64_interleave_in(&q[0], &q[4], skey + i);
        q[1] = q[0];
        q[2] = q[0];
        q[3] = q[0];
        q[5] = q[4];
        q[6] = q[4];
        q[7] = q[4];
        br_aes_ct64_ortho(q);
        comp_skey[j + 0] =
            (q[0] & (uint64_t)0x1111111111111111) | (q[1] & (uint64_t)0x2222222222222222) | (q[2] & (uint64_t)0x4444444444444444) | (q[3] & (uint64_t)0x8888888888888888);
        comp_skey[j + 1] =
            (q[4] & (uint64_t)0x1111111111111111) | (q[5] & (uint64_t)0x2222222222222222) | (q[6] & (uint64_t)0x4444444444444444) | (q[7] & (uint64_t)0x8888888888888888);
    }
}

static void br_aes_ct64_skey_expand(uint64_t *skey, const uint64_t *comp_skey, unsigned int nrounds)
{
    unsigned u, v, n;

    n = (nrounds + 1) << 1;
    for (u = 0, v = 0; u < n; u++, v += 4)
    {
        uint64_t x0, x1, x2, x3;

        x0 = x1 = x2 = x3 = comp_skey[u];
        x0 &= (uint64_t)0x1111111111111111;
        x1 &= (uint64_t)0x2222222222222222;
        x2 &= (uint64_t)0x4444444444444444;
        x3 &= (uint64_t)0x8888888888888888;
        x1 >>= 1;
        x2 >>= 2;
        x3 >>= 3;
        skey[v + 0] = (x0 << 4) - x0;
        skey[v + 1] = (x1 << 4) - x1;
        skey[v + 2] = (x2 << 4) - x2;
        skey[v + 3] = (x3 << 4) - x3;
    }
}

static inline void add_round_key(uint64_t *q, const uint64_t *sk)
{
    q[0] ^= sk[0];
    q[1] ^= sk[1];
    q[2] ^= sk[2];
    q[3] ^= sk[3];
    q[4] ^= sk[4];
    q[5] ^= sk[5];
    q[6] ^= sk[6];
    q[7] ^= sk[7];
}

static inline void shift_rows(uint64_t *q)
{
    int i;

    for (i = 0; i < 8; i++)
    {
        uint64_t x;

        x = q[i];
        q[i] = (x & (uint64_t)0x000000000000FFFF) | ((x & (uint64_t)0x00000000FFF00000) >> 4) | ((x & (uint64_t)0x00000000000F0000) << 12) | ((x & (uint64_t)0x0000FF0000000000) >> 8) | ((x & (uint64_t)0x000000FF00000000) << 8) | ((x & (uint64_t)0xF000000000000000) >> 12) | ((x & (uint64_t)0x0FFF000000000000) << 4);
    }
}

static inline uint64_t rotr32(uint64_t x)
{
    return (x << 32) | (x >> 32);
}

static inline void mix_columns(uint64_t *q)
{
    uint64_t q0, q1, q2, q3, q4, q5, q6, q7;
    uint64_t r0, r1, r2, r3, r4, r5, r6, r7;

    q0 = q[0];
    q1 = q[1];
    q2 = q[2];
    q3 = q[3];
    q4 = q[4];
    q5 = q[5];
    q6 = q[6];
    q7 = q[7];
    r0 = (q0 >> 16) | (q0 << 48);
    r1 = (q1 >> 16) | (q1 << 48);
    r2 = (q2 >> 16) | (q2 << 48);
    r3 = (q3 >> 16) | (q3 << 48);
    r4 = (q4 >> 16) | (q4 << 48);
    r5 = (q5 >> 16) | (q5 << 48);
    r6 = (q6 >> 16) | (q6 << 48);
    r7 = (q7 >> 16) | (q7 << 48);

    q[0] = q7 ^ r7 ^ r0 ^ rotr32(q0 ^ r0);
    q[1] = q0 ^ r0 ^ q7 ^ r7 ^ r1 ^ rotr32(q1 ^ r1);
    q[2] = q1 ^ r1 ^ r2 ^ rotr32(q2 ^ r2);
    q[3] = q2 ^ r2 ^ q7 ^ r7 ^ r3 ^ rotr32(q3 ^ r3);
    q[4] = q3 ^ r3 ^ q7 ^ r7 ^ r4 ^ rotr32(q4 ^ r4);
    q[5] = q4 ^ r4 ^ r5 ^ rotr32(q5 ^ r5);
    q[6] = q5 ^ r5 ^ r6 ^ rotr32(q6 ^ r6);
    q[7] = q6 ^ r6 ^ r7 ^ rotr32(q7 ^ r7);
}

static void inc4_be(uint32_t *x)
{
    uint32_t t = br_swap32(*x) + 4;
    *x = br_swap32(t);
}

static void aes_ecb4x(unsigned char out[64], const uint32_t ivw[16], const uint64_t *sk_exp, unsigned int nrounds)
{
    uint32_t w[16];
    uint64_t q[8];
    unsigned int i;

    memcpy(w, ivw, sizeof(w));
    for (i = 0; i < 4; i++)
    {
        br_aes_ct64_interleave_in(&q[i], &q[i + 4], w + (i << 2));
    }
    br_aes_ct64_ortho(q);

    add_round_key(q, sk_exp);
    for (i = 1; i < nrounds; i++)
    {
        br_aes_ct64_bitslice_Sbox(q);
        shift_rows(q);
        mix_columns(q);
        add_round_key(q, sk_exp + (i << 3));
    }
    br_aes_ct64_bitslice_Sbox(q);
    shift_rows(q);
    add_round_key(q, sk_exp + 8 * nrounds);

    br_aes_ct64_ortho(q);
    for (i = 0; i < 4; i++)
    {
        br_aes_ct64_interleave_out(w + (i << 2), q[i], q[i + 4]);
    }
    br_range_enc32le(out, w, 16);
}

static void aes_ctr4x(unsigned char out[64], uint32_t ivw[16], const uint64_t *sk_exp, unsigned int nrounds)
{
    aes_ecb4x(out, ivw, sk_exp, nrounds);

    /* Increase counter for next 4 blocks */
    inc4_be(ivw + 3);
    inc4_be(ivw + 7);
    inc4_be(ivw + 11);
    inc4_be(ivw + 15);
}

static void aes_ecb(unsigned char *out, const unsigned char *in, size_t nblocks, const uint64_t *rkeys, unsigned int nrounds)
{
    uint32_t blocks[16];
    unsigned char t[64];

    while (nblocks >= 4)
    {
        br_range_dec32le(blocks, 16, in);
        aes_ecb4x(out, blocks, rkeys, nrounds);
        nblocks -= 4;
        in += 64;
        out += 64;
    }

    if (nblocks)
    {
        br_range_dec32le(blocks, nblocks * 4, in);
        aes_ecb4x(t, blocks, rkeys, nrounds);
        memcpy(out, t, nblocks * 16);
    }
}

static void aes_ctr(unsigned char *out, size_t outlen, const unsigned char *iv, const uint64_t *rkeys, unsigned int nrounds)
{
    uint32_t ivw[16];
    size_t i;
    uint32_t cc = 0;

    br_range_dec32le(ivw, 3, iv);
    memcpy(ivw + 4, ivw, 3 * sizeof(uint32_t));
    memcpy(ivw + 8, ivw, 3 * sizeof(uint32_t));
    memcpy(ivw + 12, ivw, 3 * sizeof(uint32_t));
    ivw[3] = br_swap32(cc);
    ivw[7] = br_swap32(cc + 1);
    ivw[11] = br_swap32(cc + 2);
    ivw[15] = br_swap32(cc + 3);

    while (outlen > 64)
    {
        aes_ctr4x(out, ivw, rkeys, nrounds);
        out += 64;
        outlen -= 64;
    }
    if (outlen > 0)
    {
        unsigned char tmp[64];
        aes_ctr4x(tmp, ivw, rkeys, nrounds);
        for (i = 0; i < outlen; i++)
        {
            out[i] = tmp[i];
        }
    }
}

void aes128_ecb_keyexp(aes128ctx *r, const unsigned char *key)
{
    uint64_t skey[22];

    r->sk_exp = malloc(sizeof(uint64_t) * PQC_AES128_STATESIZE);
    if (r->sk_exp == NULL)
    {
        exit(111);
    }

    br_aes_ct64_keysched(skey, key, 16);
    br_aes_ct64_skey_expand(r->sk_exp, skey, 10);
}

void aes128_ctr_keyexp(aes128ctx *r, const unsigned char *key)
{
    aes128_ecb_keyexp(r, key);
}

void aes192_ecb_keyexp(aes192ctx *r, const unsigned char *key)
{
    uint64_t skey[26];
    r->sk_exp = malloc(sizeof(uint64_t) * PQC_AES192_STATESIZE);
    if (r->sk_exp == NULL)
    {
        exit(111);
    }

    br_aes_ct64_keysched(skey, key, 24);
    br_aes_ct64_skey_expand(r->sk_exp, skey, 12);
}

void aes192_ctr_keyexp(aes192ctx *r, const unsigned char *key)
{
    aes192_ecb_keyexp(r, key);
}

void aes256_ecb_keyexp(aes256ctx *r, const unsigned char *key)
{
    uint64_t skey[30];
    r->sk_exp = malloc(sizeof(uint64_t) * PQC_AES256_STATESIZE);
    if (r->sk_exp == NULL)
    {
        exit(111);
    }

    br_aes_ct64_keysched(skey, key, 32);
    br_aes_ct64_skey_expand(r->sk_exp, skey, 14);
}

void aes256_ctr_keyexp(aes256ctx *r, const unsigned char *key)
{
    aes256_ecb_keyexp(r, key);
}

void aes128_ecb(unsigned char *out, const unsigned char *in, size_t nblocks, const aes128ctx *ctx)
{
    aes_ecb(out, in, nblocks, ctx->sk_exp, 10);
}

void aes128_ctr(unsigned char *out, size_t outlen, const unsigned char *iv, const aes128ctx *ctx)
{
    aes_ctr(out, outlen, iv, ctx->sk_exp, 10);
}

void aes192_ecb(unsigned char *out, const unsigned char *in, size_t nblocks, const aes192ctx *ctx)
{
    aes_ecb(out, in, nblocks, ctx->sk_exp, 12);
}

void aes192_ctr(unsigned char *out, size_t outlen, const unsigned char *iv, const aes192ctx *ctx)
{
    aes_ctr(out, outlen, iv, ctx->sk_exp, 12);
}

void aes256_ecb(unsigned char *out, const unsigned char *in, size_t nblocks, const aes256ctx *ctx)
{
    aes_ecb(out, in, nblocks, ctx->sk_exp, 14);
}

void aes256_ctr(unsigned char *out, size_t outlen, const unsigned char *iv, const aes256ctx *ctx)
{
    aes_ctr(out, outlen, iv, ctx->sk_exp, 14);
}

void aes128_ctx_release(aes128ctx *r)
{
    free(r->sk_exp);
}

void aes192_ctx_release(aes192ctx *r)
{
    free(r->sk_exp);
}

void aes256_ctx_release(aes256ctx *r)
{
    free(r->sk_exp);
}


void AES_256_ECB(const uint8_t *input, const unsigned char *key, unsigned char *output)
{
    aes256ctx ctx;

    aes256_ecb_keyexp(&ctx, key);
    aes256_ecb(output, input, 1, &ctx);
    aes256_ctx_release(&ctx);
}