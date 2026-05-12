use std::process;

#[derive(Copy, Clone, PartialEq)]
struct ComplexF32 {
    re: f32,
    im: f32,
}

#[derive(Copy, Clone, PartialEq)]
struct ComplexF64 {
    re: f64,
    im: f64,
}

#[derive(Copy, Clone, PartialEq)]
struct S {
    a: [i16; 1024],
}

fn abort() -> ! {
    process::abort();
}

macro_rules! test_simple_assign {
    ($ty:ty, $zero:expr, $val:expr) => {{
        let mut a: $ty = $zero;
        let b: $ty = $val;
        if a != $zero {
            abort();
        }
        if b != $val {
            abort();
        }
        a = b;
        if a != $val {
            abort();
        }
    }};
}

fn test_simple_assign_arith_i32(val: i32) {
    let bool_val = val != 0;
    test_simple_assign!(bool, false, bool_val);

    test_simple_assign!(i8, 0i8, val as i8); // char
    test_simple_assign!(i8, 0i8, val as i8); // signed char
    test_simple_assign!(u8, 0u8, val as u8);

    test_simple_assign!(i16, 0i16, val as i16);
    test_simple_assign!(u16, 0u16, val as u16);

    test_simple_assign!(i32, 0i32, val as i32);
    test_simple_assign!(u32, 0u32, val as u32);

    test_simple_assign!(i64, 0i64, val as i64); // long
    test_simple_assign!(u64, 0u64, val as u64); // unsigned long

    test_simple_assign!(i64, 0i64, val as i64); // long long
    test_simple_assign!(u64, 0u64, val as u64); // unsigned long long

    test_simple_assign!(f32, 0.0f32, val as f32);
    test_simple_assign!(f64, 0.0f64, val as f64);
    test_simple_assign!(f64, 0.0f64, val as f64); // long double

    test_simple_assign!(
        ComplexF32,
        ComplexF32 { re: 0.0, im: 0.0 },
        ComplexF32 {
            re: val as f32,
            im: 0.0
        }
    );
    test_simple_assign!(
        ComplexF64,
        ComplexF64 { re: 0.0, im: 0.0 },
        ComplexF64 {
            re: val as f64,
            im: 0.0
        }
    );
    test_simple_assign!(
        ComplexF64,
        ComplexF64 { re: 0.0, im: 0.0 },
        ComplexF64 {
            re: val as f64,
            im: 0.0
        }
    );
}

fn test_simple_assign_arith_u64(val: u64) {
    let bool_val = val != 0;
    test_simple_assign!(bool, false, bool_val);

    test_simple_assign!(i8, 0i8, val as i8); // char
    test_simple_assign!(i8, 0i8, val as i8); // signed char
    test_simple_assign!(u8, 0u8, val as u8);

    test_simple_assign!(i16, 0i16, val as i16);
    test_simple_assign!(u16, 0u16, val as u16);

    test_simple_assign!(i32, 0i32, val as i32);
    test_simple_assign!(u32, 0u32, val as u32);

    test_simple_assign!(i64, 0i64, val as i64); // long
    test_simple_assign!(u64, 0u64, val as u64); // unsigned long

    test_simple_assign!(i64, 0i64, val as i64); // long long
    test_simple_assign!(u64, 0u64, val as u64); // unsigned long long

    test_simple_assign!(f32, 0.0f32, val as f32);
    test_simple_assign!(f64, 0.0f64, val as f64);
    test_simple_assign!(f64, 0.0f64, val as f64); // long double

    test_simple_assign!(
        ComplexF32,
        ComplexF32 { re: 0.0, im: 0.0 },
        ComplexF32 {
            re: val as f32,
            im: 0.0
        }
    );
    test_simple_assign!(
        ComplexF64,
        ComplexF64 { re: 0.0, im: 0.0 },
        ComplexF64 {
            re: val as f64,
            im: 0.0
        }
    );
    test_simple_assign!(
        ComplexF64,
        ComplexF64 { re: 0.0, im: 0.0 },
        ComplexF64 {
            re: val as f64,
            im: 0.0
        }
    );
}

fn test_simple_assign_arith_f64(val: f64) {
    let bool_val = val != 0.0;
    test_simple_assign!(bool, false, bool_val);

    test_simple_assign!(i8, 0i8, val as i8); // char
    test_simple_assign!(i8, 0i8, val as i8); // signed char
    test_simple_assign!(u8, 0u8, val as u8);

    test_simple_assign!(i16, 0i16, val as i16);
    test_simple_assign!(u16, 0u16, val as u16);

    test_simple_assign!(i32, 0i32, val as i32);
    test_simple_assign!(u32, 0u32, val as u32);

    test_simple_assign!(i64, 0i64, val as i64); // long
    test_simple_assign!(u64, 0u64, val as u64); // unsigned long

    test_simple_assign!(i64, 0i64, val as i64); // long long
    test_simple_assign!(u64, 0u64, val as u64); // unsigned long long

    test_simple_assign!(f32, 0.0f32, val as f32);
    test_simple_assign!(f64, 0.0f64, val as f64);
    test_simple_assign!(f64, 0.0f64, val as f64); // long double

    test_simple_assign!(
        ComplexF32,
        ComplexF32 { re: 0.0, im: 0.0 },
        ComplexF32 {
            re: val as f32,
            im: 0.0
        }
    );
    test_simple_assign!(
        ComplexF64,
        ComplexF64 { re: 0.0, im: 0.0 },
        ComplexF64 {
            re: val as f64,
            im: 0.0
        }
    );
    test_simple_assign!(
        ComplexF64,
        ComplexF64 { re: 0.0, im: 0.0 },
        ComplexF64 {
            re: val as f64,
            im: 0.0
        }
    );
}

fn test_simple_assign_arith_complex(re: f64, im: f64) {
    let bool_val = re != 0.0 || im != 0.0;
    let real_val = re;

    test_simple_assign!(bool, false, bool_val);

    test_simple_assign!(i8, 0i8, real_val as i8); // char
    test_simple_assign!(i8, 0i8, real_val as i8); // signed char
    test_simple_assign!(u8, 0u8, real_val as u8);

    test_simple_assign!(i16, 0i16, real_val as i16);
    test_simple_assign!(u16, 0u16, real_val as u16);

    test_simple_assign!(i32, 0i32, real_val as i32);
    test_simple_assign!(u32, 0u32, real_val as u32);

    test_simple_assign!(i64, 0i64, real_val as i64); // long
    test_simple_assign!(u64, 0u64, real_val as u64); // unsigned long

    test_simple_assign!(i64, 0i64, real_val as i64); // long long
    test_simple_assign!(u64, 0u64, real_val as u64); // unsigned long long

    test_simple_assign!(f32, 0.0f32, real_val as f32);
    test_simple_assign!(f64, 0.0f64, real_val as f64);
    test_simple_assign!(f64, 0.0f64, real_val as f64); // long double

    test_simple_assign!(
        ComplexF32,
        ComplexF32 { re: 0.0, im: 0.0 },
        ComplexF32 {
            re: re as f32,
            im: im as f32
        }
    );
    test_simple_assign!(
        ComplexF64,
        ComplexF64 { re: 0.0, im: 0.0 },
        ComplexF64 { re, im }
    );
    test_simple_assign!(
        ComplexF64,
        ComplexF64 { re: 0.0, im: 0.0 },
        ComplexF64 { re, im }
    );
}

fn test_simple_assign() {
    test_simple_assign_arith_i32(0);
    test_simple_assign_arith_i32(1);
    test_simple_assign_arith_i32(2);
    test_simple_assign_arith_i32(-1);
    test_simple_assign_arith_u64(1u64 << 63);
    test_simple_assign_arith_f64(1.5);
    test_simple_assign_arith_complex(2.5, 3.5);

    test_simple_assign!(usize, 0usize, 0usize);
    test_simple_assign!(usize, 0usize, 1usize);

    let mut init = S { a: [0i16; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    let mut s1 = S { a: [0i16; 1024] };
    let mut s2 = S { a: [0i16; 1024] };

    let mut copy = {
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

fn main() {
    test_simple_assign();
    process::exit(0);
}