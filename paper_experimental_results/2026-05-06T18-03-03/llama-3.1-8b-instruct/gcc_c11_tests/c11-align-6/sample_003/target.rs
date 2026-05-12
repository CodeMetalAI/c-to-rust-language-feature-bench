fn main() {
    use std::mem;

    #[test]
    fn test_align() {
        struct Test<T> {
            _c: u8,
            v: T,
        }

        macro_rules! check_align {
            ($ty:ty) => {{
                let test = Test { _c: 0, v: $ty::default() };
                if mem::size_of::<$ty>() > mem::size_of::<Test<$ty>>() {
                    panic!("Alignment mismatch for type {}", stringify!($ty));
                }
            }};
        }

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
        check_align!(isize);
        check_align!(usize);
    }

    test_align();
    std::process::exit(0);
}