use std::process::{abort, exit};

#[derive(Clone, Copy, PartialEq, Default)]
struct ComplexFloat {
    re: f32,
    im: f32,
}

#[derive(Clone, Copy, PartialEq, Default)]
struct ComplexDouble {
    re: f64,
    im: f64,
}

#[derive(Clone, Copy, PartialEq, Default)]
struct ComplexLongDouble {
    re: f64,
    im: f64,
}

#[derive(Clone, Copy, PartialEq, Default)]
struct S {
    a: [i16; 1024],
}

fn cmplx_d(re: f64, im: f64) -> ComplexDouble {
    ComplexDouble { re, im }
}

macro_rules! test_simple_assign {
    ($t:ty, $v:expr) => {
        let mut a: $t = Default::default();
        let b: $t = $v;
        if a != (0 as $t) {
            abort();
        }
        if b != $v {
            abort();
        }
        a = b;
        if a != $v {
            abort();
        }
        if a != $v {
            abort();
        }
    };
}

fn test_simple_assign_arith(v: i64, vf: f64, vc: ComplexDouble) {
    test_simple_assign!(bool, v != 0);
    test_simple_assign!(i8, v as i8);
    test_simple_assign!(i8, v as i8);
    test_simple_assign!(u8, v as u8);
    test_simple_assign!(i16, v as i16);
    test_simple_assign!(u16, v as u16);
    test_simple_assign!(i32, v as i32);
    test_simple_assign!(u32, v as u32);
    test_simple_assign!(i64, v);
    test_simple_assign!(u64, v as u64);
    test_simple_assign!(i64, v);
    test_simple_assign!(u64, v as u64);
    test_simple_assign!(f32, vf as f32);
    test_simple_assign!(f64, vf);
    test_simple_assign!(f64, vf);
    test_simple_assign!(ComplexFloat, ComplexFloat { re: vf as f32, im: 0.0 });
    test_simple_assign!(ComplexDouble, vc);
    test_simple_assign!(ComplexLongDouble, ComplexLongDouble { re: vf, im: 0.0 });
}

fn test_simple_assign() {
    test_simple_assign_arith(0, 0.0, cmplx_d(0.0, 0.0));
    test_simple_assign_arith(1, 1.0, cmplx_d(1.0, 1.0));
    test_simple_assign_arith(2, 2.0, cmplx_d(2.0, 2.0));
    test_simple_assign_arith(-1, -1.0, cmplx_d(-1.0, -1.0));
    test_simple_assign_arith(1i64 << 63, (1i64 << 63) as f64, cmplx_d((1i64 << 63) as f64, (1i64 << 63) as f64));
    test_simple_assign_arith(0, 1.5, cmplx_d(1.5, 0.0));
    test_simple_assign_arith(0, 0.0, cmplx_d(2.5, 3.5));
    static I: i32 = 0;
    test_simple_assign!(*mut i32, std::ptr::null_mut());
    test_simple_assign!(*mut i32, &I as *const i32 as *mut i32);
    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    let mut s1 = S::default();
    let mut s2 = S::default();
    let mut copy = S::default();
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