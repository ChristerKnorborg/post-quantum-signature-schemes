use std::os::raw as ccty;

extern "C" {
   pub fn vmull_values(
        result: *mut ccty::c_uchar, // Pointer to const u8 for C compatibility
        a: *const ccty::c_uchar, 
        b: *const ccty::c_uchar,
        n: ccty::c_int
    );

    pub fn veor_values(
        result: *const ccty::c_uchar, // Pointer to const u8 for C compatibility
        a: *const ccty::c_uchar, 
        b: *const ccty::c_uchar,
        n: ccty::c_int
    );

    pub fn vmull_values_flat(
        result: *mut ccty::c_uchar, // Pointer to const u8 for C compatibility
        a: *const ccty::c_uchar, 
        b: *const ccty::c_uchar,
        rows_a: ccty::c_int,
        cols_a: ccty::c_int,
        cols_b: ccty::c_int
    );


    pub fn o_transposed_mul_p2(
        result: *mut ccty::c_uchar, // Pointer to const u8 for C compatibility
        o: *const ccty::c_uchar, 
        p2: *const ccty::c_uchar,
        rows_o: ccty::c_int,
        cols_o: ccty::c_int,
        rows_p2: ccty::c_int,
        cols_p2: ccty::c_int,
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




//  extern "C" {
//     pub fn vmull_values(
//         result: *const ccty::c_uchar, // Pointer to const u8 for C compatibility
//         a: *const ccty::c_uchar, 
//         b: ccty::c_uchar,
//     );
// } 