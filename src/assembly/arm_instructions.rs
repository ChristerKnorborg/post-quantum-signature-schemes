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

    pub fn p1_o_matrix_mult(
        result: *mut ccty::c_uchar, // Pointer to const u8 for C compatibility
        a: *const ccty::c_uchar,
        b: *const ccty::c_uchar,
        rows_a: ccty::c_int,
        cols_a: ccty::c_int,
        cols_b: ccty::c_int,
    );

    pub fn calculate_p3(
        result: *mut ccty::c_uchar, // Pointer to const u8 for C compatibility
        o: *const ccty::c_uchar, 
        p1: *const ccty::c_uchar,
        p2: *const ccty::c_uchar,
        param_v: ccty::c_int,
        param_o: ccty::c_int,
        param_m: ccty::c_int
    );


}

