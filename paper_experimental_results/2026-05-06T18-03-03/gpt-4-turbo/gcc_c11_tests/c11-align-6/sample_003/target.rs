fn main() {
    macro_rules! check_align {
        ($t:ty) => {{
            #[repr(C)]
            struct Test {
                c: u8,
                v: $t,
            }
            let alignment_of_v = std::mem::align_of::<$t>();
            let offset_of_v = std::mem::align_of::<Test>();
            if alignment_of_v > offset_of_v {
                panic!();
            }
        }};
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
    // Rust does not have built-in complex number types, so we skip those checks.
}