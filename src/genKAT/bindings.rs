use std::os::raw as ccty;

extern "C" {
    pub fn randombytes_init_nist(
        entropy_input: *const ccty::c_uchar, // Pointer to const u8 for C compatibility
        personalization_string: *const ccty::c_uchar, 
        security_strength: ccty::c_int,
    );

    pub fn randombytes_init(
        entropy_input: *mut ccty::c_uchar, // Use pointer to mutable u8 for slices
        personalization_string: *const ccty::c_uchar, 
        security_strength: ccty::c_int,
    );

    pub fn randombytes(random_arrays: *mut ccty::c_uchar, nbytes: ccty::c_ulonglong);

    pub fn randombytes_nist(x: &mut ccty::c_uchar, xlen: ccty::c_ulong);

    pub fn AES_128_CTR(
        output: *mut ccty::c_uchar,
        outputByteLen: ccty::c_ulonglong,
        input: *const ccty::c_uchar,
        inputByteLen: ccty::c_ulonglong,
    );

// }
    pub fn shake256(
        output: *mut ccty::c_uchar,
        outputByteLen: ccty::c_ulonglong,
        input: *const ccty::c_uchar,
        inputByteLen: ccty::c_ulonglong,
    );
}
