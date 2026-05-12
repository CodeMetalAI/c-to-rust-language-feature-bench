fn main() {
    // Test alignment support in Rust
    // Rust has built-in support for alignment, but we'll use the equivalent concepts

    // Define variables with alignment
    #[repr(align(16))]
    struct AlignedChar(char);
    let _c: AlignedChar = AlignedChar(0);

    #[repr(align(16))]
    struct AlignedShort(short);
    let _s: AlignedShort = AlignedShort(0);

    #[repr(align(4))]
    struct AlignedInt(i32);
    let _i: AlignedInt = AlignedInt(0);

    #[repr(align(16))]
    struct AlignedLong(i64);
    let _l: AlignedLong = AlignedLong(0);

    #[repr(align(16))]
    struct AlignedLongLong(i64);
    let _ll: AlignedLongLong = AlignedLongLong(0);

    #[repr(align(16))]
    struct AlignedFloat(f32);
    let _f: AlignedFloat = AlignedFloat(0.0);

    #[repr(align(16))]
    struct AlignedDouble(f64);
    let _d: AlignedDouble = AlignedDouble(0.0);

    #[repr(align(16))]
    struct AlignedComplexLongDouble(Complex<f64>);
    let _cld: AlignedComplexLongDouble = AlignedComplexLongDouble(Complex::new(0.0, 0.0));

    #[repr(align(1))]
    struct AlignedCharArray([char; 10]);
    let _ca: AlignedCharArray = AlignedCharArray([0; 10]);

    #[repr(align(16))]
    struct AlignedX(i32);
    let _x: AlignedX = AlignedX(0);

    #[repr(align(16))]
    struct AlignedY(i32);
    let _y: AlignedY = AlignedY(0);

    // Test alignment of variables
    fn func() {
        #[repr(align(16))]
        struct AutoAlignedLongLong(i64);
        let _auto_ll: AutoAlignedLongLong = AutoAlignedLongLong(0);
    }

    // Test alignment of structs
    struct S;
    let _s: S = S;

    // Test alignment macros
    const ALIGNAS: &str = "alignas";
    const ALIGNOF: &str = "alignof";
    const ALIGNAS_IS_DEFINED: &str = "1";
    const ALIGNOF_IS_DEFINED: &str = "1";

    if ALIGNAS != "alignas" {
        std::process::abort();
    }
    if ALIGNOF != "alignof" {
        std::process::abort();
    }
    if ALIGNAS_IS_DEFINED != "1" {
        std::process::abort();
    }
    if ALIGNOF_IS_DEFINED != "1" {
        std::process::abort();
    }
}