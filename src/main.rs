use lib::{encode_bit_sliced_matrices, write_and_compare_kat_file::write_and_compare_kat_file};

fn main() {
    write_and_compare_kat_file();
}


fn main() {
    

    // make example. Add other values for clarity
    let mut test = [2u8; 64];
    test[0] = 1;
    test[20] = 12;


    let plain = encode_bit_sliced_array!(test, 64);

    let assembly = [0u8; 64/2];
    safe_encode_bit_sliced_array!(test, assembly, 64);


    println!("plain: {:?}", plain);
    println!("assembly: {:?}", assembly);
}