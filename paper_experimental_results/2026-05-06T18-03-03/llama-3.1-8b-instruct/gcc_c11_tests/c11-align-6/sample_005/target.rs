// Test Rust's alignment attributes returning minimum alignment for a type.
// { dg-do run }
// { dg-options "-C" }

use std::mem;

fn main() {
    // Define a macro to check the alignment of a type.
    macro_rules! check_align {
        ($ty:ty) => {{
            let x = [0; mem::size_of::<$ty>()];
            if mem::align_of::<$ty>() > mem::align_of::<[u8; mem::size_of::<$ty>()]>() {
                panic!("Alignment mismatch for type {}", stringify!($ty));
            }
        }};
    }

    // Check the alignment of various types.
    check_align!(bool);
    check_align!(char);
    check_align!(i8);
    check_align!(u8);
    check_align!(i16);
    check_align!(u16);
    check_align!(i32);
    check_align!(u32);
    check_align!(i64);
    check_align!(u64);
    check_align!(f32);
    check_align!(f64);
    check_align!(f80);
    check_align!(complex_f32);
    check_align!(complex_f64);
    check_align!(complex_f80);
}

// Define complex types for testing.
#[repr(C)]
struct ComplexF32(f32);
#[repr(C)]
struct ComplexF64(f64);
#[repr(C)]
struct ComplexF80(f80);