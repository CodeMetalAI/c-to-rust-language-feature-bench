use std::process;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Complex<T> {
    real: T,
    imag: T,
}

macro_rules! cmplx {
    ($x:expr, $y:expr) => {
        Complex { real: $x, imag: $y }
    };
}

macro_rules! test_simple_assign {
    ($t:ty, $val:expr) => {{
        let mut a: $t = Default::default();
        let b: $t = $val as $t;
        if a != (0 as $t) {
            abort();
        }
        if b != ($val as $t) {
            abort();
        }
        a = b;
        if a != ($val as $t) {
            abort();
        }
    }};
}

macro_rules! test_simple_assign_ptr {
    ($val:expr) => {{
        let mut a: *const i32 = std::ptr::null();
        let b: *const i32 = $val;
        if a != std::ptr::null() {
            abort();
        }
        if b != $val {
            abort();
        }
        a = b;
        if a != $val {
            abort();
        }
    }};
}

#[derive(Clone, PartialEq)]
struct S {
    a: [i16; 1024],
}

fn init_struct() -> S {
    let mut s = S { a: [0; 1024] };
    for i in 0..1024 {
        s.a[i] = i;
    }
    s
}

fn abort() {
    process::exit(1);
}

fn test_simple_assign_arith(value: i64) {
    test_simple_assign!(bool, value);
    test_simple_assign!(i8, value);
    test_simple_assign!(u8, value);
    test_simple_assign!(i16, value);
    test_simple_assign!(u16, value);
    test_simple_assign!(i32, value);
    test_simple_assign!(u32, value);
    test_simple_assign!(i64, value);
    test_simple_assign!(u64, value);
}

fn test_simple_assign_float(value: f64) {
    test_simple_assign!(f32, value);
    test_simple_assign!(f64, value);
}

fn test_simple_assign_complex(value: Complex<f64>) {
    test_simple_assign!(Complex<f32>, cmplx!(value.real as f32, value.imag as f32));
    test_simple_assign!(Complex<f64>, value);
}

fn test_simple_assign() {
    test_simple_assign_arith(0);
    test_simple_assign_arith(1);
    test_simple_assign_arith(2);
    test_simple_assign_arith(-1);
    let large = 1u64 << 63;
    test_simple_assign_arith(large as i64);
    test_simple_assign_float(1.5);
    let cmplx_val = cmplx!(2.5, 3.5);
    test_simple_assign_complex(cmplx_val);
    let mut i = 0;
    test_simple_assign_ptr!(std::ptr::null());
    test_simple_assign_ptr!(&i as *const i32);
    let init = init_struct();
    let mut s1 = S { a: [0; 1024] };
    let mut s2 = S { a: [0; 1024] };
    let mut copy = S { a: [0; 1024] };
    s1 = init.clone();
    copy = s1.clone();
    if init.a != copy.a {
        abort();
    }
    copy = (s2 = s1.clone());
    if init.a != copy.a {
        abort();
    }
    copy = s1.clone();
    if init.a != copy.a {
        abort();
    }
    copy = s2.clone();
    if init.a != copy.a {
        abort();
    }
}

fn main() {
    test_simple_assign();
    process::exit(0);
}