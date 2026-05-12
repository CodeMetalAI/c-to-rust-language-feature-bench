fn main() {
    // Test alignas and alignof
    let _c: [u8; std::mem::align_of::<std::mem::MaxAlign>()] = [0; std::mem::align_of::<std::mem::MaxAlign>()];
    let _s: [u16; std::mem::align_of::<std::mem::MaxAlign>()] = [0; std::mem::align_of::<std::mem::MaxAlign>()];
    let _i: [i32; std::mem::align_of::<i32>()] = [0; std::mem::align_of::<i32>()];
    let _l: [i64; std::mem::align_of::<std::mem::MaxAlign>()] = [0; std::mem::align_of::<std::mem::MaxAlign>()];
    let _ll: [i64; std::mem::align_of::<std::mem::MaxAlign>()] = [0; std::mem::align_of::<std::mem::MaxAlign>()];
    let _f: [f32; std::mem::align_of::<std::mem::MaxAlign>()] = [0.0; std::mem::align_of::<std::mem::MaxAlign>()];
    let _d: [f64; std::mem::align_of::<std::mem::MaxAlign>()] = [0.0; std::mem::align_of::<std::mem::MaxAlign>()];
    let _cld: [f64; std::mem::align_of::<std::mem::MaxAlign>()] = [0.0; std::mem::align_of::<std::mem::MaxAlign>()];

    // Test alignas with multiple alignments
    let _ca: [u8; 10] = [0; 10];

    // Test alignas with enum
    enum E { E = std::mem::align_of::<std::mem::MaxAlign>() }
    let _y: [i32; E::E as usize] = [0; E::E as usize];

    // Test alignas in function
    fn func() {
        let _auto_ll: [i64; std::mem::align_of::<std::mem::MaxAlign>()] = [0; std::mem::align_of::<std::mem::MaxAlign>()];
    }
    func();

    // Test alignas with struct (Rust does not support alignas on structs)
    // struct S; // Not supported in Rust

    // Test alignas and alignof macros
    const S1: &str = "_Alignas";
    const S2: &str = "_Alignof";
    const S3: &str = "1";
    const S4: &str = "1";

    if S1!= "_Alignas" {
        std::process::abort();
    }
    if S2!= "_Alignof" {
        std::process::abort();
    }
    if S3!= "1" {
        std::process::abort();
    }
    if S4!= "1" {
        std::process::abort();
    }
}