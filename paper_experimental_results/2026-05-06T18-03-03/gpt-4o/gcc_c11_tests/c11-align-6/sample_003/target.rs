fn main() {
    macro_rules! check_align {
        ($type:ty) => {
            {
                struct X {
                    c: u8,
                    v: $type,
                }
                if std::mem::align_of::<$type>() > std::mem::align_of::<X>() {
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
    check_align!(i128);
    check_align!(u128);
    check_align!(f32);
    check_align!(f64);

    // Rust does not have a direct equivalent for long double or complex types.
    // Assuming long double is equivalent to f64 for this context.
    // Complex numbers can be represented using tuples or a custom struct.
    // Here, we will use a tuple of two f32/f64 for complex numbers.
    check_align!((f32, f32)); // _Complex float
    check_align!((f64, f64)); // _Complex double
    check_align!((f64, f64)); // _Complex long double

    std::process::exit(0);
}