#![allow(non_camel_case_types)]
use std::mem;

fn check_align<T>() {
    struct Test<T> {
        c: u8,
        v: T,
    }

    if mem::align_of::<T>() > mem::align_of::<Test<T>>() {
        panic!("Alignment check failed");
    }
}

fn main() {
    check_align::<bool>();
    check_align::<i8>();
    check_align::<u8>();
    check_align::<i16>();
    check_align::<u16>();
    check_align::<i32>();
    check_align::<u32>();
    check_align::<i64>();
    check_align::<u64>();
    check_align::<i128>();
    check_align::<u128>();
    check_align::<f32>();
    check_align::<f64>();
    // Note: Rust does not have a direct equivalent to C's `long double` and complex types.
    // Rust's `f64` is typically used for `long double` equivalence, but without complex numbers.
    // You may need to use external crates like `num-complex` for complex numbers in Rust.
    println!("All alignment checks passed successfully.");
}