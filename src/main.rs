use lib::{crypto_primitives::safe_encode_bit_sliced_array_mayo12, encode_bit_sliced_array, write_and_compare_kat_file::write_and_compare_kat_file};

// fn main() {
//     write_and_compare_kat_file();
// }


fn main() {
    

    // make example. Add other values for clarity
    let mut test = [0u8; 32];
    //test[0] = 1;
    test[31] = 0b1111;
    test[30] = 0b1111;

    println!("test:");
    for &byte in test.iter() {
        print!("{:08b}", byte); // Print each byte in binary, padded to 8 bits
    }


    let plain = encode_bit_sliced_array!(test, 32);

    println!();
    println!("plain:");
    for &byte in plain.iter() {
        print!("{:08b}", byte);
    }

    let mut modified_test = [0u8; 32/2]; // Half the size of the original, as we're combining every two nibbles
    for i in (0..test.len()).step_by(2) {
        let first_nibble = test[i] & 0x0F;      // Last 4 bits of the first byte
        let second_nibble = test[i+1] & 0x0F;  // Last 4 bits of the second byte
        modified_test[i / 2] = (second_nibble << 4) | first_nibble;
    }


    let mut assembly = [0u8; 32/2];
    safe_encode_bit_sliced_array_mayo12(&mut modified_test, &mut assembly, 32);


    println!();
    println!("assembly:");
    for &byte in assembly.iter() {
        print!("{:08b}", byte);
    }


    println!();
    println!("plain: {:?}", plain);
    println!("assembly: {:?}", assembly);
}