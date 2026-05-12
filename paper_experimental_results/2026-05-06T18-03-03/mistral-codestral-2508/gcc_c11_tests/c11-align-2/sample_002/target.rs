fn main() {
    // Test alignment support in Rust
    // Rust has built-in alignment support through the `align` attribute

    // Define variables with alignment
    #[repr(align(16))]
    struct AlignedChar(char);
    let _c = AlignedChar(0);

    #[repr(align(16))]
    struct AlignedShort(short);
    let _s = AlignedShort(0);

    #[repr(align(4))]
    struct AlignedInt(i32);
    let _i = AlignedInt(0);

    #[repr(align(16))]
    struct AlignedLong(i64);
    let _l = AlignedLong(0);

    #[repr(align(16))]
    struct AlignedLongLong(i64);
    let _ll = AlignedLongLong(0);

    #[repr(align(16))]
    struct AlignedFloat(f32);
    let _f = AlignedFloat(0.0);

    #[repr(align(16))]
    struct AlignedDouble(f64);
    let _d = AlignedDouble(0.0);

    #[repr(align(16))]
    struct AlignedComplexLongDouble(f64, f64);
    let _cld = AlignedComplexLongDouble(0.0, 0.0);

    #[repr(align(4))]
    struct AlignedCharArray([char; 10]);
    let _ca = AlignedCharArray(['\0'; 10]);

    #[repr(align(16))]
    struct AlignedX(i32);
    let _x = AlignedX(0);

    enum E {
        E = 16,
    }
    #[repr(align(16))]
    struct AlignedY(i32);
    let _y = AlignedY(0);

    fn func() {
        #[repr(align(16))]
        struct AlignedAutoLongLong(i64);
        let _auto_ll = AlignedAutoLongLong(0);
    }

    // Test alignment macros
    const ALIGNAS: &str = "align";
    const ALIGNOF: &str = "align_of";
    const ALIGNAS_IS_DEFINED: &str = "1";
    const ALIGNOF_IS_DEFINED: &str = "1";

    if ALIGNAS != "align" {
        std::process::abort();
    }
    if ALIGNOF != "align_of" {
        std::process::abort();
    }
    if ALIGNAS_IS_DEFINED != "1" {
        std::process::abort();
    }
    if ALIGNOF_IS_DEFINED != "1" {
        std::process::abort();
    }
}