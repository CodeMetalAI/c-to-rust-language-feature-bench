fn main() {
    // Rust has inbuilt alignment with types.
    // Here are equivalent Rust declarations using std::mem::align_of.

    use std::mem::align_of;

    // alignas(alignof(max_align_t)) char c;
    let c: char = 0 as char;
    assert_eq!(align_of::<char>(), align_of::<c>());

    // alignas(max_align_t) short s;
    let s: i16 = 0;
    assert_eq!(align_of::<i16>(), align_of::<s>());

    // alignas(int) int i;
    let i: i32 = 0;
    assert_eq!(align_of::<i32>(), align_of::<i>());

    // alignas(max_align_t) long l;
    let l: i64 = 0;
    assert_eq!(align_of::<i64>(), align_of::<l>());

    // alignas(max_align_t) long long ll;
    let ll: i64 = 0;
    assert_eq!(align_of::<i64>(), align_of::<ll>());

    // alignas(max_align_t) float f;
    let f: f32 = 0.0;
    assert_eq!(align_of::<f32>(), align_of::<f>());

    // alignas(max_align_t) double d;
    let d: f64 = 0.0;
    assert_eq!(align_of::<f64>(), align_of::<d>());

    // alignas(0) alignas(int) alignas(char) char ca[10];
    let ca: [char; 10] = [0 as char; 10];
    assert_eq!(align_of::<char>(), align_of_val(&ca[0]));

    // alignas((int)alignof(max_align_t) + 0) int x;
    let x: i32 = 0;
    assert_eq!(align_of::<i32>(), align_of::<x>());

    // enum e { E = alignof(max_align_t) };
    // alignas(E) int y;
    enum E {
        E = align_of::<i64>() as isize,
    }
    let y: i32 = 0;
    assert_eq!(E::E as usize, align_of::<y>());

    // Check values with strings
    let s1 = stringify!(align_of);
    let s2 = stringify!(align_of);
    let s3 = "1";
    let s4 = "1";

    if s1 != "align_of" {
        panic!();
    }
    if s2 != "align_of" {
        panic!();
    }
    if s3 != "1" {
        panic!();
    }
    if s4 != "1" {
        panic!();
    }
}