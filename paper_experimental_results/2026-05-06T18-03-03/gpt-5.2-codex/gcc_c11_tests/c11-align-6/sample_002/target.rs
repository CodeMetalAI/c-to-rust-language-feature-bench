use std::mem::align_of;
use std::os::raw::{
    c_char, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint, c_ulong,
    c_ulonglong, c_ushort,
};
use std::process;

fn check_align<T>() {
    #[repr(C)]
    struct X<T> {
        c: u8,
        v: T,
    }
    if align_of::<T>() > align_of::<X<T>>() {
        process::abort();
    }
}

#[repr(C, align(16))]
struct LongDouble([u8; 16]);

#[repr(C)]
struct ComplexF32 {
    re: f32,
    im: f32,
}

#[repr(C)]
struct ComplexF64 {
    re: f64,
    im: f64,
}

#[repr(C)]
struct ComplexLongDouble {
    re: LongDouble,
    im: LongDouble,
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
    check_align::<LongDouble>();
    check_align::<ComplexF32>();
    check_align::<ComplexF64>();
    check_align::<ComplexLongDouble>();
    process::exit(0);
}