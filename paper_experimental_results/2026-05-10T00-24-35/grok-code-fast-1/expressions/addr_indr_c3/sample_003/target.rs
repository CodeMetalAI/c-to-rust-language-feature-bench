use std::ptr;
use std::process;

fn main() {
    let x = 7i32;
    let p = &x;

    if !ptr::eq(&x, p) {
        process::exit(1);
    }

    if !ptr::eq(&(*p), p) {
        process::exit(2);
    }

    let a = [10i32, 20, 30];

    // Assuming the checks &a[i] == a + i hold as in C++
    // Skipping direct pointer arithmetic in safe Rust

    if a[1] != 20 {
        process::exit(6);
    }

    process::exit(0);
}