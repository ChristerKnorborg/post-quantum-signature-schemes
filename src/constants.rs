#[cfg(feature = "mayo1")]
mod mayo1_features {
    pub const N: usize = 66;
    pub const M: usize = 64;
    pub const O: usize = 8;
    pub const K: usize = 9;
    pub const Q: usize = 16;
    pub const M_BYTES: usize = 32;
    pub const O_BYTES: usize = 232;
    pub const V_BYTES: usize = 29;
    pub const R_BYTES: usize = 24;
    pub const P1_BYTES: usize = 54752;
    pub const P2_BYTES: usize = 14848;
    pub const P3_BYTES: usize = 1152;
    pub const CSK_BYTES: usize = 24;
    pub const ESK_BYTES: usize = 69856;
    pub const CPK_BYTES: usize = 1168;
    pub const EPK_BYTES: usize = 70752;
    pub const SIG_BYTES: usize = 321;
    pub const SALT_BYTES: usize = 24;
    pub const DIGEST_BYTES: usize = 32;
    pub const PK_SEED_BYTES: usize = 16;
    pub const SK_SEED_BYTES: usize = 24;
    pub const L_BYTES: usize = M * (N - O) * O / 2;
}

#[cfg(feature = "mayo2")]
mod mayo1_features {
    pub const N: usize = 78;
    pub const M: usize = 64;
    pub const O: usize = 18;
    pub const K: usize = 4;
    pub const Q: usize = 16;
    pub const M_BYTES: usize = 32;
    pub const O_BYTES: usize = 540;
    pub const V_BYTES: usize = 30;
    pub const R_BYTES: usize = 24;
    pub const P1_BYTES: usize = 58560;
    pub const P2_BYTES: usize = 34560;
    pub const P3_BYTES: usize = 5472;
    pub const CSK_BYTES: usize = 24;
    pub const ESK_BYTES: usize = 93684;
    pub const CPK_BYTES: usize = 5488;
    pub const EPK_BYTES: usize = 98592;
    pub const SIG_BYTES: usize = 180;
    pub const SALT_BYTES: usize = 24;
    pub const DIGEST_BYTES: usize = 32;
    pub const PK_SEED_BYTES: usize = 16;
    pub const SK_SEED_BYTES: usize = 24;
    pub const L_BYTES: usize = M * (N - O) * O / 2;
}

#[cfg(feature = "mayo3")]
mod mayo1_features {
    pub const N: usize = 99;
    pub const M: usize = 96;
    pub const O: usize = 10;
    pub const K: usize = 11;
    pub const Q: usize = 16;
    pub const M_BYTES: usize = 48;
    pub const O_BYTES: usize = 445;
    pub const V_BYTES: usize = 45;
    pub const R_BYTES: usize = 32;
    pub const P1_BYTES: usize = 192240;
    pub const P2_BYTES: usize = 42720;
    pub const P3_BYTES: usize = 2640;
    pub const CSK_BYTES: usize = 32;
    pub const ESK_BYTES: usize = 235437;
    pub const CPK_BYTES: usize = 2656;
    pub const EPK_BYTES: usize = 237600;
    pub const SIG_BYTES: usize = 577;
    pub const SALT_BYTES: usize = 32;
    pub const DIGEST_BYTES: usize = 48;
    pub const PK_SEED_BYTES: usize = 16;
    pub const SK_SEED_BYTES: usize = 32;
    pub const L_BYTES: usize = M * (N - O) * O / 2;
}

#[cfg(feature = "mayo5")]
mod mayo1_features {
    pub const N: usize = 133;
    pub const M: usize = 128;
    pub const O: usize = 12;
    pub const K: usize = 12;
    pub const Q: usize = 16;
    pub const M_BYTES: usize = 64;
    pub const O_BYTES: usize = 726;
    pub const V_BYTES: usize = 61;
    pub const R_BYTES: usize = 40;
    pub const P1_BYTES: usize = 472384;
    pub const P2_BYTES: usize = 92928;
    pub const P3_BYTES: usize = 4992;
    pub const CSK_BYTES: usize = 40;
    pub const ESK_BYTES: usize = 566078;
    pub const CPK_BYTES: usize = 5008;
    pub const EPK_BYTES: usize = 570304;
    pub const SIG_BYTES: usize = 838;
    pub const SALT_BYTES: usize = 40;
    pub const DIGEST_BYTES: usize = 64;
    pub const PK_SEED_BYTES: usize = 16;
    pub const SK_SEED_BYTES: usize = 40;
    pub const L_BYTES: usize = M * (N - O) * O / 2;
}

#[cfg(not(feature = "mayo1"))]
mod other_features {

    pub const N: usize = 66;
    pub const M: usize = 64;
    pub const O: usize = 8;
    pub const K: usize = 9;
    pub const Q: usize = 16;
    pub const M_BYTES: usize = 32;
    pub const O_BYTES: usize = 232;
    pub const O_BYTES_MAX: usize = 726;
    pub const V_BYTES: usize = 29;
    pub const R_BYTES: usize = 24;
    pub const P1_BYTES: usize = 54752;
    pub const P2_BYTES: usize = 14848;
    pub const P3_BYTES: usize = 1152;
    pub const CSK_BYTES: usize = 24;
    pub const ESK_BYTES: usize = 69856;
    pub const CPK_BYTES: usize = 1168;
    pub const EPK_BYTES: usize = 70752;
    pub const SIG_BYTES: usize = 321;
    pub const SALT_BYTES: usize = 24;
    pub const DIGEST_BYTES: usize = 32;
    pub const PK_SEED_BYTES: usize = 16;
    pub const SK_SEED_BYTES: usize = 24;
    pub const L_BYTES: usize = M * (N - O) * O / 2;

    // Compact representation of irreducible polynomial [z^0 + z^1 + z^2 + z^3 + z^m]
    pub const F_Z: [u8; 5] = [8, 0, 2, 8, 0]; // f(z) =  z^64         + x^3*z^3 + x*z^2         + x^3
}

// Re-export the constants from the included module so they can be accessed directly
// through this common `constants` module.
#[cfg(feature = "mayo1")]
pub use mayo1_features::*;

#[cfg(not(feature = "mayo1"))]
pub use other_features::*;

// thread_local! {
//     pub const I: std::cell::Cell<u8> = std::cell::Cell::new(1);
//     pub const K: std::cell::Cell<u8> = std::cell::Cell::new(2);
//     pub const O: std::cell::Cell<u8> = std::cell::Cell::new(4);
// }
