use std::process;

fn test_simple_assign<T: Copy + PartialEq + Default>(value: T) {
    let mut a: T = Default::default();
    let mut b: T = value;

    if a != Default::default() {
        process::exit(1);
    }
    if b != value {
        process::exit(1);
    }

    a = b;
    if a != value {
        process::exit(1);
    }
}

fn test_simple_assign_arith<T: Copy + PartialEq + Default + From<i32> + From<f64>>(value: T) {
    test_simple_assign(value);
}

fn test_simple_assign_complex<T: Copy + PartialEq + Default>(value: T) {
    test_simple_assign(value);
}

fn main() {
    test_simple_assign_arith(0);
    test_simple_assign_arith(1);
    test_simple_assign_arith(2);
    test_simple_assign_arith(-1);
    test_simple_assign_arith(1 << 63);
    test_simple_assign_arith(1.5);
    test_simple_assign_complex((2.5, 3.5));

    let mut i: i32 = 0;
    test_simple_assign(&i as *const i32);
    test_simple_assign(&i as *const i32);

    #[derive(Copy, Clone, PartialEq)]
    struct S {
        a: [i16; 1024],
    }

    let mut init = S { a: [0; 1024] };
    let mut copy = S { a: [0; 1024] };

    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    let mut s1 = init;
    let mut s2 = s1;

    copy = s1;
    if init != copy {
        process::exit(1);
    }

    copy = s2;
    if init != copy {
        process::exit(1);
    }

    copy = s1;
    if init != copy {
        process::exit(1);
    }

    copy = s2;
    if init != copy {
        process::exit(1);
    }

    process::exit(0);
}