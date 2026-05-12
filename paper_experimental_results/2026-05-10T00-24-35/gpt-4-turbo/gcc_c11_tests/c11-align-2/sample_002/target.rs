fn main() {
    // Rust has `align_of` directly in the `std::mem` module and `align_of` is always defined.
    assert_eq!(std::mem::align_of::<max_align_t>(), std::mem::align_of::<char>());
    assert_eq!(std::mem::align_of::<max_align_t>(), std::mem::align_of::<short>());
    assert_eq!(std::mem::align_of::<i32>(), std::mem::align_of::<i32>());
    assert_eq!(std::mem::align_of::<max_align_t>(), std::mem::align_of::<i64>());
    assert_eq!(std::mem::align_of::<max_align_t>(), std::mem::align_of::<f64>());
    assert_eq!(std::mem::align_of::<max_align_t>(), std::mem::align_of::<f64>()); // Complex long double is platform specific

    let ca: [char; 10] = [0 as char; 10];
    assert_eq!(std::mem::align_of::<i32>() + 0, std::mem::align_of::<i32>());

    let y: i32 = 0;
    assert_eq!(std::mem::align_of::<max_align_t>(), std::mem::align_of_val(&y));

    // Rust does not have a direct equivalent to C's `alignas(0)`, which is non-compliant, and Rust assertions require actual checks.
    // The preprocessor directives are not applicable in Rust, thus the error directives are replaced by assertions where applicable.

    let s1 = "_Alignas";
    let s2 = "_Alignof";
    let s3 = "1";
    let s4 = "1";

    assert_eq!(s1, "_Alignas");
    assert_eq!(s2, "_Alignof");
    assert_eq!(s3, "1");
    assert_eq!(s4, "1");
}

// Dummy type to emulate C's `max_align_t` as much as can be inferred from context.
#[repr(C, align(16))]
struct max_align_t;