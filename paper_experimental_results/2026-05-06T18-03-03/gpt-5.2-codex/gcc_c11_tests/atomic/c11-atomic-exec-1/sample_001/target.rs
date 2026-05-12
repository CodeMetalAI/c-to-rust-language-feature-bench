use std::process::{abort, exit};
use std::ptr;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<T: Default> Default for Complex<T> {
    fn default() -> Self {
        Self {
            re: Default::default(),
            im: Default::default(),
        }
    }
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Self {
        Self { re, im }
    }
}

#[derive(Copy, Clone)]
enum Value {
    Int(i128),
    Float(f64),
    Complex(Complex<f64>),
}

impl Value {
    fn to_bool(self) -> bool {
        match self {
            Value::Int(i) => i != 0,
            Value::Float(f) => f != 0.0,
            Value::Complex(c) => c.re != 0.0 || c.im != 0.0,
        }
    }
    fn to_i8(self) -> i8 {
        match self {
            Value::Int(i) => i as i8,
            Value::Float(f) => f as i8,
            Value::Complex(c) => c.re as i8,
        }
    }
    fn to_u8(self) -> u8 {
        match self {
            Value::Int(i) => i as u8,
            Value::Float(f) => f as u8,
            Value::Complex(c) => c.re as u8,
        }
    }
    fn to_i16(self) -> i16 {
        match self {
            Value::Int(i) => i as i16,
            Value::Float(f) => f as i16,
            Value::Complex(c) => c.re as i16,
        }
    }
    fn to_u16(self) -> u16 {
        match self {
            Value::Int(i) => i as u16,
            Value::Float(f) => f as u16,
            Value::Complex(c) => c.re as u16,
        }
    }
    fn to_i32(self) -> i32 {
        match self {
            Value::Int(i) => i as i32,
            Value::Float(f) => f as i32,
            Value::Complex(c) => c.re as i32,
        }
    }
    fn to_u32(self) -> u32 {
        match self {
            Value::Int(i) => i as u32,
            Value::Float(f) => f as u32,
            Value::Complex(c) => c.re as u32,
        }
    }
    fn to_i64(self) -> i64 {
        match self {
            Value::Int(i) => i as i64,
            Value::Float(f) => f as i64,
            Value::Complex(c) => c.re as i64,
        }
    }
    fn to_u64(self) -> u64 {
        match self {
            Value::Int(i) => i as u64,
            Value::Float(f) => f as u64,
            Value::Complex(c) => c.re as u64,
        }
    }
    fn to_f32(self) -> f32 {
        match self {
            Value::Int(i) => i as f32,
            Value::Float(f) => f as f32,
            Value::Complex(c) => c.re as f32,
        }
    }
    fn to_f64(self) -> f64 {
        match self {
            Value::Int(i) => i as f64,
            Value::Float(f) => f,
            Value::Complex(c) => c.re,
        }
    }
    fn to_c32(self) -> Complex<f32> {
        match self {
            Value::Int(i) => Complex::new(i as f32, 0.0),
            Value::Float(f) => Complex::new(f as f32, 0.0),
            Value::Complex(c) => Complex::new(c.re as f32, c.im as f32),
        }
    }
    fn to_c64(self) -> Complex<f64> {
        match self {
            Value::Int(i) => Complex::new(i as f64, 0.0),
            Value::Float(f) => Complex::new(f, 0.0),
            Value::Complex(c) => c,
        }
    }
}

fn test_simple_assign<T: Copy + PartialEq + Default>(val: T) {
    let mut a: T = Default::default();
    let b: T = val;
    if a != Default::default() {
        abort();
    }
    if b != val {
        abort();
    }
    a = b;
    if a != val {
        abort();
    }
    if a != val {
        abort();
    }
}

fn test_simple_assign_arith(val: Value) {
    test_simple_assign::<bool>(val.to_bool());
    test_simple_assign::<i8>(val.to_i8()); // char
    test_simple_assign::<i8>(val.to_i8()); // signed char
    test_simple_assign::<u8>(val.to_u8()); // unsigned char
    test_simple_assign::<i16>(val.to_i16());
    test_simple_assign::<u16>(val.to_u16());
    test_simple_assign::<i32>(val.to_i32());
    test_simple_assign::<u32>(val.to_u32());
    test_simple_assign::<i64>(val.to_i64()); // long
    test_simple_assign::<u64>(val.to_u64()); // unsigned long
    test_simple_assign::<i64>(val.to_i64()); // long long
    test_simple_assign::<u64>(val.to_u64()); // unsigned long long
    test_simple_assign::<f32>(val.to_f32());
    test_simple_assign::<f64>(val.to_f64());
    test_simple_assign::<f64>(val.to_f64()); // long double
    test_simple_assign::<Complex<f32>>(val.to_c32());
    test_simple_assign::<Complex<f64>>(val.to_c64());
    test_simple_assign::<Complex<f64>>(val.to_c64()); // long double complex
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct S {
    a: [i16; 1024],
}

fn test_simple_assign_all() {
    test_simple_assign_arith(Value::Int(0));
    test_simple_assign_arith(Value::Int(1));
    test_simple_assign_arith(Value::Int(2));
    test_simple_assign_arith(Value::Int(-1));
    test_simple_assign_arith(Value::Int(1i128 << 63));
    test_simple_assign_arith(Value::Float(1.5));
    test_simple_assign_arith(Value::Complex(Complex::new(2.5, 3.5)));

    let i: i32 = 0;
    test_simple_assign::<*const i32>(ptr::null());
    test_simple_assign::<*const i32>(&i as *const i32);

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
    test_simple_assign_all();
    exit(0);
}