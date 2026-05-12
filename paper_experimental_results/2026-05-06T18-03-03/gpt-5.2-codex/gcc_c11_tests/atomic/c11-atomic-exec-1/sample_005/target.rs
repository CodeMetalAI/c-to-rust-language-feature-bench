use std::process;

#[derive(Copy, Clone, PartialEq)]
struct Complex32 {
    re: f32,
    im: f32,
}
impl Default for Complex32 {
    fn default() -> Self {
        Complex32 { re: 0.0, im: 0.0 }
    }
}

#[derive(Copy, Clone, PartialEq)]
struct Complex64 {
    re: f64,
    im: f64,
}
impl Default for Complex64 {
    fn default() -> Self {
        Complex64 { re: 0.0, im: 0.0 }
    }
}

#[derive(Copy, Clone, PartialEq)]
struct ComplexLD {
    re: f64,
    im: f64,
}
impl Default for ComplexLD {
    fn default() -> Self {
        ComplexLD { re: 0.0, im: 0.0 }
    }
}

fn abort() -> ! {
    process::abort();
}

fn test_simple_assign<T: PartialEq + Copy>(zero: T, value: T) {
    let mut a = zero;
    let b = value;
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
}

fn test_arith_values(
    b: bool,
    c: i8,
    sc: i8,
    uc: u8,
    ss: i16,
    us: u16,
    si: i32,
    ui: u32,
    sl: i64,
    ul: u64,
    sll: i64,
    ull: u64,
    f: f32,
    d: f64,
    ld: f64,
    cf: Complex32,
    cd: Complex64,
    cld: ComplexLD,
) {
    test_simple_assign(false, b);
    test_simple_assign(0i8, c);
    test_simple_assign(0i8, sc);
    test_simple_assign(0u8, uc);
    test_simple_assign(0i16, ss);
    test_simple_assign(0u16, us);
    test_simple_assign(0i32, si);
    test_simple_assign(0u32, ui);
    test_simple_assign(0i64, sl);
    test_simple_assign(0u64, ul);
    test_simple_assign(0i64, sll);
    test_simple_assign(0u64, ull);
    test_simple_assign(0.0f32, f);
    test_simple_assign(0.0f64, d);
    test_simple_assign(0.0f64, ld);
    test_simple_assign(Complex32::default(), cf);
    test_simple_assign(Complex64::default(), cd);
    test_simple_assign(ComplexLD::default(), cld);
}

fn test_from_i64(v: i64) {
    test_arith_values(
        v != 0,
        v as i8,
        v as i8,
        v as u8,
        v as i16,
        v as u16,
        v as i32,
        v as u32,
        v as i64,
        v as u64,
        v as i64,
        v as u64,
        v as f32,
        v as f64,
        v as f64,
        Complex32 {
            re: v as f32,
            im: 0.0,
        },
        Complex64 {
            re: v as f64,
            im: 0.0,
        },
        ComplexLD {
            re: v as f64,
            im: 0.0,
        },
    );
}

fn test_from_u64(v: u64) {
    test_arith_values(
        v != 0,
        v as i8,
        v as i8,
        v as u8,
        v as i16,
        v as u16,
        v as i32,
        v as u32,
        v as i64,
        v as u64,
        v as i64,
        v as u64,
        v as f32,
        v as f64,
        v as f64,
        Complex32 {
            re: v as f32,
            im: 0.0,
        },
        Complex64 {
            re: v as f64,
            im: 0.0,
        },
        ComplexLD {
            re: v as f64,
            im: 0.0,
        },
    );
}

fn test_from_f64(v: f64) {
    test_arith_values(
        v != 0.0,
        v as i8,
        v as i8,
        v as u8,
        v as i16,
        v as u16,
        v as i32,
        v as u32,
        v as i64,
        v as u64,
        v as i64,
        v as u64,
        v as f32,
        v,
        v,
        Complex32 {
            re: v as f32,
            im: 0.0,
        },
        Complex64 { re: v, im: 0.0 },
        ComplexLD { re: v, im: 0.0 },
    );
}

fn test_from_complex(re: f64, im: f64) {
    test_arith_values(
        re != 0.0,
        re as i8,
        re as i8,
        re as u8,
        re as i16,
        re as u16,
        re as i32,
        re as u32,
        re as i64,
        re as u64,
        re as i64,
        re as u64,
        re as f32,
        re,
        re,
        Complex32 {
            re: re as f32,
            im: im as f32,
        },
        Complex64 { re, im },
        ComplexLD { re, im },
    );
}

#[derive(Copy, Clone, PartialEq)]
struct S {
    a: [i16; 1024],
}

fn test_simple_assign_all() {
    test_from_i64(0);
    test_from_i64(1);
    test_from_i64(2);
    test_from_i64(-1);
    test_from_u64(1u64 << 63);
    test_from_f64(1.5);
    test_from_complex(2.5, 3.5);

    static I: i32 = 0;
    let ptr_null: *const i32 = std::ptr::null();
    let ptr_i: *const i32 = &I as *const i32;
    test_simple_assign(ptr_null, ptr_null);
    test_simple_assign(ptr_null, ptr_i);

    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    let mut s1 = S { a: [0; 1024] };
    let mut s2 = S { a: [0; 1024] };
    let mut copy: S;

    s1 = init;
    copy = s1;
    if copy != init {
        abort();
    }
    s2 = s1;
    copy = s2;
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
    test_simple_assign_all();
}