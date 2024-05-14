use std::os::raw as ccty;

extern "C" {

    pub fn matrix_add(
        result: *mut ccty::c_uchar, // Pointer to const u8 for C compatibility
        a: *const ccty::c_uchar,
        b: *const ccty::c_uchar,
        n: ccty::c_int
    );

    pub fn inner_product(
        result: *mut ccty::c_uchar, // Pointer to const u8 for C compatibility
        a: *const ccty::c_uchar,
        b: *const ccty::c_uchar,
        n: ccty::c_int
    );

}

