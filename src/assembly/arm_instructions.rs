use std::os::raw as ccty;

extern "C" {
    pub fn mul_add_bitsliced_m_vec(
        input:  *const ccty::c_uint,    // Pointer to const u32 for C compatibility
        input_start: ccty::c_int,  // Using c_ulong for size compatibility
        nibble: ccty::c_uchar,       // u8 in C is generally an unsigned char
        acc: *mut ccty::c_uint,         // Pointer to mutable u32 for C compatibility
        acc_start: ccty::c_int  // Using c_ulong for size compatibility
    );

    pub fn mul_add_bitsliced_m_vec_mayo1(
        input:  *const ccty::c_uint,    // Pointer to const u32 for C compatibility
        input_start: ccty::c_int,  // Using c_ulong for size compatibility
        input_offset: ccty::c_int,  // Using c_ulong for size compatibility
        nibble1: ccty::c_uchar,
        nibble2: ccty::c_uchar,       // u8 in C is generally an unsigned char
        acc: *mut ccty::c_uint,         // Pointer to mutable u32 for C compatibility
        acc_start: ccty::c_int  // Using c_ulong for size compatibility
    );

    pub fn mul_add_bitsliced_m_vec_mayo1_new(
        input:  *const ccty::c_uint,    // Pointer to const u32 for C compatibility
        input_start_1: ccty::c_int,
        input_start_2: ccty::c_int,  // Using c_ulong for size compatibility
        nibble1: ccty::c_uchar,
        nibble2: ccty::c_uchar,       // u8 in C is generally an unsigned char
        acc: *mut ccty::c_uint,         // Pointer to mutable u32 for C compatibility
        acc_start_1: ccty::c_int,
        acc_start_2: ccty::c_int  // Using c_ulong for size compatibility
    );
}

