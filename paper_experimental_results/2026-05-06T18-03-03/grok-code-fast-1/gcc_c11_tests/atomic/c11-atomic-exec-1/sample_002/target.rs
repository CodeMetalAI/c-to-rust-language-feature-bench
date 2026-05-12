use std::process::exit;

fn abort() {
    panic!();
}

#[derive(Clone, Copy, PartialEq)]
struct Complex<T> {
    re: T,
    im: T,
}

fn cmplx<T>(re: T, im: T) -> Complex<T> {
    Complex { re, im }
}

#[derive(Clone, Copy, PartialEq)]
struct S {
    a: [i16; 1024],
}

fn test_bool(val: bool) {
    let mut a: bool = false;
    let b: bool = val;
    if a != false {
        abort();
    }
    if b != val {
        abort();
    }
    a = b;
    if a != val {
        abort();
    }
}

fn test_char(val: i8) {
    let mut a: i8 = 0;
    let b: i8 = val;
    if a != 0 {
        abort();
    }
    if b != val {
        abort();
    }
    a = b;
    if a != val {
        abort();
    }
}

fn test_signed_char(val: i8) {
    let mut a: i8 = 0;
    let b: i8 = val;
    if a != 0 {
        abort();
    }
    if b != val {
        abort();
    }
    a = b;
    if a != val {
        abort();
    }
}

fn test_unsigned_char(val: u8) {
    let mut a: u8 = 0;
    let b: u8 = val;
    if a != 0 {
        abort();
    }
    if b != val {
        abort();
    }
    a = b;
    if a != val {
        abort();
    }
}

fn test_signed_short(val: i16) {
    let mut a: i16 = 0;
    let b: i16 = val;
    if a != 0 {
        abort();
    }
    if b != val {
        abort();
    }
    a = b;
    if a != val {
        abort();
    }
}

fn test_unsigned_short(val: u16) {
    let mut a: u16 = 0;
    let b: u16 = val;
    if a != 0 {
        abort();
    }
    if b != val {
        abort();
    }
    a = b;
    if a != val {
        abort();
    }
}

fn test_signed_int(val: i32) {
    let mut a: i32 = 0;
    let b: i32 = val;
    if a != 0 {
        abort();
    }
    if b != val {
        abort();
    }
    a = b;
    if a != val {
        abort();
    }
}

fn test_unsigned_int(val: u32) {
    let mut a: u32 = 0;
    let b: u32 = val;
    if a != 0 {
        abort();
    }
    if b != val {
        abort();
    }
    a = b;
    if a != val {
        abort();
    }
}

fn test_signed_long(val: isize) {
    let mut a: isize = 0;
    let b: isize = val;
    if a != 0 {
        abort();
    }
    if b != val {
        abort();
    }
    a = b;
    if a != val {
        abort();
    }
}

fn test_unsigned_long(val: usize) {
    let mut a: usize = 0;
    let b: usize = val;
    if a != 0 {
        abort();
    }
    if b != val {
        abort();
    }
    a = b;
    if a != val {
        abort();
    }
}

fn test_signed_long_long(val: i64) {
    let mut a: i64 = 0;
    let b: i64 = val;
    if a != 0 {
        abort();
    }
    if b != val {
        abort();
    }
    a = b;
    if a != val {
        abort();
    }
}

fn test_unsigned_long_long(val: u64) {
    let mut a: u64 = 0;
    let b: u64 = val;
    if a != 0 {
        abort();
    }
    if b != val {
        abort();
    }
    a = b;
    if a != val {
        abort();
    }
}

fn test_float(val: f32) {
    let mut a: f32 = 0.0;
    let b: f32 = val;
    if a != 0.0 {
        abort();
    }
    if b != val {
        abort();
    }
    a = b;
    if a != val {
        abort();
    }
}

fn test_double(val: f64) {
    let mut a: f64 = 0.0;
    let b: f64 = val;
    if a != 0.0 {
        abort();
    }
    if b != val {
        abort();
    }
    a = b;
    if a != val {
        abort();
    }
}

fn test_long_double(val: f64) {
    let mut a: f64 = 0.0;
    let b: f64 = val;
    if a != 0.0 {
        abort();
    }
    if b != val {
        abort();
    }
    a = b;
    if a != val {
        abort();
    }
}

fn test_complex_float(val: Complex<f32>) {
    let mut a: Complex<f32> = cmplx(0.0, 0.0);
    let b: Complex<f32> = val;
    if a != cmplx(0.0, 0.0) {
        abort();
    }
    if b != val {
        abort();
    }
    a = b;
    if a != val {
        abort();
    }
}

fn test_complex_double(val: Complex<f64>) {
    let mut a: Complex<f64> = cmplx(0.0, 0.0);
    let b: Complex<f64> = val;
    if a != cmplx(0.0, 0.0) {
        abort();
    }
    if b != val {
        abort();
    }
    a = b;
    if a != val {
        abort();
    }
}

fn test_complex_long_double(val: Complex<f64>) {
    let mut a: Complex<f64> = cmplx(0.0, 0.0);
    let b: Complex<f64> = val;
    if a != cmplx(0.0, 0.0) {
        abort();
    }
    if b != val {
        abort();
    }
    a = b;
    if a != val {
        abort();
    }
}

fn test_ptr(val: *const i32) {
    let mut a: *const i32 = std::ptr::null();
    let b: *const i32 = val;
    if a != std::ptr::null() {
        abort();
    }
    if b != val {
        abort();
    }
    a = b;
    if a != val {
        abort();
    }
}

fn test_simple_assign() {
    macro_rules! test_arith {
        ($val:expr) => {
            test_bool($val != 0);
            test_char($val as i8);
            test_signed_char($val as i8);
            test_unsigned_char($val as u8);
            test_signed_short($val as i16);
            test_unsigned_short($val as u16);
            test_signed_int($val as i32);
            test_unsigned_int($val as u32);
            test_signed_long($val as isize);
            test_unsigned_long($val as usize);
            test_signed_long_long($val as i64);
            test_unsigned_long_long($val as u64);
            test_float($val as f32);
            test_double($val as f64);
            test_long_double($val as f64);
            test_complex_float(cmplx($val as f32, 0.0));
            test_complex_double(cmplx($val as f64, 0.0));
            test_complex_long_double(cmplx($val as f64, 0.0));
        };
    }
    test_arith!(0);
    test_arith!(1);
    test_arith!(2);
    test_arith!(-1);
    test_arith!(1i64 << 63);
    test_arith!(1.5);
    test_complex_float(cmplx(2.5, 3.5));
    test_complex_double(cmplx(2.5, 3.5));
    test_complex_long_double(cmplx(2.5, 3.5));
    static mut I: i32 = 0;
    test_ptr(std::ptr::null());
    test_ptr(unsafe { &I as *const i32 });
    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    let mut s1: S = S { a: [0; 1024] };
    let mut s2: S = S { a: [0; 1024] };
    let mut copy: S;
    s1 = init;
    copy = s1;
    if init != copy {
        abort();
    }
    s2 = s1;
    copy = s2;
    if init != copy {
        abort();
    }
    copy = s1;
    if init != copy {
        abort();
    }
    copy = s2;
    if init != copy {
        abort();
    }
}

fn main() {
    test_simple_assign();
    exit(0);
}