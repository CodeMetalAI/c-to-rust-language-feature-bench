use std::process;

macro_rules! test_simple_assign {
    ($type:ty, $value:expr) => {
        {
            static mut A: $type = 0 as $type;
            static mut B: $type = $value as $type;
            unsafe {
                if A != 0 as $type {
                    process::abort();
                }
                if B != $value as $type {
                    process::abort();
                }
                if (A = B) != $value as $type {
                    process::abort();
                }
                if A != $value as $type {
                    process::abort();
                }
            }
        }
    };
}

macro_rules! test_simple_assign_arith {
    ($value:expr) => {
        {
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
        }
    };
}

fn test_simple_assign() {
    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!(1u64 << 63);
    test_simple_assign_arith!(1.5);
    static mut I: i32 = 0;
    test_simple_assign!(*mut i32, 0 as *mut i32);
    unsafe {
        test_simple_assign!(*mut i32, &mut I);
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
    copy = s1;
    if init != copy {
        process::abort();
    }
    copy = s2;
    if init != copy {
        process::abort();
    }
    copy = s1;
    if init != copy {
        process::abort();
    }
    copy = s2;
    if init != copy {
        process::abort();
    }
}

fn main() {
    test_simple_assign();
    process::exit(0);
}