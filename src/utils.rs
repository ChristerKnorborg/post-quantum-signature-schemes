pub fn test_random(k: u8, o: u8) -> Vec<u8> {
    let num_elems: u16 = (k * o) as u16;

    let test_vec = vec![1; num_elems as usize];
    return test_vec;
}

pub fn print_matrix(mat: Vec<Vec<u8>>) -> () {
    mat.iter().for_each(|f| {
        println!("{:?}", f);
})
}