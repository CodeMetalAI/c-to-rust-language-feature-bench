fn main() {
    struct TestStruct<T> {
        c: char,
        v: T,
    }

    macro_rules! check_align {
        ($ty:ty) => {
            let test = TestStruct {
                c: 'a',
                v: $ty::default(),
            };
            if std::mem::align_of::<$ty>() > std::mem::align_of::<test.v>() {
                std::process::exit(1);
            }
        };
    }

    check_align!(std::primitive::bool);
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
    check_align!(f32x);
    check_align!(f64x);
    std::process::exit(0);
}