#include <stdio.h>
#include <stdlib.h>

#ifdef __ARM_NEON
#include <arm_neon.h>
#endif /* __ARM_NEON */

#define __STDC_FORMAT_MACROS

// extern void asm_function(unsigned long long *res, unsigned long long *p1, unsigned long long *final_o_vec);

void print_result(int64_t *arr, int n) {
    printf("[");
    for (int i = 0; i < n; i++) {
        printf("%lld", arr[i]);
        if (i < n - 1) {
            printf(", ");
        }
    }
    printf("]\n");
}

void print_array(int32_t *arr, int n) {
    printf("[");
    for (int i = 0; i < n; i++) {
        printf("%d", arr[i]);
        if (i < n - 1) {
            printf(", ");
        }
    }
    printf("]\n");
}


void qadd_values(int32_t *a, int32_t *b, int n) {
    int i;

    for (i = 0; i < (n & ~3); i+=4) {
        vst1q_s32(&a[i], vqaddq_s32(vld1q_s32(&a[i]), vld1q_s32(&b[i])));
    }
    if (i & 2) {
        vst1_s32(&a[i], vqadd_s32(vld1_s32(&a[i]), vld1_s32(&b[i])));
        i += 2;
    }
    if (i & 1) {
        a[i] = vqadds_s32(a[i], b[i]);
    }
}

void vmull_values(int32_t *a, int32_t *b, int64_t *result, int n) {
    int i;

    for (i = 0; i < (n & ~3); i+=2) {
        int32x2_t va = vld1_s32(&a[i]); // Load 2 elements from array a
        int32x2_t vb = vld1_s32(&b[i]); // Load 2 elements from array b
        int64x2_t res = vmull_u32(va, vb); // Perform widening multiplication
        vst1q_s64(&result[i], res); // Store the result in the result array
    }
    // Handle remaining elements (if any)
    for (; i < n; ++i) {
        result[i] = (int64_t)a[i] * (int64_t)b[i];
    }
}

void test_neon() {
    // Test cases
    int32_t a[] = {2, 3, 3, 4, 10};
    int32_t b[] = {5, 6, 10, 8, 5};
    int64_t result[5];
    int n = 5;

    printf("Array a: ");
    print_array(a, n);
    printf("Array b: ");
    print_array(b, n);

    vmull_values(a, b, result, n);

    printf("Result: ");
    print_result(result, n);
}

int main() {

    
  /*  // Dynamically allocate memory for each variable
    unsigned long long *res = malloc(sizeof(unsigned long long)); // Allocate memory for result
    unsigned long long *p1 = malloc(sizeof(unsigned long long)); // Allocate memory for p1
    unsigned long long *final_o_vec = malloc(sizeof(unsigned long long)); // Allocate memory for final_o_vec

    // Initialize your variables as needed
    *res = 0;
    *p1 = 0xFFFFFFFF; // Last row of p1 with example initialization
    *final_o_vec = 2; // Vector to be multiplied with example value

    // Call assembly function with pointers
    asm_function(res, p1, final_o_vec);

    // Print the result
    printf("Result: [%llu]\n", *res);

    // Free the allocated memory
    free(res);
    free(p1);
    free(final_o_vec); */
    
    test_neon();

    return 0;
}
