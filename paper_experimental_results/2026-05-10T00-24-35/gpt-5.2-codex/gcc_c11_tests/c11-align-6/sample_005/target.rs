use std::mem::align_of;
use std::os::raw::{
    c_char, c_int, c_long, c_longlong, c_short, c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort,
};
use std::process::{abort, exit};

#[repr(C)]
struct LongDouble {
    bytes: [u8; 16],
}

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

fn check_align<T>() {
    #[repr(C)]
    struct S<T> {
        c: u8,
        v: T,
    }
    let align_t = align_of::<T>();
    let align_v = align_of::<T>();
    if align_t > align_v {
        abort();
    }
    let _ = align_of::<S<T>>();
}

fn main() {
    check_align::<bool>();
    check_align::<c_char>();
    check_align::<i8>();
    check_align::<u8>();
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
    exit(0);
}