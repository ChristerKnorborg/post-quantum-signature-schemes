


#[cfg(feature = "mayo1")]
mod mayo1_features {
    pub const  N: usize = 66;
    pub const  M: usize = 64;
    pub const  O: usize = 8;
    pub const  K: usize = 9;
    
    pub const SALT_BYTES: usize = 24;
    pub const DIGEST_BYTES: usize = 32;
    pub const PK_SEED_BYTES: usize = 16;
    }



#[cfg(not(feature = "mayo1"))]
mod other_features {

    pub const N: usize = 66;
    pub const I: usize = 1;
    pub const K: usize = 9;
    pub const O: usize = 8;
    pub const  M: usize = 64;


    pub const SALT_BYTES: usize = 24;
    pub const R_BYTES: usize  = SALT_BYTES;
    pub const SK_SEED_BYTES: usize = SALT_BYTES;  

    pub const  PK_SEED_BYTES: usize = 16;
    pub const DIGEST_BYTES: usize= 32;

    // Compact Representiation of secret key
    pub const CSK_BYTES: usize = SK_SEED_BYTES;

    // ceil( (N - O)*O /2 )
    pub const O_BYTES: usize = if ((N - O) * O) % 2 == 0 { ((N - O) * O) / 2 } else { ((N - O) * O) / 2 + 1 }; 
    
    // ceil( (N - O) /2 )
    pub const V_BYTES: usize = if (N - O) % 2 == 0 { (N - O) / 2 } else { (N - O) / 2 + 1 }; 

    // Formula for P1 is m * binom(n-o+1, 2) /2
    pub const P1_BYTES: usize = M * 1711 / 2; // 1711 = binom(66-8+1, 2) 

    pub const P2_BYTES: usize = M*(N - O)*O /2;
    pub const P3_BYTES: usize = M * 6 /2;
    pub const L_BYTES: usize = M*(N - O)*O /2;

    // Expanded Representation of Secret key
    pub const ESK_BYTES: usize = SK_SEED_BYTES + O_BYTES + P1_BYTES +  L_BYTES; 

    // Compact Representation of Public key
    pub const CPK_BYTES: usize = PK_SEED_BYTES + P3_BYTES;

    // Expanded Representation of Public key
    pub const EPK_BYTES: usize = P1_BYTES + P2_BYTES + P3_BYTES;

    // ceil( (N*K / 2) )  + SALT_BYTES
    pub const SIG_BYTES: usize = if (N*K  % 2) == 0 { (N*K / 2) + SALT_BYTES } else { (N*K / 2) + SALT_BYTES + 1 };


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

