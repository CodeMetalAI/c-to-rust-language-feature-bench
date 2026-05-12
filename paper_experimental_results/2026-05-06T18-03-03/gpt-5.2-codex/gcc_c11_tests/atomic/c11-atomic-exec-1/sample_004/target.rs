use std::process;

#[derive(Copy, Clone)]
struct Complex32 {
    re: f32,
    im: f32,
}

impl PartialEq for Complex32 {
    fn eq(&self, other: &Self) -> bool {
        self.re == other.re && self.im == other.im
    }
}

#[derive(Copy, Clone)]
struct Complex64 {
    re: f64,
    im: f64,
}

impl PartialEq for Complex64 {
    fn eq(&self, other: &Self) -> bool {
        self.re == other.re && self.im == other.im
    }
}

#[derive(Copy, Clone)]
enum SourceValue {
    Int(i32),
    Unsigned(u64),
    Float(f64),
    Complex(f64, f64),
}

impl SourceValue {
    fn to_bool(&self) -> bool {
        match *self {
            SourceValue::Complex(re, im) => re != 0.0 || im != 0.0,
            SourceValue::Int(v) => v != 0,
            SourceValue::Unsigned(u) => u != 0,
            SourceValue::Float(f) => f != 0.0,
        }
    }
    fn to_i8(&self) -> i8 {
        match *self {
            SourceValue::Int(v) => v as i8,
            SourceValue::Unsigned(u) => u as i8,
            SourceValue::Float(f) => f as i8,
            SourceValue::Complex(re, _) => re as i8,
        }
    }
    fn to_u8(&self) -> u8 {
        match *self {
            SourceValue::Int(v) => v as u8,
            SourceValue::Unsigned(u) => u as u8,
            SourceValue::Float(f) => f as u8,
            SourceValue::Complex(re, _) => re as u8,
        }
    }
    fn to_i16(&self) -> i16 {
        match *self {
            SourceValue::Int(v) => v as i16,
            SourceValue::Unsigned(u) => u as i16,
            SourceValue::Float(f) => f as i16,
            SourceValue::Complex(re, _) => re as i16,
        }
    }
    fn to_u16(&self) -> u16 {
        match *self {
            SourceValue::Int(v) => v as u16,
            SourceValue::Unsigned(u) => u as u16,
            SourceValue::Float(f) => f as u16,
            SourceValue::Complex(re, _) => re as u16,
        }
    }
    fn to_i32(&self) -> i32 {
        match *self {
            SourceValue::Int(v) => v,
            SourceValue::Unsigned(u) => u as i32,
            SourceValue::Float(f) => f as i32,
            SourceValue::Complex(re, _) => re as i32,
        }
    }
    fn to_u32(&self) -> u32 {
        match *self {
            SourceValue::Int(v) => v as u32,
            SourceValue::Unsigned(u) => u as u32,
            SourceValue::Float(f) => f as u32,
            SourceValue::Complex(re, _) => re as u32,
        }
    }
    fn to_i64(&self) -> i64 {
        match *self {
            SourceValue::Int(v) => v as i64,
            SourceValue::Unsigned(u) => u as i64,
            SourceValue::Float(f) => f as i64,
            SourceValue::Complex(re, _) => re as i64,
        }
    }
    fn to_u64(&self) -> u64 {
        match *self {
            SourceValue::Int(v) => v as u64,
            SourceValue::Unsigned(u) => u,
            SourceValue::Float(f) => f as u64,
            SourceValue::Complex(re, _) => re as u64,
        }
    }
    fn to_f32(&self) -> f32 {
        match *self {
            SourceValue::Int(v) => v as f32,
            SourceValue::Unsigned(u) => u as f32,
            SourceValue::Float(f) => f as f32,
            SourceValue::Complex(re, _) => re as f32,
        }
    }
    fn to_f64(&self) -> f64 {
        match *self {
            SourceValue::Int(v) => v as f64,
            SourceValue::Unsigned(u) => u as f64,
            SourceValue::Float(f) => f,
            SourceValue::Complex(re, _) => re,
        }
    }
    fn to_complex32(&self) -> Complex32 {
        match *self {
            SourceValue::Complex(re, im) => Complex32 { re: re as f32, im: im as f32 },
            _ => Complex32 { re: self.to_f32(), im: 0.0 },
        }
    }
    fn to_complex64(&self) -> Complex64 {
        match *self {
            SourceValue::Complex(re, im) => Complex64 { re, im },
            _ => Complex64 { re: self.to_f64(), im: 0.0 },
        }
    }
}

fn abort() -> ! {
    process::abort()
}

fn test_simple_assign<T: PartialEq + Copy>(value: T, zero: T) {
    let mut a: T = zero;
    let b: T = value;
    if a != zero {
        abort();
    }
    if b != value {
        abort();
    }
    let c = {
        a = b;
        a
    };
    if c != value {
        abort();
    }
    if a != value {
        abort();
    }
}

fn test_simple_assign_arith(src: SourceValue) {
    test_simple_assign::<bool>(src.to_bool(), false);
    test_simple_assign::<i8>(src.to_i8(), 0);
    test_simple_assign::<i8>(src.to_i8(), 0); // char treated as signed
    test_simple_assign::<i8>(src.to_i8(), 0);
    test_simple_assign::<u8>(src.to_u8(), 0);
    test_simple_assign::<i16>(src.to_i16(), 0);
    test_simple_assign::<u16>(src.to_u16(), 0);
    test_simple_assign::<i32>(src.to_i32(), 0);
    test_simple_assign::<u32>(src.to_u32(), 0);
    test_simple_assign::<i64>(src.to_i64(), 0);
    test_simple_assign::<u64>(src.to_u64(), 0);
    test_simple_assign::<i64>(src.to_i64(), 0);
    test_simple_assign::<u64>(src.to_u64(), 0);
    test_simple_assign::<f32>(src.to_f32(), 0.0);
    test_simple_assign::<f64>(src.to_f64(), 0.0);
    test_simple_assign::<f64>(src.to_f64(), 0.0);
    test_simple_assign::<Complex32>(src.to_complex32(), Complex32 { re: 0.0, im: 0.0 });
    test_simple_assign::<Complex64>(src.to_complex64(), Complex64 { re: 0.0, im: 0.0 });
    test_simple_assign::<Complex64>(src.to_complex64(), Complex64 { re: 0.0, im: 0.0 });
}

#[derive(Copy, Clone, PartialEq)]
struct S {
    a: [i16; 1024],
}

static I: i32 = 0;

fn test_simple_assign() {
    test_simple_assign_arith(SourceValue::Int(0));
    test_simple_assign_arith(SourceValue::Int(1));
    test_simple_assign_arith(SourceValue::Int(2));
    test_simple_assign_arith(SourceValue::Int(-1));
    test_simple_assign_arith(SourceValue::Unsigned(1u64 << 63));
    test_simple_assign_arith(SourceValue::Float(1.5));
    test_simple_assign_arith(SourceValue::Complex(2.5, 3.5));

    test_simple_assign::<*const i32>(std::ptr::null(), std::ptr::null());
    test_simple_assign::<*const i32>(&I as *const i32, std::ptr::null());

    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    let mut copy: S;
    let mut s1 = S { a: [0; 1024] };
    let mut s2 = S { a: [0; 1024] };

    copy = { s1 = init; s1 };
    if copy != init {
        abort();
    }
    copy = { s2 = s1; s2 };
    if copy != init {
        abort();
    }
    copy = s1;
    if copy != init {
        abort();
    }
    copy = s2;
    if copy != init {
        abort();
    }
}

fn main() {
    test_simple_assign();
    process::exit(0);
}