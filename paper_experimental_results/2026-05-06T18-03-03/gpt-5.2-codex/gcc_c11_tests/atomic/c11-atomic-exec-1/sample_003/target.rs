use std::process::{abort, exit};

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

impl<T: Default + Copy> Default for Complex<T> {
    fn default() -> Self {
        Self {
            re: T::default(),
            im: T::default(),
        }
    }
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
    let temp = {
        a = b;
        a
    };
    if temp != value {
        abort();
    }
    if a != value {
        abort();
    }
}

fn test_simple_assign_arith_from_i128(val: i128) {
    test_simple_assign::<bool>(val != 0);
    test_simple_assign::<i8>(val as i8);
    test_simple_assign::<i8>(val as i8);
    test_simple_assign::<u8>(val as u8);
    test_simple_assign::<i16>(val as i16);
    test_simple_assign::<u16>(val as u16);
    test_simple_assign::<i32>(val as i32);
    test_simple_assign::<u32>(val as u32);
    test_simple_assign::<isize>(val as isize);
    test_simple_assign::<usize>(val as usize);
    test_simple_assign::<i64>(val as i64);
    test_simple_assign::<u64>(val as u64);
    test_simple_assign::<f32>(val as f32);
    test_simple_assign::<f64>(val as f64);
    test_simple_assign::<f64>(val as f64);
    test_simple_assign::<Complex<f32>>(Complex::new(val as f32, 0.0));
    test_simple_assign::<Complex<f64>>(Complex::new(val as f64, 0.0));
    test_simple_assign::<Complex<f64>>(Complex::new(val as f64, 0.0));
}

fn test_simple_assign_arith_from_u128(val: u128) {
    test_simple_assign::<bool>(val != 0);
    test_simple_assign::<i8>(val as i8);
    test_simple_assign::<i8>(val as i8);
    test_simple_assign::<u8>(val as u8);
    test_simple_assign::<i16>(val as i16);
    test_simple_assign::<u16>(val as u16);
    test_simple_assign::<i32>(val as i32);
    test_simple_assign::<u32>(val as u32);
    test_simple_assign::<isize>(val as isize);
    test_simple_assign::<usize>(val as usize);
    test_simple_assign::<i64>(val as i64);
    test_simple_assign::<u64>(val as u64);
    test_simple_assign::<f32>(val as f32);
    test_simple_assign::<f64>(val as f64);
    test_simple_assign::<f64>(val as f64);
    test_simple_assign::<Complex<f32>>(Complex::new(val as f32, 0.0));
    test_simple_assign::<Complex<f64>>(Complex::new(val as f64, 0.0));
    test_simple_assign::<Complex<f64>>(Complex::new(val as f64, 0.0));
}

fn test_simple_assign_arith_from_f64(val: f64, imag: f64) {
    test_simple_assign::<bool>(val != 0.0);
    test_simple_assign::<i8>(val as i8);
    test_simple_assign::<i8>(val as i8);
    test_simple_assign::<u8>(val as u8);
    test_simple_assign::<i16>(val as i16);
    test_simple_assign::<u16>(val as u16);
    test_simple_assign::<i32>(val as i32);
    test_simple_assign::<u32>(val as u32);
    test_simple_assign::<isize>(val as isize);
    test_simple_assign::<usize>(val as usize);
    test_simple_assign::<i64>(val as i64);
    test_simple_assign::<u64>(val as u64);
    test_simple_assign::<f32>(val as f32);
    test_simple_assign::<f64>(val as f64);
    test_simple_assign::<f64>(val as f64);
    test_simple_assign::<Complex<f32>>(Complex::new(val as f32, imag as f32));
    test_simple_assign::<Complex<f64>>(Complex::new(val as f64, imag as f64));
    test_simple_assign::<Complex<f64>>(Complex::new(val as f64, imag as f64));
}

#[derive(Copy, Clone, PartialEq)]
struct S {
    a: [i16; 1024],
}

impl Default for S {
    fn default() -> Self {
        Self { a: [0; 1024] }
    }
}

fn test_simple_assign_all() {
    test_simple_assign_arith_from_i128(0);
    test_simple_assign_arith_from_i128(1);
    test_simple_assign_arith_from_i128(2);
    test_simple_assign_arith_from_i128(-1);
    test_simple_assign_arith_from_u128(1u128 << 63);
    test_simple_assign_arith_from_f64(1.5, 0.0);
    test_simple_assign_arith_from_f64(2.5, 3.5);

    static I: i32 = 0;
    test_simple_assign::<*const i32>(std::ptr::null());
    test_simple_assign::<*const i32>(&I as *const i32);

    let mut init = S::default();
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    let mut s1 = S::default();
    let mut s2 = S::default();
    let mut copy: S;

    copy = {
        s1 = init;
        s1
    };
    if init != copy {
        abort();
    }

    copy = {
        s2 = s1;
        s2
    };
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
    exit(0);
}