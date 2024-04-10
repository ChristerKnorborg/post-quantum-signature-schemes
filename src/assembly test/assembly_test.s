.global _asm_function
// The parameters are pointers to the arrays/vectors in memory.
// X0 = address of res[V-1]
// X1 = address of p1[V-1]
// X2 = address of final_o_vec

_asm_function:
    // Load the last row of p1 into NEON register V0
    LD1 {V0.8B}, [X1]

    // Load final_o_vec into NEON register V1 (D = 64 bit register)
    LD1 {V1.8B}, [X2]

    // Perform the polynomial vector multiplication and store in Q (Q = 128 bit register)
    VMULL V2.8B, V1.8B, V0.8B  

    // Store the result back into the last row of res
    ST1 {V2.2S}, [X0]

    RET
