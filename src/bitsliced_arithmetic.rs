use crate::{constants::{L_BYTES, M, O, O_BYTES, P1_BYTES, V}, utils::{write_u32_array_to_file_byte, write_u32_array_to_file_int, write_u8_array_to_file_byte, write_u8_array_to_file_int}};








pub fn p1_p1_transposed_added_times_o(p1: &[u8], o: [[u8 ; O] ; V]) -> [u8 ; L_BYTES] {
    

    let mut p1_u32 = [0u32 ; P1_BYTES/4];
    for (i, chunk) in p1.chunks(4).enumerate() {
        let mut array = [0u8; 4];
        array.copy_from_slice(chunk);
        p1_u32[i] = u32::from_le_bytes(array); // Use from_be_bytes for big endian
    }


    let _ = write_u32_array_to_file_byte("After u32 byte", &p1_u32);

    // Size is slightly less than P1_BYTES/2 as there must be space for everything below the diagonals of the m p1 matrices.
    // Divide by 8 since we transform to u32 (e.g. divede by 4).
    // Also, we can represent 2 nibbles in a single byte with encoding (e.g. divide by 2).
    const ADDED_P1_SIZE: usize = V*V*M/8;

    let mut p1_p1_transposed_added = [0u32 ; ADDED_P1_SIZE];

    let m_legs = M / 32;
    let mut used = 0;


    // Add P1 and P1 transposed
    for r in 0..V {
        for c in r..V {

            // Set diagonal to 0 (entries where i=j)to 0 for all m matrices (at memory location)

            // Set remaining entries [i, j] to be [j, i]. As, all entries are 0 in the lower half of p1,
            // and all entries are 0 in the upper half of p1_transposed.
            if r != c {
                let start = m_legs * 4 * (r * V + c);
                for i in 0..(m_legs*4) {
                    p1_p1_transposed_added[start+i] = p1_u32[m_legs * 4 * used + i];
                }

                let start = m_legs * 4 * (c * V + r);
                for i in 0..(m_legs*4) {
                    p1_p1_transposed_added[start+i] = p1_u32[m_legs * 4 * used + i];
                }
            }
            used += 1;
        }
    }

    println!("added p1 size: {}", ADDED_P1_SIZE);
    let _ = write_u32_array_to_file_byte("p1_p1_transposed_added u32 byte", &p1_p1_transposed_added);


    
    

    return [0 ; L_BYTES];

}