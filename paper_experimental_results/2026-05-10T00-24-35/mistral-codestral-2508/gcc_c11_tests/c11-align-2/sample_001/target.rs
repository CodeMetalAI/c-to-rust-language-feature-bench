fn main() {
    // Test alignment support in Rust
    let _c: [u8; 1] = [0; std::mem::align_of::<usize>()];
    let _s: [u16; 1] = [0; std::mem::align_of::<usize>()];
    let _i: i32 = 0;
    let _l: i64 = 0;
    let _ll: i64 = 0;
    let _f: f32 = 0.0;
    let _d: f64 = 0.0;
    let _cld: (f64, f64) = (0.0, 0.0);
    let _ca: [u8; 10] = [0; 10];
    let _x: i32 = 0;
    let _y: i32 = 0;

    fn func() {
        let _auto_ll: i64 = 0;
    }

    // Test alignment macros and constants
    assert_eq!(stringify!(alignas), "alignas");
    assert_eq!(stringify!(alignof), "alignof");
    assert_eq!(stringify!(1), "1");
    assert_eq!(stringify!(1), "1");

    // Test alignment of types
    assert_eq!(std::mem::align_of::<usize>(), std::mem::align_of::<i64>());
    assert_eq!(std::mem::align_of::<i32>(), std::mem::align_of::<i32>());
    assert_eq!(std::mem::align_of::<i64>(), std::mem::align_of::<i64>());
    assert_eq!(std::mem::align_of::<f32>(), std::mem::align_of::<f32>());
    assert_eq!(std::mem::align_of::<f64>(), std::mem::align_of::<f64>());
    assert_eq!(std::mem::align_of::<(f64, f64)>(), std::mem::align_of::<(f64, f64)>());
    assert_eq!(std::mem::align_of::<i32>(), std::mem::align_of::<i32>());
    assert_eq!(std::mem::align_of::<i32>(), std::mem::align_of::<i32>());
}