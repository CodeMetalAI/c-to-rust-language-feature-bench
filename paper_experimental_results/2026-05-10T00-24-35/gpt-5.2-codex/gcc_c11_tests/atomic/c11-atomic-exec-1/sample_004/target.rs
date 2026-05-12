use std::process;

#[derive(Copy, Clone, PartialEq)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Self {
        Self { re, im }
    }
}

type Complex32 = Complex<f32>;
type Complex64 = Complex<f64>;

fn abort() -> ! {
    process::abort();
}

macro_rules! test_simple_assign {
    ($t:ty, $zero:expr, $value:expr) => {{
        let mut a: $t = $zero;
        let b: $t = $value;
        if a != $zero {
            abort();
        }
        if b != $value {
            abort();
        }
        a = b;
        if a != $value {
            abort();
        }
    }};
}

fn test_simple_assign_arith_from_int(val: i128) {
    test_simple_assign!(bool, false, val != 0);
    test_simple_assign!(i8, 0i8, val as i8);
    test_simple_assign!(i8, 0i8, val as i8);
    test_simple_assign!(u8, 0u8, val as u8);
    test_simple_assign!(i16, 0i16, val as i16);
    test_simple_assign!(u16, 0u16, val as u16);
    test_simple_assign!(i32, 0i32, val as i32);
    test_simple_assign!(u32, 0u32, val as u32);
    test_simple_assign!(i64, 0i64, val as i64);
    test_simple_assign!(u64, 0u64, val as u64);
    test_simple_assign!(i64, 0i64, val as i64);
    test_simple_assign!(u64, 0u64, val as u64);
    test_simple_assign!(f32, 0f32, val as f32);
    test_simple_assign!(f64, 0f64, val as f64);
    test_simple_assign!(f64, 0f64, val as f64);
    test_simple_assign!(
        Complex32,
        Complex32::new(0.0, 0.0),
        Complex32::new(val as f32, 0.0)
    );
    test_simple_assign!(
        Complex64,
        Complex64::new(0.0, 0.0),
        Complex64::new(val as f64, 0.0)
    );
    test_simple_assign!(
        Complex64,
        Complex64::new(0.0, 0.0),
        Complex64::new(val as f64, 0.0)
    );
}

fn test_simple_assign_arith_from_float(val: f64) {
    test_simple_assign!(bool, false, val != 0.0);
    test_simple_assign!(i8, 0i8, val as i8);
    test_simple_assign!(i8, 0i8, val as i8);
    test_simple_assign!(u8, 0u8, val as u8);
    test_simple_assign!(i16, 0i16, val as i16);
    test_simple_assign!(u16, 0u16, val as u16);
    test_simple_assign!(i32, 0i32, val as i32);
    test_simple_assign!(u32, 0u32, val as u32);
    test_simple_assign!(i64, 0i64, val as i64);
    test_simple_assign!(u64, 0u64, val as u64);
    test_simple_assign!(i64, 0i64, val as i64);
    test_simple_assign!(u64, 0u64, val as u64);
    test_simple_assign!(f32, 0f32, val as f32);
    test_simple_assign!(f64, 0f64, val as f64);
    test_simple_assign!(f64, 0f64, val as f64);
    test_simple_assign!(
        Complex32,
        Complex32::new(0.0, 0.0),
        Complex32::new(val as f32, 0.0)
    );
    test_simple_assign!(
        Complex64,
        Complex64::new(0.0, 0.0),
        Complex64::new(val as f64, 0.0)
    );
    test_simple_assign!(
        Complex64,
        Complex64::new(0.0, 0.0),
        Complex64::new(val as f64, 0.0)
    );
}

fn test_simple_assign_arith_from_complex(re: f64, im: f64) {
    test_simple_assign!(bool, false, re != 0.0);
    test_simple_assign!(i8, 0i8, re as i8);
    test_simple_assign!(i8, 0i8, re as i8);
    test_simple_assign!(u8, 0u8, re as u8);
    test_simple_assign!(i16, 0i16, re as i16);
    test_simple_assign!(u16, 0u16, re as u16);
    test_simple_assign!(i32, 0i32, re as i32);
    test_simple_assign!(u32, 0u32, re as u32);
    test_simple_assign!(i64, 0i64, re as i64);
    test_simple_assign!(u64, 0u64, re as u64);
    test_simple_assign!(i64, 0i64, re as i64);
    test_simple_assign!(u64, 0u64, re as u64);
    test_simple_assign!(f32, 0f32, re as f32);
    test_simple_assign!(f64, 0f64, re as f64);
    test_simple_assign!(f64, 0f64, re as f64);
    test_simple_assign!(
        Complex32,
        Complex32::new(0.0, 0.0),
        Complex32::new(re as f32, im as f32)
    );
    test_simple_assign!(
        Complex64,
        Complex64::new(0.0, 0.0),
        Complex64::new(re as f64, im as f64)
    );
    test_simple_assign!(
        Complex64,
        Complex64::new(0.0, 0.0),
        Complex64::new(re as f64, im as f64)
    );
}

#[derive(Copy, Clone, PartialEq)]
struct S {
    a: [i16; 1024],
}

fn test_simple_assign() {
    test_simple_assign_arith_from_int(0);
    test_simple_assign_arith_from_int(1);
    test_simple_assign_arith_from_int(2);
    test_simple_assign_arith_from_int(-1);
    test_simple_assign_arith_from_int((1u128 << 63) as i128);
    test_simple_assign_arith_from_float(1.5);
    test_simple_assign_arith_from_complex(2.5, 3.5);

    let i: i32 = 0;
    test_simple_assign!(*const i32, std::ptr::null(), std::ptr::null());
    test_simple_assign!(*const i32, std::ptr::null(), &i as *const i32);

    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    let mut s1 = S { a: [0; 1024] };
    let mut s2 = S { a: [0; 1024] };
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
    process::exit(0);
}