use std::process;

fn abort() {
    process::abort();
}

fn exit(code: i32) {
    process::exit(code);
}

static I: i32 = 0;

fn test_bool(val: i64) {
    let v = val != 0;
    let mut a = false;
    let b = v;
    if a != false {
        abort();
    }
    if b != v {
        abort();
    }
    a = b;
    if a != v {
        abort();
    }
}

fn test_char(val: i64) {
    let v = val as i8;
    let mut a: i8 = 0;
    let b = v;
    if a != 0 {
        abort();
    }
    if b != v {
        abort();
    }
    a = b;
    if a != v {
        abort();
    }
}

fn test_signed_char(val: i64) {
    let v = val as i8;
    let mut a: i8 = 0;
    let b = v;
    if a != 0 {
        abort();
    }
    if b != v {
        abort();
    }
    a = b;
    if a != v {
        abort();
    }
}

fn test_unsigned_char(val: i64) {
    let v = val as u8;
    let mut a: u8 = 0;
    let b = v;
    if a != 0 {
        abort();
    }
    if b != v {
        abort();
    }
    a = b;
    if a != v {
        abort();
    }
}

fn test_signed_short(val: i64) {
    let v = val as i16;
    let mut a: i16 = 0;
    let b = v;
    if a != 0 {
        abort();
    }
    if b != v {
        abort();
    }
    a = b;
    if a != v {
        abort();
    }
}

fn test_unsigned_short(val: i64) {
    let v = val as u16;
    let mut a: u16 = 0;
    let b = v;
    if a != 0 {
        abort();
    }
    if b != v {
        abort();
    }
    a = b;
    if a != v {
        abort();
    }
}

fn test_signed_int(val: i64) {
    let v = val as i32;
    let mut a: i32 = 0;
    let b = v;
    if a != 0 {
        abort();
    }
    if b != v {
        abort();
    }
    a = b;
    if a != v {
        abort();
    }
}

fn test_unsigned_int(val: i64) {
    let v = val as u32;
    let mut a: u32 = 0;
    let b = v;
    if a != 0 {
        abort();
    }
    if b != v {
        abort();
    }
    a = b;
    if a != v {
        abort();
    }
}

fn test_signed_long(val: i64) {
    let v = val;
    let mut a: i64 = 0;
    let b = v;
    if a != 0 {
        abort();
    }
    if b != v {
        abort();
    }
    a = b;
    if a != v {
        abort();
    }
}

fn test_unsigned_long(val: i64) {
    let v = val as u64;
    let mut a: u64 = 0;
    let b = v;
    if a != 0 {
        abort();
    }
    if b != v {
        abort();
    }
    a = b;
    if a != v {
        abort();
    }
}

fn test_signed_long_long(val: i64) {
    let v = val;
    let mut a: i64 = 0;
    let b = v;
    if a != 0 {
        abort();
    }
    if b != v {
        abort();
    }
    a = b;
    if a != v {
        abort();
    }
}

fn test_unsigned_long_long(val: i64) {
    let v = val as u64;
    let mut a: u64 = 0;
    let b = v;
    if a != 0 {
        abort();
    }
    if b != v {
        abort();
    }
    a = b;
    if a != v {
        abort();
    }
}

fn test_float(val: f64) {
    let v = val as f32;
    let mut a: f32 = 0.0;
    let b = v;
    if a != 0.0 {
        abort();
    }
    if b != v {
        abort();
    }
    a = b;
    if a != v {
        abort();
    }
}

fn test_double(val: f64) {
    let v = val;
    let mut a: f64 = 0.0;
    let b = v;
    if a != 0.0 {
        abort();
    }
    if b != v {
        abort();
    }
    a = b;
    if a != v {
        abort();
    }
}

fn test_long_double(val: f64) {
    let v = val;
    let mut a: f64 = 0.0;
    let b = v;
    if a != 0.0 {
        abort();
    }
    if b != v {
        abort();
    }
    a = b;
    if a != v {
        abort();
    }
}

fn test_complex_float(val: [f32; 2]) {
    let v = val;
    let mut a: [f32; 2] = [0.0, 0.0];
    let b = v;
    if a != [0.0, 0.0] {
        abort();
    }
    if b != v {
        abort();
    }
    a = b;
    if a != v {
        abort();
    }
}

fn test_complex_double(val: [f64; 2]) {
    let v = val;
    let mut a: [f64; 2] = [0.0, 0.0];
    let b = v;
    if a != [0.0, 0.0] {
        abort();
    }
    if b != v {
        abort();
    }
    a = b;
    if a != v {
        abort();
    }
}

fn test_complex_long_double(val: [f64; 2]) {
    let v = val;
    let mut a: [f64; 2] = [0.0, 0.0];
    let b = v;
    if a != [0.0, 0.0] {
        abort();
    }
    if b != v {
        abort();
    }
    a = b;
    if a != v {
        abort();
    }
}

fn test_pointer(val: Option<&'static i32>) {
    let v = val;
    let mut a: Option<&'static i32> = None;
    let b = v;
    if a.is_some() {
        abort();
    }
    if b != v {
        abort();
    }
    a = b;
    if a != v {
        abort();
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct S {
    a: [i16; 1024],
}

fn test_struct() {
    let mut s1: S = S { a: [0; 1024] };
    let mut s2: S = S { a: [0; 1024] };
    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    let mut copy = {
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

fn test_simple_assign() {
    test_bool(0);
    test_char(0);
    test_signed_char(0);
    test_unsigned_char(0);
    test_signed_short(0);
    test_unsigned_short(0);
    test_signed_int(0);
    test_unsigned_int(0);
    test_signed_long(0);
    test_unsigned_long(0);
    test_signed_long_long(0);
    test_unsigned_long_long(0);
    test_float(0.0);
    test_double(0.0);
    test_long_double(0.0);
    test_complex_float([0.0, 0.0]);
    test_complex_double([0.0, 0.0]);
    test_complex_long_double([0.0, 0.0]);
    test_pointer(None);

    test_bool(1);
    test_char(1);
    test_signed_char(1);
    test_unsigned_char(1);
    test_signed_short(1);
    test_unsigned_short(1);
    test_signed_int(1);
    test_unsigned_int(1);
    test_signed_long(1);
    test_unsigned_long(1);
    test_signed_long_long(1);
    test_unsigned_long_long(1);
    test_float(1.0);
    test_double(1.0);
    test_long_double(1.0);
    test_complex_float([1.0, 0.0]);
    test_complex_double([1.0, 0.0]);
    test_complex_long_double([1.0, 0.0]);

    test_bool(2);
    test_char(2);
    test_signed_char(2);
    test_unsigned_char(2);
    test_signed_short(2);
    test_unsigned_short(2);
    test_signed_int(2);
    test_unsigned_int(2);
    test_signed_long(2);
    test_unsigned_long(2);
    test_signed_long_long(2);
    test_unsigned_long_long(2);
    test_float(2.0);
    test_double(2.0);
    test_long_double(2.0);
    test_complex_float([2.0, 0.0]);
    test_complex_double([2.0, 0.0]);
    test_complex_long_double([2.0, 0.0]);

    test_bool(-1);
    test_char(-1);
    test_signed_char(-1);
    test_unsigned_char(-1);
    test_signed_short(-1);
    test_unsigned_short(-1);
    test_signed_int(-1);
    test_unsigned_int(-1);
    test_signed_long(-1);
    test_unsigned_long(-1);
    test_signed_long_long(-1);
    test_unsigned_long_long(-1);
    test_float(-1.0);
    test_double(-1.0);
    test_long_double(-1.0);
    test_complex_float([-1.0, 0.0]);
    test_complex_double([-1.0, 0.0]);
    test_complex_long_double([-1.0, 0.0]);

    let big = 1i64 << 63;
    test_bool(big as i64);
    test_char(big);
    test_signed_char(big);
    test_unsigned_char(big as u64 as i64);
    test_signed_short(big);
    test_unsigned_short(big as u64 as i64);
    test_signed_int(big);
    test_unsigned_int(big as u64 as i64);
    test_signed_long(big);
    test_unsigned_long(big as u64 as i64);
    test_signed_long_long(big);
    test_unsigned_long_long(big as u64 as i64);
    test_float(big as f64);
    test_double(big as f64);
    test_long_double(big as f64);
    test_complex_float([big as f64 as f32, 0.0]);
    test_complex_double([big as f64, 0.0]);
    test_complex_long_double([big as f64, 0.0]);

    test_float(1.5);
    test_double(1.5);
    test_long_double(1.5);
    test_complex_float([1.5, 0.0]);
    test_complex_double([1.5, 0.0]);
    test_complex_long_double([1.5, 0.0]);

    test_complex_float([2.5, 3.5]);
    test_complex_double([2.5, 3.5]);
    test_complex_long_double([2.5, 3.5]);

    test_pointer(Some(&I));

    test_struct();
}

fn main() {
    test_simple_assign();
    exit(0);
}