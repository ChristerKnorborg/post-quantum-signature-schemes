use std::os::raw as ccty;

extern "C" {
    pub fn mul_add_bitsliced_m_vec_mayo12(
        input:  *const ccty::c_uint,    // Pointer to const u32 for C compatibility
        input_start: ccty::c_int,  // Using c_ulong for size compatibility
        nibble: ccty::c_uchar,       // u8 in C is generally an unsigned char
        acc: *mut ccty::c_uint,         // Pointer to mutable u32 for C compatibility
        acc_start: ccty::c_int  // Using c_ulong for size compatibility
    );

    pub fn mul_add_bitsliced_m_vec_mayo3(
        input:  *const ccty::c_uint,    // Pointer to const u32 for C compatibility
        input_start: ccty::c_int,  // Using c_ulong for size compatibility
        nibble: ccty::c_uchar,
        acc: *mut ccty::c_uint,         // Pointer to mutable u32 for C compatibility
        acc_start: ccty::c_int  // Using c_ulong for size compatibility
    );

    pub fn mul_add_bitsliced_m_vec_mayo5(
        input:  *const ccty::c_uint,    // Pointer to const u32 for C compatibility
        input_start: ccty::c_int,  // Using c_ulong for size compatibility
        nibble: ccty::c_uchar,
        acc: *mut ccty::c_uint,         // Pointer to mutable u32 for C compatibility
        acc_start: ccty::c_int  // Using c_ulong for size compatibility
    );
}

