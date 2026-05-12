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
    let _ca: [u8; 10] = [0; 10];
    let _x: [i32; std::mem::align_of::<std::mem::MaxAlign>()] = [0; std::mem::align_of::<std::mem::MaxAlign>()];
    let _y: [i32; std::mem::align_of::<std::mem::MaxAlign>()] = [0; std::mem::align_of::<std::mem::MaxAlign>()];

    // Test alignas and alignof in function
    let _auto_ll: [i64; std::mem::align_of::<std::mem::MaxAlign>()] = [0; std::mem::align_of::<std::mem::MaxAlign>()];

    // Test alignas and alignof in enum
    enum E { E = std::mem::align_of::<std::mem::MaxAlign>() }
    let _e: [i32; E::E as usize] = [0; E::E as usize];

    // Test alignas and alignof in struct
    struct S;
    let _s: [u8; std::mem::align_of::<S>()] = [0; std::mem::align_of::<S>()];

    // Test alignas and alignof in const
    const S1: &str = "alignas";
    const S2: &str = "alignof";
    const S3: &str = "1";
    const S4: &str = "1";

    // Test strcmp
    if S1!= "alignas" {
        std::process::exit(1);
    }
    if S2!= "alignof" {
        std::process::exit(1);
    }
    if S3!= "1" {
        std::process::exit(1);
    }
    if S4!= "1" {
        std::process::exit(1);
    }
}