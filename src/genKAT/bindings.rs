use std::os::raw as ccty;

extern "C" {
    pub fn randombytes_init_nist (entropy_input: ccty::c_uchar,
        personalization_string: ccty::c_uchar,
        security_strength: ccty::c_int); 
}

extern "C" {
    pub fn randombytes_init (entropy_input: &mut [ccty::c_uchar],
        personalization_string: ccty::c_uchar,
        security_strength: ccty::c_int); 
}

extern "C" {
    pub fn randombytes (random_arrays: ccty::c_uchar,
        nbytes: ccty::c_ulonglong); 
}

extern "C" {
    pub fn randombytes_nist (x: &mut ccty::c_uchar,
        xlen: ccty::c_ulong); 
}

