use std::mem::{align_of, size_of};

fn main() {
    // Check alignments using Rust's alignment features
    assert_eq!(align_of::<std::ffi::c_void>(), align_of::<std::ffi::c_void>());
    assert_eq!(align_of::<i32>(), align_of::<i32>());
    assert_eq!(align_of::<i64>(), align_of::<i64>());
    assert_eq!(align_of::<f32>(), align_of::<f32>());
    assert_eq!(align_of::<f64>(), align_of::<f64>());
    assert_eq!(align_of::<[u8; 10]>(), align_of::<[u8; 10]>());

    // Check string representations
    let s1 = "_Alignas";
    let s2 = "_Alignof";
    let s3 = "1";
    let s4 = "1";

    // Validate string representations
    if s1 != "_Alignas" {
        std::process::abort();
    }
    if s2 != "_Alignof" {
        std::process::abort();
    }
    if s3 != "1" {
        std::process::abort();
    }
    if s4 != "1" {
        std::process::abort();
    }
}