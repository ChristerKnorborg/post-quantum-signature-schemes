// build.rs
fn main() {
    cc::Build::new()
        .file("src/genKAT/randombytes_ctrdrbg.c")
        .file("src/genKAT/mem.c")
        .file("src/genKAT/aes_c.c")
        .file("src/genKAT/fips202.c")
        .compile("randombytes_nist");
}
