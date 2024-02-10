
#[cfg(feature = "mayo1")]
mod mayo1_features {
    pub static  N: u8 = 66;
    pub static  M: u8 = 64;
    pub static  O: u8 = 8;
    pub static  K: u8 = 9;
    pub static SALT_BYTES: u8 = 24;
    pub static DIGEST_BYTES: u8 = 32;
    pub static PK_SEED_BYTES: u8 = 16;
    }



#[cfg(not(feature = "mayo1"))]
mod other_features {
    pub static I: u8 = 1;
    pub static K: u8 = 2;
    pub static O: u8 = 4;
}


// Re-export the constants from the included module so they can be accessed directly
// through this common `constants` module.
#[cfg(feature = "mayo1")]
pub use mayo1_features::*;

#[cfg(not(feature = "mayo1"))]
pub use other_features::*;


// thread_local! {
//     pub static I: std::cell::Cell<u8> = std::cell::Cell::new(1);
//     pub static K: std::cell::Cell<u8> = std::cell::Cell::new(2);
//     pub static O: std::cell::Cell<u8> = std::cell::Cell::new(4);
// }

