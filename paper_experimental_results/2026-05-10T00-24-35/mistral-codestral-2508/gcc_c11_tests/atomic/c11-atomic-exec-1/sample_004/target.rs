use std::process;
use std::mem::transmute;

fn abort() {
    process::exit(1);
}

fn test_simple_assign<T: Copy + PartialEq + Default>(value: T) {
    let mut a: T = Default::default();
    let b: T = value;

    if a != Default::default() {
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

fn test_simple_assign_arith<T: Copy + PartialEq + Default>(value: T) {
    test_simple_assign(value);
}

fn test_simple_assign_complex<T: Copy + PartialEq + Default>(value: T) {
    test_simple_assign(value);
}

fn test_simple_assign_struct() {
    #[derive(Copy, Clone, PartialEq)]
    struct S {
        a: [i16; 1024],
    }

    let mut init = S { a: [0; 1024] };
    let mut copy = S { a: [0; 1024] };
    let mut s1 = S { a: [0; 1024] };
    let mut s2 = S { a: [0; 1024] };

    for j in 0..1024 {
        init.a[j] = j as i16;
    }

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
    test_simple_assign_arith(0i8);
    test_simple_assign_arith(1i8);
    test_simple_assign_arith(2i8);
    test_simple_assign_arith(-1i8);
    test_simple_assign_arith(1i64 << 63);
    test_simple_assign_arith(1.5f64);
    test_simple_assign_complex((2.5f64, 3.5f64));

    let i = 0;
    test_simple_assign(&i as *const i32);
    test_simple_assign(&i as *const i32);

    test_simple_assign_struct();

    process::exit(0);
}