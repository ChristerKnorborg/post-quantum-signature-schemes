## Description
This repository contains the implementation of the MAYO signature scheme in the Rust programming language. The implementation was part of our Master's thesis project with the goal of exploring efficient implementations of the scheme, particularly focused on optimizations targeting the ARMv8 architecture using NEON intrinsics. Parts of the code is inspired by the MAYO authors implementation found at: https://github.com/PQCMayo/MAYO-C. Furthermore, their specification can be found at: https://pqmayo.org/. 

The different branches contains improvements throughout our implementation process. However, we have combined the most effective and best performing approaches on the main branch targeting the ARMv8 architecture. 

The code can be executed using the following Rust commands, which run the MAYO authors suggested parameter versions:

`cargo mayo1`  
`cargo mayo2`  
`cargo mayo3`  
`cargo mayo5`

Furthermore, the scheme can be executed with our native ARM AES implementation using the commands:

`cargo mayo1_aes`  
`cargo mayo2_aes`  
`cargo mayo3_aes`  
`cargo mayo5_aes`
