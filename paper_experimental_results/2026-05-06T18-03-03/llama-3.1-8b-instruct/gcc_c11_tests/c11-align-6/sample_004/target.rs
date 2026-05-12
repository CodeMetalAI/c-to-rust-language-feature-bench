fn main() {
    #[allow(unused_macros)]
    macro_rules! check_align {
        ($type:ty) => {{
            let x = [$type: 0];
            if std::mem::align_of::<$type>() > std::mem::align_of::<[u8; 1]>() {
                std::process::abort();
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
    check_align!(f32);
    check_align!(f64);
    check_align!(f80);
    check_align!(complex_f32);
    check_align!(complex_f64);
    check_align!(complex_f80);

    std::process::exit(0);
}