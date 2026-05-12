use std::mem;

fn main() {
    // Check alignments using Rust's equivalent features
    let c: u8 = 0;
    let s: i16 = 0;
    let i: i32 = 0;
    let l: i64 = 0;
    let ll: i64 = 0;
    let f: f32 = 0.0;
    let d: f64 = 0.0;
    let cld: (f64, f64) = (0.0, 0.0); // Complex numbers are not native in Rust, using tuple

    let ca: [u8; 10] = [0; 10];
    let x: i32 = 0;
    let y: i32 = 0;

    // Check alignments
    assert_eq!(mem::align_of_val(&c), mem::align_of::<mem::Align>());
    assert_eq!(mem::align_of_val(&s), mem::align_of::<mem::Align>());
    assert_eq!(mem::align_of_val(&i), mem::align_of::<i32>());
    assert_eq!(mem::align_of_val(&l), mem::align_of::<mem::Align>());
    assert_eq!(mem::align_of_val(&ll), mem::align_of::<mem::Align>());
    assert_eq!(mem::align_of_val(&f), mem::align_of::<mem::Align>());
    assert_eq!(mem::align_of_val(&d), mem::align_of::<mem::Align>());
    assert_eq!(mem::align_of_val(&cld), mem::align_of::<mem::Align>());
    assert_eq!(mem::align_of_val(&ca), mem::align_of::<u8>());
    assert_eq!(mem::align_of_val(&x), mem::align_of::<mem::Align>());
    assert_eq!(mem::align_of_val(&y), mem::align_of::<mem::Align>());

    // String comparisons
    let s1 = "_Alignas";
    let s2 = "_Alignof";
    let s3 = "1";
    let s4 = "1";

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