use std::os::raw as ccty;

extern "C" {
   pub fn vmull_values(
        result: *mut ccty::c_uchar, // Pointer to const u8 for C compatibility
        a: *const ccty::c_uchar, 
        b: *const ccty::c_uchar,
        n: ccty::c_int
    );
}




//  extern "C" {
//     pub fn vmull_values(
//         result: *const ccty::c_uchar, // Pointer to const u8 for C compatibility
//         a: *const ccty::c_uchar, 
//         b: ccty::c_uchar,
//     );
// } 