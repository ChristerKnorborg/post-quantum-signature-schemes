mod sample;
mod finite_field;
mod utils;
mod bitsliced_functionality;


use crate::bitsliced_functionality as bf;

fn main() {


    let vec_1: Vec<Vec<u8>> = vec![
            vec![0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x4], 
            vec![0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xf], 
            vec![0x8, 0x9, 0xf, 0xe, 0x3, 0x4, 0x5, 0x6],
            vec![0x8, 0x9, 0x5, 0x6, 0xe, 0xa, 0xb, 0x1],  
        ];
        
        let vec_2 = vec_1.clone();
        let vec_3 = vec_1.clone();
        let vec_4 = vec_1.clone();
        let vec_5 = vec_1.clone();
        let vec_6 = vec_1.clone();
        let vec_7 = vec_1.clone();
        let vec_8 = vec_1.clone();

        let rows = vec_1.len();
        let cols = vec_1[0].len();

        let plain_input: Vec<Vec<Vec<u8>>> = vec![vec_1.clone(), vec_2, vec_3, vec_4, vec_5, vec_6, vec_7, vec_8];

        let bytestring = bf::encode_bit_sliced_matrices(rows, cols, plain_input.clone(), false);

        let result = bf::decode_bit_sliced_matrices(rows, cols, bytestring, false);

}