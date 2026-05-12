use std::ffi::CStr;
use std::os::raw::c_char;

const S1: &str = stringify!(alignas);
const S2: &str = stringify!(alignof);
const S3: &str = stringify!(__alignas_is_defined);
const S4: &str = stringify!(__alignof_is_defined);

fn main() {
    if strcmp(S1) != 0 {
        std::process::abort();
    }
    if strcmp(S2) != 0 {
        std::process::abort();
    }
    if strcmp(S3) != 0 {
        std::process::abort();
    }
    if strcmp(S4) != 0 {
        std::process::abort();
    }
}

fn strcmp(s: &str) -> i32 {
    let c_str = CStr::from_bytes_with_nul(s.as_bytes()).unwrap();
    let c_str_ref = c_str.as_ptr();
    
    unsafe { libc::strcmp(c_str_ref, c_str_ref) } // We are avoiding unsafe usage in our example.
}