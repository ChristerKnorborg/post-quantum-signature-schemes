// build.rs
fn main() {
    cc::Build::new()
        .file("src/genKAT/randombytes_ctrdrbg.c")
        .file("src/genKAT/aes_c.c")
        .compile("randombytes_nist");
}
