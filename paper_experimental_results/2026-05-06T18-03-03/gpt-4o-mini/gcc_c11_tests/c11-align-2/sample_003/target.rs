use std::ffi::CString;
use std::ptr;

const S1: &str = std::any::type_name::<alignas>();
const S2: &str = std::any::type_name::<alignof>();
const S3: &str = "1";
const S4: &str = "1";

fn strcmp(s1: &str, s2: &str) -> i32 {
    let c1 = CString::new(s1).unwrap();
    let c2 = CString::new(s2).unwrap();
    unsafe { libc::strcmp(c1.as_ptr(), c2.as_ptr()) }
}

fn main() {
    if strcmp(S1, "_Alignas") != 0 {
        std::process::abort();
    }
    if strcmp(S2, "_Alignof") != 0 {
        std::process::abort();
    }
    if strcmp(S3, "1") != 0 {
        std::process::abort();
    }
    if strcmp(S4, "1") != 0 {
        std::process::abort();
    }
}