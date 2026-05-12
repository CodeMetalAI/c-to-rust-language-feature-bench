use std::process::{exit, abort};
use std::mem::size_of;

fn test_simple_assign() {
    macro_rules! test_simple_assign_arith {
        ($value:expr) => {
            test_simple_assign::<$value>();
        };
    }

    fn test_simple_assign<T: PartialEq + Copy>(value: T) {
        let mut a: T = unsafe { std::mem::zeroed() };
        let mut b: T = value;

        if a != unsafe { std::mem::zeroed() } {
            abort();
        }
        if b != value {
            abort();
        }
        if (a = b) != value {
            abort();
        }
        if a != value {
            abort();
        }
    }

    test_simple_assign_arith!(false);
    test_simple_assign_arith!(true);
    test_simple_assign_arith!(0i8);
    test_simple_assign_arith!(0u8);
    test_simple_assign_arith!(0i16);
    test_simple_assign_arith!(0u16);
    test_simple_assign_arith!(0i32);
    test_simple_assign_arith!(0u32);
    test_simple_assign_arith!(0i64);
    test_simple_assign_arith!(0u64);
    test_simple_assign_arith!(0f32);
    test_simple_assign_arith!(0f64);

    let mut i: i32 = 0;
    test_simple_assign::<*const i32>(std::ptr::null());
    test_simple_assign::<*const i32>(&i as *const i32);

    #[derive(Copy, Clone, PartialEq)]
    struct S {
        a: [i16; 1024],
    }

    let mut init = S { a: [0; 1024] };
    let mut copy = S { a: [0; 1024] };

    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    let mut s1: S = unsafe { std::mem::zeroed() };
    let mut s2: S = unsafe { std::mem::zeroed() };

    copy = s1 = init;
    if init != copy {
        abort();
    }

    copy = s2 = s1;
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
    test_simple_assign();
    exit(0);
}