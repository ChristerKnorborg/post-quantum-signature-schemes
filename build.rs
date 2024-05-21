fn main() {
    let mut build = cc::Build::new();

    // Check if aes_neon is enabled
    if std::env::var("CARGO_FEATURE_AES_NEON").is_ok() {
        println!("cargo:info=Using AES with NEON intrinsics");
        build.file("src/genkat/aes_arm.c");
        build.flag_if_supported("-march=armv8-a+crypto"); // Enable Cryptography extensions

    } else {

        println!("cargo:info=Using default AES");
        build.file("src/genkat/aes_c.c");
    }

    build.file("src/genkat/randombytes_ctrdrbg.c")
        .file("src/genkat/mem.c")
        .file("src/genkat/fips202.c")
        .flag("-O3")
        .compile("randombytes_nist");
}