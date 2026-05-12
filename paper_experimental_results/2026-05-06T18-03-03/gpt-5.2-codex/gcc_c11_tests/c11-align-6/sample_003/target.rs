use std::mem::{align_of, align_of_val};
use std::os::raw::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint,
    c_ulong, c_ulonglong, c_ushort,
};

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct Complex<T> {
    re: T,
    im: T,
}

type LongDouble = f128;

fn check_align<T: Default>() {
    #[repr(C)]
    struct Wrapper<T> {
        c: u8,
        v: T,
    }
    let x = Wrapper {
        c: 0u8,
        v: T::default(),
    };
    if align_of::<T>() > align_of_val(&x.v) {
        std::process::abort();
    }
}

fn main() {
    check_align::<bool>();
    check_align::<c_char>();
    check_align::<c_schar>();
    check_align::<c_uchar>();
    check_align::<c_short>();
    check_align::<c_ushort>();
    check_align::<c_int>();
    check_align::<c_uint>();
    check_align::<c_long>();
    check_align::<c_ulong>();
    check_align::<c_longlong>();
    check_align::<c_ulonglong>();
    check_align::<c_float>();
    check_align::<c_double>();
    check_align::<LongDouble>();
    check_align::<Complex<c_float>>();
    check_align::<Complex<c_double>>();
    check_align::<Complex<LongDouble>>();
}