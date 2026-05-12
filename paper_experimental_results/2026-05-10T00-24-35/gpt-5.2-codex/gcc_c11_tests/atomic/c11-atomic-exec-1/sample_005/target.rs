use std::process::abort;

#[derive(Copy, Clone, PartialEq)]
struct Complex32 {
    re: f32,
    im: f32,
}

impl Complex32 {
    fn zero() -> Self {
        Complex32 { re: 0.0, im: 0.0 }
    }
}

#[derive(Copy, Clone, PartialEq)]
struct Complex64 {
    re: f64,
    im: f64,
}

impl Complex64 {
    fn zero() -> Self {
        Complex64 { re: 0.0, im: 0.0 }
    }
}

#[derive(Copy, Clone)]
enum Val {
    Int(i128),
    U64(u64),
    Real(f64),
    Complex(f64, f64),
}

fn val_to_f64_real(val: &Val) -> f64 {
    match *val {
        Val::Int(i) => i as f64,
        Val::U64(u) => u as f64,
        Val::Real(f) => f,
        Val::Complex(re, _) => re,
    }
}

fn val_to_bool(val: &Val) -> bool {
    val_to_f64_real(val) != 0.0
}

fn val_to_i8(val: &Val) -> i8 {
    match *val {
        Val::Int(i) => i as i8,
        Val::U64(u) => u as i8,
        Val::Real(f) => f.trunc() as i8,
        Val::Complex(re, _) => re.trunc() as i8,
    }
}

fn val_to_u8(val: &Val) -> u8 {
    match *val {
        Val::Int(i) => i as u8,
        Val::U64(u) => u as u8,
        Val::Real(f) => f.trunc() as u8,
        Val::Complex(re, _) => re.trunc() as u8,
    }
}

fn val_to_i16(val: &Val) -> i16 {
    match *val {
        Val::Int(i) => i as i16,
        Val::U64(u) => u as i16,
        Val::Real(f) => f.trunc() as i16,
        Val::Complex(re, _) => re.trunc() as i16,
    }
}

fn val_to_u16(val: &Val) -> u16 {
    match *val {
        Val::Int(i) => i as u16,
        Val::U64(u) => u as u16,
        Val::Real(f) => f.trunc() as u16,
        Val::Complex(re, _) => re.trunc() as u16,
    }
}

fn val_to_i32(val: &Val) -> i32 {
    match *val {
        Val::Int(i) => i as i32,
        Val::U64(u) => u as i32,
        Val::Real(f) => f.trunc() as i32,
        Val::Complex(re, _) => re.trunc() as i32,
    }
}

fn val_to_u32(val: &Val) -> u32 {
    match *val {
        Val::Int(i) => i as u32,
        Val::U64(u) => u as u32,
        Val::Real(f) => f.trunc() as u32,
        Val::Complex(re, _) => re.trunc() as u32,
    }
}

fn val_to_i64(val: &Val) -> i64 {
    match *val {
        Val::Int(i) => i as i64,
        Val::U64(u) => u as i64,
        Val::Real(f) => f.trunc() as i64,
        Val::Complex(re, _) => re.trunc() as i64,
    }
}

fn val_to_u64(val: &Val) -> u64 {
    match *val {
        Val::Int(i) => i as u64,
        Val::U64(u) => u,
        Val::Real(f) => f.trunc() as u64,
        Val::Complex(re, _) => re.trunc() as u64,
    }
}

fn val_to_f32(val: &Val) -> f32 {
    val_to_f64_real(val) as f32
}

fn val_to_f64(val: &Val) -> f64 {
    val_to_f64_real(val)
}

fn val_to_complex32(val: &Val) -> Complex32 {
    match *val {
        Val::Complex(re, im) => Complex32 {
            re: re as f32,
            im: im as f32,
        },
        _ => Complex32 {
            re: val_to_f64_real(val) as f32,
            im: 0.0,
        },
    }
}

fn val_to_complex64(val: &Val) -> Complex64 {
    match *val {
        Val::Complex(re, im) => Complex64 { re, im },
        _ => Complex64 {
            re: val_to_f64_real(val),
            im: 0.0,
        },
    }
}

fn test_simple_assign_value<T: Copy + PartialEq>(zero: T, val: T) {
    let mut a = zero;
    let mut b = val;
    if a != zero {
        abort();
    }
    if b != val {
        abort();
    }
    let assigned = {
        a = b;
        a
    };
    if assigned != val {
        abort();
    }
    if a != val {
        abort();
    }
}

fn test_simple_assign_arith(val: &Val) {
    test_simple_assign_value(false, val_to_bool(val));

    test_simple_assign_value(0i8, val_to_i8(val)); // char
    test_simple_assign_value(0i8, val_to_i8(val)); // signed char
    test_simple_assign_value(0u8, val_to_u8(val));

    test_simple_assign_value(0i16, val_to_i16(val));
    test_simple_assign_value(0u16, val_to_u16(val));

    test_simple_assign_value(0i32, val_to_i32(val));
    test_simple_assign_value(0u32, val_to_u32(val));

    test_simple_assign_value(0i64, val_to_i64(val));
    test_simple_assign_value(0u64, val_to_u64(val));

    test_simple_assign_value(0i64, val_to_i64(val));
    test_simple_assign_value(0u64, val_to_u64(val));

    test_simple_assign_value(0.0f32, val_to_f32(val));
    test_simple_assign_value(0.0f64, val_to_f64(val));
    test_simple_assign_value(0.0f64, val_to_f64(val));

    test_simple_assign_value(Complex32::zero(), val_to_complex32(val));
    test_simple_assign_value(Complex64::zero(), val_to_complex64(val));
    test_simple_assign_value(Complex64::zero(), val_to_complex64(val));
}

static I: i32 = 0;

fn ptr_option_eq(a: &Option<&'static i32>, b: Option<&'static i32>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => std::ptr::eq(*x, y),
        _ => false,
    }
}

fn test_simple_assign_ptr() {
    let mut a: Option<&'static i32> = None;
    let mut b: Option<&'static i32> = Some(&I);
    if !a.is_none() {
        abort();
    }
    if !ptr_option_eq(&b, Some(&I)) {
        abort();
    }
    let assigned = {
        a = b;
        a
    };
    if !ptr_option_eq(&assigned, Some(&I)) {
        abort();
    }
    if !ptr_option_eq(&a, Some(&I)) {
        abort();
    }
}

#[derive(Copy, Clone, PartialEq)]
struct S {
    a: [i16; 1024],
}

fn test_simple_assign_struct() {
    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    let mut copy: S;
    let mut s1 = S { a: [0; 1024] };
    let mut s2 = S { a: [0; 1024] };

    copy = {
        s1 = init;
        s1
    };
    if copy != init {
        abort();
    }

    copy = {
        s2 = s1;
        s2
    };
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

fn test_simple_assign() {
    let values = [
        Val::Int(0),
        Val::Int(1),
        Val::Int(2),
        Val::Int(-1),
        Val::U64(1u64 << 63),
        Val::Real(1.5),
        Val::Complex(2.5, 3.5),
    ];

    for v in &values {
        test_simple_assign_arith(v);
    }

    test_simple_assign_ptr();
    test_simple_assign_struct();
}

fn main() {
    test_simple_assign();
}