fn main() {
    macro_rules! check_align {
        ($t:ty) => {
            let _x = {
                #[repr(C)]
                struct X {
                    c: char,
                    v: $t,
                }
                X { c: '0', v: Default::default() }
            };
            let alignment_of_type = std::mem::align_of::<$t>();
            let alignment_of_field = std::mem::align_of_val(&_x.v);
            assert!(alignment_of_type <= alignment_of_field);
        };
    }

    check_align!(bool);
    check_align!(char);
    check_align!(i8);
    check_align!(u8);
    check_align!(i16);
    check_align!(u16);
    check_align!(i32);
    check_align!(u32);
    check_align!(i64);
    check_align!(u64);
    check_align!(i128);
    check_align!(u128);
    check_align!(f32);
    check_align!(f64);
    // Rust doesn't directly support complex types in the standard library,
    // and Rust doesn't have a native `long double` type; so complex and
    // extended precision types are omitted here.
}