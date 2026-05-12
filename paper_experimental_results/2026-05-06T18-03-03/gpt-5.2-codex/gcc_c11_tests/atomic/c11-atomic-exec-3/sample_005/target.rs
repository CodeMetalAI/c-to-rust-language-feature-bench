use std::process::{abort, exit};

#[derive(Copy, Clone)]
enum Value {
    Signed(i128),
    Unsigned(u128),
    Float(f64),
}

fn check(cond: bool) {
    if !cond {
        abort();
    }
}

fn to_i128(v: &Value) -> i128 {
    match *v {
        Value::Signed(x) => x,
        Value::Unsigned(x) => x as i128,
        Value::Float(f) => f.trunc() as i128,
    }
}

fn to_u128(v: &Value) -> u128 {
    match *v {
        Value::Signed(x) => x as u128,
        Value::Unsigned(x) => x,
        Value::Float(f) => (f.trunc() as i128) as u128,
    }
}

fn cast_bool(v: &Value) -> bool {
    match *v {
        Value::Signed(x) => x != 0,
        Value::Unsigned(x) => x != 0,
        Value::Float(f) => f != 0.0,
    }
}

fn cast_i8(v: &Value) -> i8 {
    to_i128(v) as i8
}
fn cast_u8(v: &Value) -> u8 {
    to_u128(v) as u8
}
fn cast_i16(v: &Value) -> i16 {
    to_i128(v) as i16
}
fn cast_u16(v: &Value) -> u16 {
    to_u128(v) as u16
}
fn cast_i32(v: &Value) -> i32 {
    to_i128(v) as i32
}
fn cast_u32(v: &Value) -> u32 {
    to_u128(v) as u32
}
fn cast_i64(v: &Value) -> i64 {
    to_i128(v) as i64
}
fn cast_u64(v: &Value) -> u64 {
    to_u128(v) as u64
}
fn cast_f32(v: &Value) -> f32 {
    match *v {
        Value::Signed(x) => x as f32,
        Value::Unsigned(x) => x as f32,
        Value::Float(f) => f as f32,
    }
}
fn cast_f64(v: &Value) -> f64 {
    match *v {
        Value::Signed(x) => x as f64,
        Value::Unsigned(x) => x as f64,
        Value::Float(f) => f,
    }
}

fn add_bool(v: bool, change: i32) -> bool {
    let base = if v { 1 } else { 0 };
    (base + change) != 0
}
fn add_i8(v: i8, change: i32) -> i8 {
    (v as i128 + change as i128) as i8
}
fn add_u8(v: u8, change: i32) -> u8 {
    (v as i128 + change as i128) as u8
}
fn add_i16(v: i16, change: i32) -> i16 {
    (v as i128 + change as i128) as i16
}
fn add_u16(v: u16, change: i32) -> u16 {
    (v as i128 + change as i128) as u16
}
fn add_i32(v: i32, change: i32) -> i32 {
    (v as i128 + change as i128) as i32
}
fn add_u32(v: u32, change: i32) -> u32 {
    (v as i128 + change as i128) as u32
}
fn add_i64(v: i64, change: i32) -> i64 {
    (v as i128 + change as i128) as i64
}
fn add_u64(v: u64, change: i32) -> u64 {
    (v as i128 + change as i128) as u64
}
fn add_f32(v: f32, change: i32) -> f32 {
    v + change as f32
}
fn add_f64(v: f64, change: i32) -> f64 {
    v + change as f64
}

fn test_incdec<T: Copy + PartialEq>(
    value: &Value,
    pre: bool,
    change: i32,
    cast: fn(&Value) -> T,
    add: fn(T, i32) -> T,
) {
    let mut a = cast(value);
    let res = if pre {
        a = add(a, change);
        a
    } else {
        let old = a;
        a = add(a, change);
        old
    };
    let expected_expr = if pre {
        add(cast(value), change)
    } else {
        cast(value)
    };
    check(res == expected_expr);
    check(a == add(cast(value), change));
}

#[derive(Copy, Clone, PartialEq)]
struct Ptr(isize);

fn add_ptr(p: Ptr, change: i32) -> Ptr {
    Ptr(p.0 + change as isize)
}

fn test_incdec_ptr(pre: bool, change: i32) {
    let value = Ptr(1);
    let mut a = value;
    let res = if pre {
        a = add_ptr(a, change);
        a
    } else {
        let old = a;
        a = add_ptr(a, change);
        old
    };
    let expected_expr = if pre { add_ptr(value, change) } else { value };
    check(res == expected_expr);
    check(a == add_ptr(value, change));
}

fn test_incdec_arith(value: &Value, pre: bool, change: i32) {
    test_incdec(value, pre, change, cast_bool, add_bool);
    test_incdec(value, pre, change, cast_i8, add_i8); // char
    test_incdec(value, pre, change, cast_i8, add_i8); // signed char
    test_incdec(value, pre, change, cast_u8, add_u8); // unsigned char
    test_incdec(value, pre, change, cast_i16, add_i16); // signed short
    test_incdec(value, pre, change, cast_u16, add_u16); // unsigned short
    test_incdec(value, pre, change, cast_i32, add_i32); // signed int
    test_incdec(value, pre, change, cast_u32, add_u32); // unsigned int
    test_incdec(value, pre, change, cast_i64, add_i64); // signed long
    test_incdec(value, pre, change, cast_u64, add_u64); // unsigned long
    test_incdec(value, pre, change, cast_i64, add_i64); // signed long long
    test_incdec(value, pre, change, cast_u64, add_u64); // unsigned long long
    test_incdec(value, pre, change, cast_f32, add_f32); // float
    test_incdec(value, pre, change, cast_f64, add_f64); // double
    test_incdec(value, pre, change, cast_f64, add_f64); // long double
}

fn test_all_incdec_arith(value: &Value) {
    test_incdec_arith(value, true, 1);
    test_incdec_arith(value, true, -1);
    test_incdec_arith(value, false, 1);
    test_incdec_arith(value, false, -1);
}

fn test_incdec() {
    let v0 = Value::Signed(0);
    let v1 = Value::Signed(1);
    let v2 = Value::Signed(2);
    let vneg1 = Value::Signed(-1);
    let vu60 = Value::Unsigned(1u128 << 60);
    let v1_5 = Value::Float(1.5);

    test_all_incdec_arith(&v0);
    test_all_incdec_arith(&v1);
    test_all_incdec_arith(&v2);
    test_all_incdec_arith(&vneg1);
    test_all_incdec_arith(&vu60);
    test_all_incdec_arith(&v1_5);

    test_incdec_ptr(true, 1);
    test_incdec_ptr(true, -1);
    test_incdec_ptr(false, 1);
    test_incdec_ptr(false, -1);
}

fn main() {
    test_incdec();
    exit(0);
}