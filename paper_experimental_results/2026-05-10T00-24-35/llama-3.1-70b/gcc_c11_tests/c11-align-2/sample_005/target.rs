fn main() {
    #[repr(align(16))]
    struct AlignedStruct;

    let _c: AlignedStruct = AlignedStruct;
    let _s: i16 = 0;
    let _i: i32 = 0;
    let _l: i64 = 0;
    let _ll: i128 = 0;
    let _f: f32 = 0.0;
    let _d: f64 = 0.0;
    let _cld: std::complex::Complex<f64> = std::complex::Complex { re: 0.0, im: 0.0 };

    let _ca: [u8; 10] = [0; 10];
    let _x: i32 = 0;

    enum E {
        E = 16,
    }

    #[repr(align(16))]
    struct S;

    let _y: i32 = 0;

    {
        let _auto_ll: i128 = 0;
    }

    let s1 = stringify!(std::mem::align_of);
    let s2 = stringify!(std::mem::align_of);
    let s3 = stringify!(std::mem::align_of);
    let s4 = stringify!(std::mem::align_of);

    assert_eq!(s1, "align_of");
    assert_eq!(s2, "align_of");
    assert_eq!(s3, "align_of");
    assert_eq!(s4, "align_of");
}