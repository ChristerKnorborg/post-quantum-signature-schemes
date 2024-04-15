// build.rs
fn main() {
    cc::Build::new()
        .file("src/genkat/randombytes_ctrdrbg.c")
        .file("src/genkat/mem.c")
        .file("src/genkat/aes_c.c")
        .file("src/genkat/fips202.c")
        .flag("-O2")
        .compile("randombytes_nist");
}
