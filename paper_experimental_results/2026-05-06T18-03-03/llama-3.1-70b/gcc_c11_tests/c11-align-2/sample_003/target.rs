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
    let _x: [i32; std::mem::align_of::<i32>() + 0;
    enum E { E = std::mem::align_of::<std::mem::MaxAlign>() }
    let _y: [i32; E::E as usize] = [0; E::E as usize];

    // Test alignas and alignof in function
    fn func() {
        let _auto_ll: [i64; std::mem::align_of::<std::mem::MaxAlign>()] = [0; std::mem::align_of::<std::mem::MaxAlign>()];
    }
    func();

    // Test alignas and alignof with 0 alignment
    struct S;
    std::mem::align_of::<S>();

    // Test alignas and alignof with const generics
    const ALIGN: usize = std::mem::align_of::<std::mem::MaxAlign>();
    let _z: [u8; ALIGN] = [0; ALIGN];

    // Test __alignas_is_defined and __alignof_is_defined
    if !std::mem::align_of::<std::mem::MaxAlign>().is_power_of_two() {
        std::process::abort();
    }

    // Test stringification of alignas and alignof
    let s1 = stringify!(alignas);
    let s2 = stringify!(alignof);
    if s1 != "alignas" || s2 != "alignof" {
        std::process::abort();
    }
}