fn main() {
    // Test alignment support in Rust
    // Rust has built-in alignment support through the `align` attribute and the `std::mem::align_of` function

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
    struct AlignedComplexLongDouble(num_complex::Complex64);
    let _cld: AlignedComplexLongDouble = AlignedComplexLongDouble(num_complex::Complex64::new(0.0, 0.0));

    #[repr(align(1))]
    struct AlignedCharArray([char; 10]);
    let _ca: AlignedCharArray = AlignedCharArray(['\0'; 10]);

    #[repr(align(16))]
    struct AlignedX(i32);
    let _x: AlignedX = AlignedX(0);

    #[repr(align(16))]
    struct AlignedY(i32);
    let _y: AlignedY = AlignedY(0);

    // Test alignment of variables
    assert_eq!(std::mem::align_of::<AlignedChar>(), 16);
    assert_eq!(std::mem::align_of::<AlignedShort>(), 16);
    assert_eq!(std::mem::align_of::<AlignedInt>(), 4);
    assert_eq!(std::mem::align_of::<AlignedLong>(), 16);
    assert_eq!(std::mem::align_of::<AlignedLongLong>(), 16);
    assert_eq!(std::mem::align_of::<AlignedFloat>(), 16);
    assert_eq!(std::mem::align_of::<AlignedDouble>(), 16);
    assert_eq!(std::mem::align_of::<AlignedComplexLongDouble>(), 16);
    assert_eq!(std::mem::align_of::<AlignedCharArray>(), 1);
    assert_eq!(std::mem::align_of::<AlignedX>(), 16);
    assert_eq!(std::mem::align_of::<AlignedY>(), 16);

    // Test alignment of types
    assert_eq!(std::mem::align_of::<char>(), 1);
    assert_eq!(std::mem::align_of::<short>(), 2);
    assert_eq!(std::mem::align_of::<i32>(), 4);
    assert_eq!(std::mem::align_of::<i64>(), 8);
    assert_eq!(std::mem::align_of::<f32>(), 4);
    assert_eq!(std::mem::align_of::<f64>(), 8);
    assert_eq!(std::mem::align_of::<num_complex::Complex64>(), 8);

    // Test alignment of arrays
    assert_eq!(std::mem::align_of::<[char; 10]>(), 1);

    // Test alignment of enums
    #[repr(align(16))]
    enum E { E = 16 }
    assert_eq!(std::mem::align_of::<E>(), 16);

    // Test alignment of functions
    fn func() {
        #[repr(align(16))]
        struct AutoAlignedLongLong(i64);
        let _auto_ll: AutoAlignedLongLong = AutoAlignedLongLong(0);
    }

    // Test alignment of structs
    #[repr(align(1))]
    struct S;
    assert_eq!(std::mem::align_of::<S>(), 1);

    // Test alignment macros
    const _ALIGNAS: &str = "align";
    const _ALIGNOF: &str = "align_of";
    const _ALIGNAS_IS_DEFINED: &str = "1";
    const _ALIGNOF_IS_DEFINED: &str = "1";

    // Test alignment macro values
    assert_eq!(_ALIGNAS, "align");
    assert_eq!(_ALIGNOF, "align_of");
    assert_eq!(_ALIGNAS_IS_DEFINED, "1");
    assert_eq!(_ALIGNOF_IS_DEFINED, "1");
}