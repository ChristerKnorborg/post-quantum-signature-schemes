/* extern "C" {
    fn asm_function() -> u64;
}

pub fn safe_asm(x: &mut [u8], y: &[u8], z: &[u8]) -> u64 {
    unsafe { asm_function() }
} */



extern "C" {
    fn asm_function(res_last_row: *mut u8, p1_last_row: *const u8, final_o_vec: *const u8);
}


pub fn safe_asm(res_last_row: &mut [u8], p1_last_row: &[u8], final_o_vec: &[u8]) {
    unsafe { asm_function(res_last_row.as_mut_ptr(), p1_last_row.as_ptr(), final_o_vec.as_ptr()); }
}