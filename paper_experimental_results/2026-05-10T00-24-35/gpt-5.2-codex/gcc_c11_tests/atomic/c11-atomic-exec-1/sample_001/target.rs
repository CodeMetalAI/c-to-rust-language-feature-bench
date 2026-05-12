use std::process;

#[derive(Copy, Clone, PartialEq)]
struct Complex32 {
    re: f32,
    im: f32,
}

impl Complex32 {
    fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }
}

impl Default for Complex32 {
    fn default() -> Self {
        Self { re: 0.0, im: 0.0 }
    }
}

#[derive(Copy, Clone, PartialEq)]
struct Complex64 {
    re: f64,
    im: f64,
}

impl Complex64 {
    fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
}

impl Default for Complex64 {
    fn default() -> Self {
        Self { re: 0.0, im: 0.0 }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct S {
    a: [i16; 1024],
}

impl Default for S {
    fn default() -> Self {
        Self { a: [0; 1024] }
    }
}

fn abort() -> ! {
    process::abort()
}

fn test_simple_assign<T: Copy + PartialEq + Default>(value: T) {
    let mut a: T = Default::default();
    let mut b: T = value;
    if a != Default::default() {
        abort();
    }
    if b != value {
        abort();
    }
    a = b;
    if a != value {
        abort();
    }
    if a != value {
        abort();
    }
}

macro_rules! test_simple_assign_scalar {
    ($val:expr) => {{
        let v = $val;
        let v_f = v as f64;
        test_simple_assign::<bool>(v_f != 0.0);
        test_simple_assign::<i8>(v as i8); // char
        test_simple_assign::<i8>(v as i8); // signed char
        test_simple_assign::<u8>(v as u8);
        test_simple_assign::<i16>(v as i16);
        test_simple_assign::<u16>(v as u16);
        test_simple_assign::<i32>(v as i32);
        test_simple_assign::<u32>(v as u32);
        test_simple_assign::<isize>(v as isize);
        test_simple_assign::<usize>(v as usize);
        test_simple_assign::<i64>(v as i64);
        test_simple_assign::<u64>(v as u64);
        test_simple_assign::<f32>(v as f32);
        test_simple_assign::<f64>(v as f64);
        test_simple_assign::<f64>(v as f64); // long double
        test_simple_assign::<Complex32>(Complex32::new(v as f32, 0.0));
        test_simple_assign::<Complex64>(Complex64::new(v as f64, 0.0));
        test_simple_assign::<Complex64>(Complex64::new(v as f64, 0.0)); // long double complex
    }};
}

macro_rules! test_simple_assign_complex {
    ($val:expr) => {{
        let v = $val;
        let real = v.re;
        let real_f = real as f64;
        test_simple_assign::<bool>(real_f != 0.0);
        test_simple_assign::<i8>(real as i8); // char
        test_simple_assign::<i8>(real as i8); // signed char
        test_simple_assign::<u8>(real as u8);
        test_simple_assign::<i16>(real as i16);
        test_simple_assign::<u16>(real as u16);
        test_simple_assign::<i32>(real as i32);
        test_simple_assign::<u32>(real as u32);
        test_simple_assign::<isize>(real as isize);
        test_simple_assign::<usize>(real as usize);
        test_simple_assign::<i64>(real as i64);
        test_simple_assign::<u64>(real as u64);
        test_simple_assign::<f32>(real as f32);
        test_simple_assign::<f64>(real as f64);
        test_simple_assign::<f64>(real as f64); // long double
        test_simple_assign::<Complex32>(Complex32::new(v.re as f32, v.im as f32));
        test_simple_assign::<Complex64>(Complex64::new(v.re as f64, v.im as f64));
        test_simple_assign::<Complex64>(Complex64::new(v.re as f64, v.im as f64)); // long double complex
    }};
}

fn test_simple_assign_all() {
    test_simple_assign_scalar!(0i64);
    test_simple_assign_scalar!(1i64);
    test_simple_assign_scalar!(2i64);
    test_simple_assign_scalar!(-1i64);
    test_simple_assign_scalar!(1u64 << 63);
    test_simple_assign_scalar!(1.5f64);
    test_simple_assign_complex!(Complex64::new(2.5, 3.5));

    let i: i32 = 0;
    test_simple_assign::<*const i32>(std::ptr::null());
    test_simple_assign::<*const i32>(&i as *const i32);

    let mut init = S::default();
    let mut copy = S::default();
    let mut s1 = S::default();
    let mut s2 = S::default();

    for j in 0..1024 {
        init.a[j] = j as i16;
    }

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
    test_simple_assign_all();
}