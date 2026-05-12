use std::mem;

fn main() {
    const S1: &str = "_Alignas";
    const S2: &str = "_Alignof";
    const S3: &str = "1";
    const S4: &str = "1";

    let s1 = "_Alignas";
    let s2 = "_Alignof";
    let s3 = "1";
    let s4 = "1";

    if s1 != S1 {
        std::process::abort();
    }
    if s2 != S2 {
        std::process::abort();
    }
    if s3 != S3 {
        std::process::abort();
    }
    if s4 != S4 {
        std::process::abort();
    }
    
    // Alignments - in Rust, alignment is checked using mem::align_of
    #[repr(align(16))]
    struct AlignC;

    assert_eq!(mem::align_of::<AlignC>(), mem::align_of::<mem::MaybeUninit<()>>());

    #[repr(align(16))]
    struct AlignS;
    let _s: AlignS;

    #[repr(align(4))]
    struct AlignI;
    let _i: AlignI;

    #[repr(align(16))]
    struct AlignL;
    let _l: AlignL;

    #[repr(align(16))]
    struct AlignLL;
    let _ll: AlignLL;

    #[repr(align(16))]
    struct AlignF;
    let _f: AlignF;

    #[repr(align(16))]
    struct AlignD;
    let _d: AlignD;

    #[repr(align(16))]
    struct AlignCLD;
    let _cld: AlignCLD;

    #[repr(align(16))]
    struct AlignCA([u8; 10]);
    let _ca: AlignCA;

    #[repr(align(16))]
    struct AlignX;
    let _x: AlignX;

    #[repr(align(16))]
    struct AlignY;
    let _y: AlignY;

    fn func() {
        #[repr(align(16))]
        struct AlignAutoLL;
        let _auto_ll: AlignAutoLL;
    }

    // A struct with 0 alignment in Rust is not directly representable,
    // but align(1) is the minimum that can be used.
    #[repr(align(1))]
    struct Useless;

    func();
}