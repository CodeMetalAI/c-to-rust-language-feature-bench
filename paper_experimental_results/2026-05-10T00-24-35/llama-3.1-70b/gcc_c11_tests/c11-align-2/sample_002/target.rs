fn main() {
    const C: [u8; std::mem::align_of::<std::mem::MaxAlign>()] = [0; std::mem::align_of::<std::mem::MaxAlign>()];
    static S: [u16; std::mem::align_of::<std::mem::MaxAlign>()] = [0; std::mem::align_of::<std::mem::MaxAlign>()];
    const I: [i32; std::mem::align_of::<i32>()] = [0; std::mem::align_of::<i32>()];
    static L: [i64; std::mem::align_of::<std::mem::MaxAlign>()] = [0; std::mem::align_of::<std::mem::MaxAlign>()];
    static LL: [i64; std::mem::align_of::<std::mem::MaxAlign>()] = [0; std::mem::align_of::<std::mem::MaxAlign>()];
    static F: [f32; std::mem::align_of::<std::mem::MaxAlign>()] = [0.0; std::mem::align_of::<std::mem::MaxAlign>()];
    static D: [f64; std::mem::align_of::<std::mem::MaxAlign>()] = [0.0; std::mem::align_of::<std::mem::MaxAlign>()];
    static CLD: [f64; std::mem::align_of::<std::mem::MaxAlign>()] = [0.0; std::mem::align_of::<std::mem::MaxAlign>()];
    const CA: [u8; 10] = [0; 10];
    static X: [i32; std::mem::align_of::<std::mem::MaxAlign>()] = [0; std::mem::align_of::<std::mem::MaxAlign>()];
    enum E { E = std::mem::align_of::<std::mem::MaxAlign>() }
    static Y: [i32; E as usize] = [0; E as usize];

    {
        let _auto_ll: [i64; std::mem::align_of::<std::mem::MaxAlign>()] = [0; std::mem::align_of::<std::mem::MaxAlign>()];
    }

    struct S;
    static _S: [S; 1] = [S];

    if std::mem::align_of::<std::mem::MaxAlign>()!= std::mem::align_of::<std::mem::MaxAlign>() {
        panic!();
    }

    if std::mem::align_of::<std::mem::MaxAlign>()!= std::mem::align_of::<std::mem::MaxAlign>() {
        panic!();
    }

    const S1: &str = "alignas";
    const S2: &str = "alignof";
    const S3: &str = "1";
    const S4: &str = "1";

    if S1!= "_Alignas" {
        panic!();
    }
    if S2!= "_Alignof" {
        panic!();
    }
    if S3!= "1" {
        panic!();
    }
    if S4!= "1" {
        panic!();
    }
}