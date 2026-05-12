fn main() {
    macro_rules! check_align {
        ($type:ty) => {{
            struct X {
                _c: (),
                _v: $type,
            }
            let x = X { _c: (), _v: std::mem::MaybeUninit::uninit().assume_init() };
            let align = std::mem::align_of::<$type>();
            let actual_align = std::mem::align_of_val(&x._v);
            if align > actual_align {
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
    // Note: Rust does not have a direct equivalent of _Complex float,
    // _Complex double, or _Complex long double. We can use a struct
    // with two f32, f64, or f64 fields to mimic the behavior.
    check_align!([f32; 2]);
    check_align!([f64; 2]);
    check_align!([f64; 2]);
}