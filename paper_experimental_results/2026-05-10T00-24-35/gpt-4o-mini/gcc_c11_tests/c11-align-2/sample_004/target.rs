use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> i32;
}

const S1: &str = std::any::type_name::<AlignAs>();
const S2: &str = std::any::type_name::<AlignOf>();
const S3: &str = "1";
const S4: &str = "1";

struct AlignAs;
struct AlignOf;

#[repr(align(8))]
struct MaxAlign;

#[repr(align(4))]
struct AlignedInt(i32);

#[repr(align(16))]
struct AlignedLong(i64);

#[repr(align(16))]
struct AlignedFloat(f32);

#[repr(align(16))]
struct AlignedDouble(f64);

#[repr(align(16))]
struct AlignedComplexLongDouble;

fn main() {
    unsafe {
        if strcmp(CString::new(S1).unwrap().as_ptr(), CString::new("_Alignas").unwrap().as_ptr()) != 0 {
            std::process::abort();
        }
        if strcmp(CString::new(S2).unwrap().as_ptr(), CString::new("_Alignof").unwrap().as_ptr()) != 0 {
            std::process::abort();
        }
        if strcmp(CString::new(S3).unwrap().as_ptr(), CString::new("1").unwrap().as_ptr()) != 0 {
            std::process::abort();
        }
        if strcmp(CString::new(S4).unwrap().as_ptr(), CString::new("1").unwrap().as_ptr()) != 0 {
            std::process::abort();
        }
    }
}