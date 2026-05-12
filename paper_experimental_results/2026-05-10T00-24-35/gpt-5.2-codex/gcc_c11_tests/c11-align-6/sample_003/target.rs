use std::mem::align_of;
use std::os::raw::{
    c_char, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint, c_ulong,
    c_ulonglong, c_ushort,
};

#[repr(C, align(16))]
struct CLongDouble([u8; 16]);

#[repr(C)]
struct CComplex<T> {
    re: T,
    im: T,
}

#[repr(C)]
struct S<T> {
    c: u8,
    v: T,
}

fn check_align<T>() {
    if align_of::<T>() > align_of::<S<T>>() {
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
    check_align::<f32>();
    check_align::<f64>();
    check_align::<CLongDouble>();
    check_align::<CComplex<f32>>();
    check_align::<CComplex<f64>>();
    check_align::<CComplex<CLongDouble>>();
    std::process::exit(0);
}