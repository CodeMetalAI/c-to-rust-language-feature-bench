fn main() {
    macro_rules! check_align {
        ($t:ty) => {
            struct Wrapper {
                c: u8,
                v: $t,
            }

            if std::mem::align_of::<$t>() > std::mem::align_of::<Wrapper>() {
                std::process::abort();
            }
        };
    }

    check_align!(bool);
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

    std::process::exit(0);
}