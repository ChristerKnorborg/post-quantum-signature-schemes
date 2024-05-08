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

    pub fn mayo_12_P1_times_O_mayo1(
        P1:  *const ccty::c_uint,    // Pointer to const u32 for C compatibility
        O: *const ccty::c_uchar,       // u8 in C is generally an unsigned char
        acc: *mut ccty::c_uint,         // Pointer to mutable u32 for C compatibility
    );

    pub fn mayo_P1_times_O_mayo1(
        P1:  *const ccty::c_uint,    // Pointer to const u32 for C compatibility
        O: *const ccty::c_uchar,       // u8 in C is generally an unsigned char
        acc: *mut ccty::c_uint,         // Pointer to mutable u32 for C compatibility
    );

    pub fn mayo_P1_times_Vt_mayo1(
        P1:  *const ccty::c_uint,    // Pointer to const u32 for C compatibility
        V: *const ccty::c_uchar,       // u8 in C is generally an unsigned char
        acc: *mut ccty::c_uint,         // Pointer to mutable u32 for C compatibility
    );

    pub fn mayo_P1_times_O_mayo2(
        P1:  *const ccty::c_uint,    // Pointer to const u32 for C compatibility
        O: *const ccty::c_uchar,       // u8 in C is generally an unsigned char
        acc: *mut ccty::c_uint,         // Pointer to mutable u32 for C compatibility
    );

    pub fn mul_add_64_bitsliced_m_vec(
        input:  *const ccty::c_uint,    // Pointer to const u32 for C compatibility
        input_start: ccty::c_int,  // Using c_ulong for size compatibility
        nibble: ccty::c_uchar,       // u8 in C is generally an unsigned char
        acc: *mut ccty::c_uint,         // Pointer to mutable u32 for C compatibility
        acc_start: ccty::c_int  // Using c_ulong for size compatibility
    );

    pub fn mul_add_96_bitsliced_m_vec(
        input:  *const ccty::c_uint,    // Pointer to const u32 for C compatibility
        input_start: ccty::c_int,  // Using c_ulong for size compatibility
        nibble: ccty::c_uchar,       // u8 in C is generally an unsigned char
        acc: *mut ccty::c_uint,         // Pointer to mutable u32 for C compatibility
        acc_start: ccty::c_int  // Using c_ulong for size compatibility
    );

    pub fn mul_add_128_bitsliced_m_vec(
        input:  *const ccty::c_uint,    // Pointer to const u32 for C compatibility
        input_start: ccty::c_int,  // Using c_ulong for size compatibility
        nibble: ccty::c_uchar,       // u8 in C is generally an unsigned char
        acc: *mut ccty::c_uint,         // Pointer to mutable u32 for C compatibility
        acc_start: ccty::c_int  // Using c_ulong for size compatibility
    );


    pub fn encode_bit_sliced_array_mayo12
    (
        input:  *mut ccty::c_uchar,    
        output: *mut ccty::c_uchar,  // Pointer to const u32 for C compatibility
        matrices: ccty::c_int,
    );
}

