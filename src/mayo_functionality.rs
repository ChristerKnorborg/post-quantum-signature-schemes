use sha3::{Shake256, Digest};
use sha3::digest::Update;


// Function to hash a bytestring with SHAKE256 to a specified output length
pub fn shake256(bytestring: Vec<u8>, output_length: usize) {

    let input = b"hello world";
    let mut hasher = Shake256::default();

    input.hash(&mut hasher);
    let hashed = hasher.();
    println!("{:?}", hashed);
}


// MAYO algorithm 5: 
pub fn compact_key_gen() {

    let sk_seed_bytes = 16;
    


    // Pick random seed of byte length sk_seed_bytes for secret key


    // let mut sk_seed = Vec::with_capacity(SALT_BYTES);
    // let mut rng = rand::thread_rng();
    // rng.fill_bytes(&mut sk_seed);
    


}