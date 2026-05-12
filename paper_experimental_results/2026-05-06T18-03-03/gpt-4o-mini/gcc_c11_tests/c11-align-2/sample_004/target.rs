use std::ffi::CStr;
use std::os::raw::c_char;

const ALIGNAS: &str = std::any::type_name::<Alignas>();
const ALIGNOF: &str = std::any::type_name::<Alignof>();
const ALIGNAS_IS_DEFINED: &str = "1";
const ALIGNOF_IS_DEFINED: &str = "1";

#[repr(align(8))]
struct Alignas;

#[repr(align(8))]
struct Alignof;

fn strcmp(s1: &CStr, s2: &CStr) -> i32 {
    s1.to_bytes().cmp(s2.to_bytes()) as i32
}

fn main() {
    let s1 = CStr::from_bytes_with_nul(b"_Alignas\0").unwrap();
    let s2 = CStr::from_bytes_with_nul(b"_Alignof\0").unwrap();
    let s3 = CStr::from_bytes_with_nul(b"1\0").unwrap();
    let s4 = CStr::from_bytes_with_nul(b"1\0").unwrap();

    if strcmp(s1, CStr::from_bytes_with_nul(ALIGNAS.as_bytes()).unwrap()) != 0 {
        std::process::abort();
    }
    if strcmp(s2, CStr::from_bytes_with_nul(ALIGNOF.as_bytes()).unwrap()) != 0 {
        std::process::abort();
    }
    if strcmp(s3, CStr::from_bytes_with_nul(ALIGNAS_IS_DEFINED.as_bytes()).unwrap()) != 0 {
        std::process::abort();
    }
    if strcmp(s4, CStr::from_bytes_with_nul(ALIGNOF_IS_DEFINED.as_bytes()).unwrap()) != 0 {
        std::process::abort();
    }
}