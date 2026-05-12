use std::process::exit;
use std::mem::transmute;

fn test_simple_assign() {
    macro_rules! test_simple_assign {
        ($type:ty, $value:expr) => {
            let mut a: $type = 0;
            let mut b: $type = $value;
            if a != 0 {
                eprintln!("Test failed: a != 0");
                exit(1);
            }
            if b != $value {
                eprintln!("Test failed: b != {}", $value);
                exit(1);
            }
            a = b;
            if a != $value {
                eprintln!("Test failed: a != {}", $value);
                exit(1);
            }
        };
    }

    macro_rules! test_simple_assign_arith {
        ($value:expr) => {
            test_simple_assign!(bool, $value);
            test_simple_assign!(i8, $value);
            test_simple_assign!(u8, $value);
            test_simple_assign!(i16, $value);
            test_simple_assign!(u16, $value);
            test_simple_assign!(i32, $value);
            test_simple_assign!(u32, $value);
            test_simple_assign!(i64, $value);
            test_simple_assign!(u64, $value);
            test_simple_assign!(f32, $value);
            test_simple_assign!(f64, $value);
        };
    }

    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!(1u64 << 63);
    test_simple_assign_arith!(1.5);

    let mut a: *const i32 = std::ptr::null();
    let mut b: *const i32 = std::ptr::null();
    if a != std::ptr::null() {
        eprintln!("Test failed: a != null");
        exit(1);
    }
    if b != std::ptr::null() {
        eprintln!("Test failed: b != null");
        exit(1);
    }
    a = b;
    if a != std::ptr::null() {
        eprintln!("Test failed: a != null");
        exit(1);
    }

    let mut i: i32 = 0;
    let mut a_ptr: *const i32 = std::ptr::null();
    let mut b_ptr: *const i32 = &i;
    if a_ptr != std::ptr::null() {
        eprintln!("Test failed: a_ptr != null");
        exit(1);
    }
    if b_ptr != &i {
        eprintln!("Test failed: b_ptr != &i");
        exit(1);
    }
    a_ptr = b_ptr;
    if a_ptr != &i {
        eprintln!("Test failed: a_ptr != &i");
        exit(1);
    }

    #[repr(C)]
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

    copy = unsafe { transmute(s1 = init) };
    if init.a != copy.a {
        eprintln!("Test failed: init.a != copy.a");
        exit(1);
    }

    copy = unsafe { transmute(s2 = s1) };
    if init.a != copy.a {
        eprintln!("Test failed: init.a != copy.a");
        exit(1);
    }

    copy = unsafe { transmute(s1) };
    if init.a != copy.a {
        eprintln!("Test failed: init.a != copy.a");
        exit(1);
    }

    copy = unsafe { transmute(s2) };
    if init.a != copy.a {
        eprintln!("Test failed: init.a != copy.a");
        exit(1);
    }
}

fn main() {
    test_simple_assign();
    exit(0);
}