#[repr(C)]
struct ComplexFloat {
    re: f32,
    im: f32,
}

#[repr(C)]
struct ComplexDouble {
    re: f64,
    im: f64,
}

#[repr(C)]
struct ComplexLongDouble {
    re: f64,
    im: f64,
}

macro_rules! check_align {
    ($t:ty) => {
        if std::mem::align_of::<$t>() > std::mem::align_of::<$t>() {
            std::process::exit(1);
        }
    };
}

fn main() {
    check_align!(bool);
    check_align!(i8);
    check_align!(i8);
    check_align!(u8);
    check_align!(i16);
    check_align!(u16);
    check_align!(i32);
    check_align!(u32);
    check_align!(isize);
    check_align!(isize);
    check_align!(i64);
    check_align!(u64);
    check_align!(f32);
    check_align!(f64);
    check_align!(f64);
    check_align!(ComplexFloat);
    check_align!(ComplexDouble);
    check_align!(ComplexLongDouble);
    std::process::exit(0);
}