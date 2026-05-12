use std::mem::{align_of, size_of};
use std::process::abort;

fn main() {
    let s1 = "_Alignas";
    let s2 = "_Alignof";
    let s3 = "1";
    let s4 = "1";

    // Check align_of equivalents
    const MAX_ALIGN: usize = align_of::<std::ffi::c_void>();

    // Check all the alignments
    assert_eq!(align_of::<char>(), align_of::<std::ffi::c_void>());
    assert_eq!(align_of::<i16>(), align_of::<std::ffi::c_void>());
    assert_eq!(align_of::<i32>(), align_of::<std::ffi::c_void>());
    assert_eq!(align_of::<i64>(), align_of::<std::ffi::c_void>());
    assert_eq!(align_of::<f32>(), align_of::<std::ffi::c_void>());
    assert_eq!(align_of::<f64>(), align_of::<std::ffi::c_void>());
    
    // Check alignas with zero and char
    let ca: [u8; 10] = [0; 10];
    assert_eq!(align_of::<[u8; 10]>(), align_of::<i32>());
    
    // Check alignas with alignof + 0
    let x: i32 = 0;
    assert_eq!(align_of_val(&x), align_of::<std::ffi::c_void>());

    // Check enum based alignment
    let y: i32 = 0;
    assert_eq!(align_of_val(&y), align_of::<std::ffi::c_void>());

    // Validate string definitions
    if s1 != "_Alignas" {
        abort();
    }
    if s2 != "_Alignof" {
        abort();
    }
    if s3 != "1" {
        abort();
    }
    if s4 != "1" {
        abort();
    }
}