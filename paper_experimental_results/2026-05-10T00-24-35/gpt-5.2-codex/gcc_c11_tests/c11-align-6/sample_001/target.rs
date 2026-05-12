use std::mem::align_of;
use std::os::raw::{
    c_char, c_int, c_long, c_longdouble, c_longlong, c_schar, c_short, c_uchar, c_uint, c_ulong,
    c_ulonglong, c_ushort,
};
use std::process::abort;

#[repr(C)]
#[derive(Copy, Clone)]
struct Complex<T> {
    re: T,
    im: T,
}

fn check_align<T>() {
    #[repr(C)]
    struct Wrapper<T> {
        c: u8,
        v: T,
    }

    if align_of::<T>() > align_of::<Wrapper<T>>() {
        abort();
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
    check_align::<f32>();
    check_align::<f64>();
    check_align::<c_longdouble>();
    check_align::<Complex<f32>>();
    check_align::<Complex<f64>>();
    check_align::<Complex<c_longdouble>>();
    std::process::exit(0);
}