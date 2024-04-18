// build.rs
fn main() {
    let mut build = cc::Build::new();

    // Use conditional compilation based on the feature
    if std::env::var("CARGO_FEATURE_SYSTEM_AES").is_ok() {
        println!("cargo:info=Using system AES");
        build.file("src/genkat/randombytes_system.c");
    } else {
        println!("cargo:info=Using default AES");
        build.file("src/genkat/randombytes_ctrdrbg.c");
    }

    build.file("src/genkat/mem.c")
        .file("src/genkat/aes_c.c")
        .file("src/genkat/fips202.c")
        .flag("-O3")
        .flag("-mcpu=apple-m1")
        .compile("randombytes_nist");
}
