## Description
This repository contains the implementation of the MAYO signature scheme in the Rust programming language. The implementation was part of our Master's thesis project with the goal of exploring efficient implementations of the scheme, particularly focused on optimizations targeting the ARMv8 architecture using NEON intrinsics. Parts of the code is inspired by the MAYO authors implementation found at: https://github.com/PQCMayo/MAYO-C. Furthermore, their specification can be found at: https://pqmayo.org/. 

The different branches contains improvements throughout our implementation process. However, we have combined the most effective and best performing approaches on the main branch targeting the ARMv8 architecture. 



### Generate and Compare with Known Answer Test File
The code can be executed using the Rust commands below, which runs the MAYO authors suggested parameter versions dependent on the command. Each of these executions runs the algorithms of the MAYO scheme using the NIST test vectors and stores the input and output in a txt-file. 
This is then compared to the KAT-file of the authors NIST submission. We emphasize the file is deleted afterwards if no differences are present.

`cargo mayo1`  
`cargo mayo2`  
`cargo mayo3`  
`cargo mayo5`

Furthermore, the scheme can be executed with our ARMv8 intrinsics AES implementation using the commands:

`cargo mayo1_aes`  
`cargo mayo2_aes`  
`cargo mayo3_aes`  
`cargo mayo5_aes`

### Benchmark

To run 1000 samples of timings the following commands can be executed:

`cargo bench_mayo1`  
`cargo bench_mayo2`  
`cargo bench_mayo3`  
`cargo bench_mayo5`

Each of

