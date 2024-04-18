fn main() {
    // Compile C-files file into a separate static library
    cc::Build::new()
        .file("src/genkat/randombytes_ctrdrbg.c")
        .file("src/genkat/mem.c")
        .file("src/genkat/aes_c.c")
        .file("src/genkat/fips202.c")
        .file("src/assembly/vmull.c")
        .flag("-O3")  // Apply optimization level O3
        .compile("randombytes_nist");


    // Compile Assembly file into a separate static library

}