fn main() {
    #[repr(align(16))] // assume max_align_t is 16 bytes
    struct C;
    let c = C;

    #[repr(align(16))]
    struct S;
    let s = S;

    let i: i32 = 0;

    let l: i64 = 0;

    let ll: i128 = 0;

    let f: f32 = 0.0;

    let d: f64 = 0.0;

    let cld: (f64, f64) = (0.0, 0.0);

    let ca: [i8; 10] = [0; 10];

    let x: i32 = 0;

    enum E { E = 16 }
    let y: i32 = 0;

    {
        let auto_ll: i128 = 0;
    }

    #[repr(align(0))]
    struct S; // this is indeed useless in Rust

    if std::mem::align_of::<C>()!= 16 {
        panic!();
    }
    if std::mem::align_of::<S>()!= 16 {
        panic!();
    }

    const S1: &'static str = "_Alignas";
    const S2: &'static str = "_Alignof";
    const S3: &'static str = "1";
    const S4: &'static str = "1";

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