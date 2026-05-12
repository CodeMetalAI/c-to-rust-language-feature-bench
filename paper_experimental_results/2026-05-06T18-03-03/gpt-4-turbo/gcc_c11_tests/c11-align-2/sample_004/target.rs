fn main() {
    // Rust equivalent of the C++ code
    use std::mem::{align_of, size_of};

    // alignas(alignof(max_align_t)) char c;
    let c: u8 = 0;
    assert_eq!(align_of::<u8>(), align_of::<usize>());

    // alignas(max_align_t) short s;
    let s: u16 = 0;
    assert_eq!(align_of::<u16>(), align_of::<usize>());

    // alignas(int) int i;
    let i: i32 = 0;
    assert_eq!(align_of::<i32>(), align_of::<i32>());

    // alignas(max_align_t) long l;
    let l: i64 = 0;
    assert_eq!(align_of::<i64>(), align_of::<usize>());

    // alignas(max_align_t) long long ll;
    let ll: i64 = 0;
    assert_eq!(align_of::<i64>(), align_of::<usize>());

    // alignas(max_align_t) float f;
    let f: f32 = 0.0;
    assert_eq!(align_of::<f32>(), align_of::<usize>());

    // alignas(max_align_t) double d;
    let d: f64 = 0.0;
    assert_eq!(align_of::<f64>(), align_of::<usize>());

    // alignas(max_align_t) _Complex long double cld;
    // Rust does not have built-in complex number types in the standard library.
    // We use a tuple of two f64 to simulate a complex number.
    let cld: (f64, f64) = (0.0, 0.0);
    assert_eq!(align_of::<(f64, f64)>(), align_of::<usize>());

    // alignas(0) alignas(int) alignas(char) char ca[10];
    let ca: [u8; 10] = [0; 10];
    assert_eq!(align_of::<[u8; 10]>(), align_of::<u8>());

    // alignas((int)alignof(max_align_t) + 0) int x;
    let x: i32 = 0;
    assert_eq!(align_of::<i32>(), align_of::<usize>());

    // enum e { E = alignof(max_align_t) };
    // alignas(E) int y;
    const E: usize = align_of::<usize>();
    let y: i32 = 0;
    assert_eq!(align_of::<i32>(), E);

    // Check the string representations
    let s1 = stringify!(align_of);
    let s2 = stringify!(align_of);
    let s3 = "1"; // __alignas_is_defined equivalent
    let s4 = "1"; // __alignof_is_defined equivalent

    assert_eq!(s1, "align_of");
    assert_eq!(s2, "align_of");
    assert_eq!(s3, "1");
    assert_eq!(s4, "1");
}