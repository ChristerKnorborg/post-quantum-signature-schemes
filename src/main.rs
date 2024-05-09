use lib::{crypto_primitives::safe_mul_add_bitsliced_m_vec_mayo3, write_and_compare_kat_file::write_and_compare_kat_file};

fn main() {
   // write_and_compare_kat_file();

    let mut inp: [u32; 12] = [0u32 ; 12];
    inp[0] = 0x00000001; // 1  0 (63)
    inp[1] = 0x00000002; // 0 (64)
    inp[2] = 0x00000003; // 1  0 (63)
    inp[3] = 0x00000004; // 0 (64)
    inp[4] = 0x00000005; // 1  0 (63)
    inp[5] = 0x00000006; // 0 (64)
    inp[6] = 0x00000007; // 1  0 (63)
    inp[7] = 0x00000008; // 0 (64)
    inp[8] = 0x00000009; // 1  0 (63)
    inp[9] = 0x0000000a; // 0 (64)
    inp[10] = 0x0000000b; // 1  0 (63)
    inp[11] = 0x0000000c; // 0 (64)

     // Create accumulator
     let mut acc: [u32; 12] = [0u32 ; 12];

     // Nibble
     let nibble: u8 = 6u8;
 
     safe_mul_add_bitsliced_m_vec_mayo3(&inp, 0, nibble, &mut acc, 0);
 
     println!("Accumulator: {:?}", acc);
 }

