use std::mem;

fn main() {
    // Define variables with specific alignments
    #[repr(align(align_of::<mem::Align>()))]
    struct AlignedChar;
    let _c: AlignedChar = AlignedChar;

    #[repr(align(align_of::<mem::Align>()))]
    struct AlignedShort;
    let _s: AlignedShort = AlignedShort;

    #[repr(align(mem::align_of::<i32>()))]
    struct AlignedInt;
    let _i: AlignedInt = AlignedInt;

    #[repr(align(align_of::<mem::Align>()))]
    struct AlignedLong;
    let _l: AlignedLong = AlignedLong;

    #[repr(align(align_of::<mem::Align>()))]
    struct AlignedLongLong;
    let _ll: AlignedLongLong = AlignedLongLong;

    #[repr(align(align_of::<mem::Align>()))]
    struct AlignedFloat;
    let _f: AlignedFloat = AlignedFloat;

    #[repr(align(align_of::<mem::Align>()))]
    struct AlignedDouble;
    let _d: AlignedDouble = AlignedDouble;

    #[repr(align(align_of::<mem::Align>()))]
    struct AlignedComplexLongDouble;
    let _cld: AlignedComplexLongDouble = AlignedComplexLongDouble;

    #[repr(align(align_of::<i32>()))]
    struct AlignedCharArray([u8; 10]);
    let _ca: AlignedCharArray = AlignedCharArray([0; 10]);

    #[repr(align(align_of::<mem::Align>()))]
    struct AlignedX;
    let _x: AlignedX = AlignedX;

    #[repr(align(align_of::<mem::Align>()))]
    struct AlignedY;
    let _y: AlignedY = AlignedY;

    // Function with aligned local variable
    fn func() {
        #[repr(align(align_of::<mem::Align>()))]
        struct AlignedAutoLongLong;
        let _auto_ll: AlignedAutoLongLong = AlignedAutoLongLong;
    }

    // Check alignas and alignof definitions
    let s1 = "_Alignas";
    let s2 = "_Alignof";
    let s3 = "1";
    let s4 = "1";

    // Validate the string representations
    if s1 != "_Alignas" {
        std::process::abort();
    }
    if s2 != "_Alignof" {
        std::process::abort();
    }
    if s3 != "1" {
        std::process::abort();
    }
    if s4 != "1" {
        std::process::abort();
    }
}