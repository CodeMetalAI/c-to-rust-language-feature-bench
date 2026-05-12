use std::process;

fn test_simple_assign<T: PartialEq + Copy>(value: T) {
    let mut a: T = unsafe { std::mem::zeroed() };
    let mut b: T = value;

    if a != unsafe { std::mem::zeroed() } {
        process::exit(1);
    }
    if b != value {
        process::exit(1);
    }

    a = b;
    if a != value {
        process::exit(1);
    }
    if a != value {
        process::exit(1);
    }
}

fn test_simple_assign_arith<T: PartialEq + Copy>(value: T) {
    test_simple_assign(value);
}

fn main() {
    test_simple_assign_arith(0);
    test_simple_assign_arith(1);
    test_simple_assign_arith(2);
    test_simple_assign_arith(-1);
    test_simple_assign_arith(1 << 63);
    test_simple_assign_arith(1.5);
    // Complex numbers are not directly supported in Rust, so we skip them.

    let mut i: i32 = 0;
    test_simple_assign(&i as *const i32);
    test_simple_assign(&mut i as *mut i32);

    #[derive(PartialEq, Copy, Clone)]
    struct S {
        a: [i16; 1024],
    }

    let mut init = S { a: [0; 1024] };
    let mut copy = S { a: [0; 1024] };

    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    let s1 = init;
    copy = s1;
    if init != copy {
        process::exit(1);
    }

    let s2 = s1;
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