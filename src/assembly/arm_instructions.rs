use std::os::raw as ccty;

extern "C" {

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

