//pub static i: u8;
//pub static o: u8;

#[cfg(feature = "mayo1")]
pub static  I: u8 = 2;

#[cfg(feature = "mayo1")]
pub static  K: u8 = 4;

#[cfg(feature = "mayo1")]
pub static  O: u8 = 8;

#[cfg(not(feature = "mayo1"))]
pub static  I: u8 = 1;

#[cfg(not(feature = "mayo1"))]
pub static  K: u8 = 2;

#[cfg(not(feature = "mayo1"))]
pub static  O: u8 = 4;



// thread_local! {
//     pub static I: std::cell::Cell<u8> = std::cell::Cell::new(1);
//     pub static K: std::cell::Cell<u8> = std::cell::Cell::new(2);
//     pub static O: std::cell::Cell<u8> = std::cell::Cell::new(4);
// }

