use std::mem;

macro_rules! CHECK_ALIGN {
    ($TYPE:ty) => {
        struct X {
            c: char,
            v: $TYPE,
        }
        if mem::align_of::<$TYPE>() > mem::align_of_val(&X { c: 'a', v: 0 }.v) {
            std::process::exit(1);
        }
    };
}

fn main() {
    CHECK_ALIGN!(bool);
    CHECK_ALIGN!(i8);
    CHECK_ALIGN!(u8);
    CHECK_ALIGN!(i16);
    CHECK_ALIGN!(u16);
    CHECK_ALIGN!(i32);
    CHECK_ALIGN!(u32);
    CHECK_ALIGN!(i64);
    CHECK_ALIGN!(u64);
    CHECK_ALIGN!(isize);
    CHECK_ALIGN!(usize);
    CHECK_ALIGN!(f32);
    CHECK_ALIGN!(f64);
    CHECK_ALIGN!(i128);
    CHECK_ALIGN!(u128);
    std::process::exit(0);
}