use std::process;

#[derive(Copy, Clone, PartialEq)]
struct Complex32 {
    re: f32,
    im: f32,
}

#[derive(Copy, Clone, PartialEq)]
struct Complex64 {
    re: f64,
    im: f64,
}

enum Val {
    Int(i128),
    UInt(u128),
    Float(f64),
    Complex(Complex64),
}

fn abort() -> ! {
    process::abort();
}

fn test_simple_assign<T: Copy + PartialEq>(value: T, zero: T) {
    let mut a = zero;
    let mut b = value;
    if a != zero {
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

fn to_bool(v: &Val) -> bool {
    match v {
        Val::Int(i) => *i != 0,
        Val::UInt(u) => *u != 0,
        Val::Float(f) => *f != 0.0,
        Val::Complex(c) => c.re != 0.0,
    }
}

fn to_i8(v: &Val) -> i8 {
    match v {
        Val::Int(i) => *i as i8,
        Val::UInt(u) => *u as i8,
        Val::Float(f) => *f as i8,
        Val::Complex(c) => c.re as i8,
    }
}

fn to_u8(v: &Val) -> u8 {
    match v {
        Val::Int(i) => *i as u8,
        Val::UInt(u) => *u as u8,
        Val::Float(f) => *f as u8,
        Val::Complex(c) => c.re as u8,
    }
}

fn to_i16(v: &Val) -> i16 {
    match v {
        Val::Int(i) => *i as i16,
        Val::UInt(u) => *u as i16,
        Val::Float(f) => *f as i16,
        Val::Complex(c) => c.re as i16,
    }
}

fn to_u16(v: &Val) -> u16 {
    match v {
        Val::Int(i) => *i as u16,
        Val::UInt(u) => *u as u16,
        Val::Float(f) => *f as u16,
        Val::Complex(c) => c.re as u16,
    }
}

fn to_i32(v: &Val) -> i32 {
    match v {
        Val::Int(i) => *i as i32,
        Val::UInt(u) => *u as i32,
        Val::Float(f) => *f as i32,
        Val::Complex(c) => c.re as i32,
    }
}

fn to_u32(v: &Val) -> u32 {
    match v {
        Val::Int(i) => *i as u32,
        Val::UInt(u) => *u as u32,
        Val::Float(f) => *f as u32,
        Val::Complex(c) => c.re as u32,
    }
}

fn to_i64(v: &Val) -> i64 {
    match v {
        Val::Int(i) => *i as i64,
        Val::UInt(u) => *u as i64,
        Val::Float(f) => *f as i64,
        Val::Complex(c) => c.re as i64,
    }
}

fn to_u64(v: &Val) -> u64 {
    match v {
        Val::Int(i) => *i as u64,
        Val::UInt(u) => *u as u64,
        Val::Float(f) => *f as u64,
        Val::Complex(c) => c.re as u64,
    }
}

fn to_f32_scalar(v: &Val) -> f32 {
    match v {
        Val::Int(i) => *i as f32,
        Val::UInt(u) => *u as f32,
        Val::Float(f) => *f as f32,
        Val::Complex(c) => c.re as f32,
    }
}

fn to_f64_scalar(v: &Val) -> f64 {
    match v {
        Val::Int(i) => *i as f64,
        Val::UInt(u) => *u as f64,
        Val::Float(f) => *f,
        Val::Complex(c) => c.re,
    }
}

fn to_c32(v: &Val) -> Complex32 {
    match v {
        Val::Complex(c) => Complex32 {
            re: c.re as f32,
            im: c.im as f32,
        },
        _ => Complex32 {
            re: to_f32_scalar(v),
            im: 0.0,
        },
    }
}

fn to_c64(v: &Val) -> Complex64 {
    match v {
        Val::Complex(c) => *c,
        _ => Complex64 {
            re: to_f64_scalar(v),
            im: 0.0,
        },
    }
}

fn test_simple_assign_arith(v: &Val) {
    test_simple_assign(to_bool(v), false);
    test_simple_assign(to_i8(v), 0);
    test_simple_assign(to_i8(v), 0);
    test_simple_assign(to_u8(v), 0);
    test_simple_assign(to_i16(v), 0);
    test_simple_assign(to_u16(v), 0);
    test_simple_assign(to_i32(v), 0);
    test_simple_assign(to_u32(v), 0);
    test_simple_assign(to_i64(v), 0);
    test_simple_assign(to_u64(v), 0);
    test_simple_assign(to_i64(v), 0);
    test_simple_assign(to_u64(v), 0);
    test_simple_assign(to_f32_scalar(v), 0.0);
    test_simple_assign(to_f64_scalar(v), 0.0);
    test_simple_assign(to_f64_scalar(v), 0.0);
    test_simple_assign(to_c32(v), Complex32 { re: 0.0, im: 0.0 });
    test_simple_assign(to_c64(v), Complex64 { re: 0.0, im: 0.0 });
    test_simple_assign(to_c64(v), Complex64 { re: 0.0, im: 0.0 });
}

#[derive(Copy, Clone, PartialEq)]
struct S {
    a: [i16; 1024],
}

fn test_simple_assign() {
    let values = [
        Val::Int(0),
        Val::Int(1),
        Val::Int(2),
        Val::Int(-1),
        Val::UInt(1u128 << 63),
        Val::Float(1.5),
        Val::Complex(Complex64 { re: 2.5, im: 3.5 }),
    ];
    for v in &values {
        test_simple_assign_arith(v);
    }

    static I: i32 = 0;
    let ptr_val = &I as *const i32 as usize;
    test_simple_assign(0usize, 0usize);
    test_simple_assign(ptr_val, 0usize);

    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    let mut s1 = S { a: [0; 1024] };
    let mut s2 = S { a: [0; 1024] };
    let copy = {
        s1 = init;
        s1
    };
    if init != copy {
        abort();
    }
    let copy = {
        s2 = s1;
        s2
    };
    if init != copy {
        abort();
    }
    let copy = s1;
    if init != copy {
        abort();
    }
    let copy = s2;
    if init != copy {
        abort();
    }
}

fn main() {
    test_simple_assign();
    process::exit(0);
}