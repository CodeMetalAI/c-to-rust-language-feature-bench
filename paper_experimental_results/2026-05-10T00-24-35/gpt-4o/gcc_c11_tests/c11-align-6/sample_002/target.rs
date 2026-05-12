fn main() {
    macro_rules! check_align {
        ($type:ty) => {
            {
                struct Wrapper {
                    c: u8,
                    v: $type,
                }
                let x = Wrapper { c: 0, v: <$type>::default() };
                if std::mem::align_of::<$type>() > std::mem::align_of_val(&x.v) {
                    std::process::abort();
                }
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
    check_align!(i128); // Rust does not have 'long long', but i128 is the closest match
    check_align!(u128);
    check_align!(f32);
    check_align!(f64);
    // Rust does not have an equivalent for long double, but f64 is generally used
    // Rust does not have complex numbers in its standard library

    std::process::exit(0);
}