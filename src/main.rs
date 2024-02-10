mod sample;
mod finite_field;
mod utils;
mod constants;
mod bitsliced_functionality;
mod MAYO_functionality;





fn main() {

    let (sk, pk) = MAYO_functionality::compact_key_gen();
    
    println!("Secret key: {:?}", sk);
    println!("Public key: {:?}", pk);
    


}