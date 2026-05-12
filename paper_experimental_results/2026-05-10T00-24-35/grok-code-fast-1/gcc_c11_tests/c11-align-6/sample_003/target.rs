use std::mem;
use std::process;

#[derive(Default)]
#[repr(C)]
struct ComplexFloat {
    re: f32,
    im: f32,
}

#[derive(Default)]
#[repr(C)]
struct ComplexDouble {
    re: f64,
    im: f64,
}

#[derive(Default)]
#[repr(C)]
struct ComplexLongDouble {
    re: f64,
    im: f64,
}

fn check_align<T: Default>() {
    #[derive(Default)]
    #[repr(C)]
    struct S<T> {
        c: u8,
        v: T,
    }
    let align_of_t = mem::align_of::<T>();
    let s = S::<T>::default();
    let align_of_v = mem::align_of_val(&s.v);
    if align_of_t > align_of_v {
        process::exit(1);
    }
}

fn main() {
    check_align::<bool>();
    check_align::<i8>(); // char and signed char
    check_align::<u8>(); // unsigned char
    check_align::<i16>(); // signed short
    check_align::<u16>(); // unsigned short
    check_align::<i32>(); // signed int
    check_align::<u32>(); // unsigned int
    check_align::<i64>(); // signed long and signed long long
    check_align::<u64>(); // unsigned long and unsigned long long
    check_align::<f32>(); // float
    check_align::<f64>(); // double and long double
    check_align::<ComplexFloat>(); // _Complex float
    check_align::<ComplexDouble>(); // _Complex double
    check_align::<ComplexLongDouble>(); // _Complex long double
    process::exit(0);
}