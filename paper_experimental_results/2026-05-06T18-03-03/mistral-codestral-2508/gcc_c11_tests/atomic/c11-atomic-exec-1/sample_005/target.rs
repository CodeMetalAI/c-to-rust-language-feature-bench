use std::process::exit;
use std::mem::transmute;

fn abort() {
    exit(1);
}

fn test_simple_assign<T: Copy + PartialEq>(value: T) {
    let mut a: T = unsafe { transmute(0) };
    let b: T = value;

    if a != unsafe { transmute(0) } {
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

fn test_simple_assign_arith<T: Copy + PartialEq>(value: T) {
    test_simple_assign(value);
}

fn test_simple_assign_struct() {
    #[derive(Copy, Clone, PartialEq)]
    struct S {
        a: [i16; 1024],
    }

    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    let mut s1: S = unsafe { transmute(0) };
    let mut s2: S = unsafe { transmute(0) };

    let mut copy = s1 = init;
    if copy != init {
        abort();
    }

    copy = s2 = s1;
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
    test_simple_assign_arith(false);
    test_simple_assign_arith(true);
    test_simple_assign_arith(0i8);
    test_simple_assign_arith(1i8);
    test_simple_assign_arith(2i8);
    test_simple_assign_arith(-1i8);
    test_simple_assign_arith(0u8);
    test_simple_assign_arith(1u8);
    test_simple_assign_arith(2u8);
    test_simple_assign_arith(0i16);
    test_simple_assign_arith(1i16);
    test_simple_assign_arith(2i16);
    test_simple_assign_arith(-1i16);
    test_simple_assign_arith(0u16);
    test_simple_assign_arith(1u16);
    test_simple_assign_arith(2u16);
    test_simple_assign_arith(0i32);
    test_simple_assign_arith(1i32);
    test_simple_assign_arith(2i32);
    test_simple_assign_arith(-1i32);
    test_simple_assign_arith(0u32);
    test_simple_assign_arith(1u32);
    test_simple_assign_arith(2u32);
    test_simple_assign_arith(0i64);
    test_simple_assign_arith(1i64);
    test_simple_assign_arith(2i64);
    test_simple_assign_arith(-1i64);
    test_simple_assign_arith(0u64);
    test_simple_assign_arith(1u64);
    test_simple_assign_arith(2u64);
    test_simple_assign_arith(1.5f32);
    test_simple_assign_arith(1.5f64);

    let i = 0;
    test_simple_assign_arith(&i as *const i32);

    test_simple_assign_struct();

    exit(0);
}