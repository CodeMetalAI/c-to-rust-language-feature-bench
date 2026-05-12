use std::mem::align_of;

#[repr(C)]
struct Complex<T> {
    re: T,
    im: T,
}

type LongDouble = f64;

fn check_align<T>() {
    #[repr(C)]
    struct Wrapper<T> {
        c: u8,
        v: T,
    }
    let align_type = align_of::<T>();
    let align_field = align_of::<Wrapper<T>>();
    if align_type > align_field {
        std::process::abort();
    }
}

fn main() {
    check_align::<bool>();
    check_align::<i8>(); // char
    check_align::<i8>(); // signed char
    check_align::<u8>(); // unsigned char
    check_align::<i16>(); // signed short
    check_align::<u16>(); // unsigned short
    check_align::<i32>(); // signed int
    check_align::<u32>(); // unsigned int
    check_align::<isize>(); // signed long
    check_align::<usize>(); // unsigned long
    check_align::<i64>(); // signed long long
    check_align::<u64>(); // unsigned long long
    check_align::<f32>(); // float
    check_align::<f64>(); // double
    check_align::<LongDouble>(); // long double
    check_align::<Complex<f32>>(); // _Complex float
    check_align::<Complex<f64>>(); // _Complex double
    check_align::<Complex<LongDouble>>(); // _Complex long double
}