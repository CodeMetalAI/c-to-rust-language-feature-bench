use std::mem;
use std::process;

fn abort() {
    process::exit(1);
}

macro_rules! TEST_SIMPLE_ASSIGN {
    ($type:ty, $value:expr) => {
        {
            static mut A: $type = 0;
            static mut B: $type = $value;
            // Ensure A is zero (initialized)
            if A != 0 {
                abort();
            }
            if B != $value {
                abort();
            }
            A = B;
            if A != $value {
                abort();
            }
        }
    };
}

macro_rules! TEST_SIMPLE_ASSIGN_ARITH {
    ($value:expr) => {
        TEST_SIMPLE_ASSIGN!(bool, $value);
        TEST_SIMPLE_ASSIGN!(i8, $value);
        TEST_SIMPLE_ASSIGN!(u8, $value);
        TEST_SIMPLE_ASSIGN!(i16, $value);
        TEST_SIMPLE_ASSIGN!(u16, $value);
        TEST_SIMPLE_ASSIGN!(i32, $value);
        TEST_SIMPLE_ASSIGN!(u32, $value);
        TEST_SIMPLE_ASSIGN!(i64, $value);
        TEST_SIMPLE_ASSIGN!(u64, $value);
        TEST_SIMPLE_ASSIGN!(f32, $value);
        TEST_SIMPLE_ASSIGN!(f64, $value);
        // Rust doesn't have a long double equivalent, use f64
        TEST_SIMPLE_ASSIGN!(f64, $value);
        // Complex numbers are not standard in Rust, skip them
        // TEST_SIMPLE_ASSIGN!(_Complex float, $value);
        // TEST_SIMPLE_ASSIGN!(_Complex double, $value);
        // TEST_SIMPLE_ASSIGN!(_Complex long double, $value);
    };
}

fn test_simple_assign() {
    TEST_SIMPLE_ASSIGN_ARITH!(0);
    TEST_SIMPLE_ASSIGN_ARITH!(1);
    TEST_SIMPLE_ASSIGN_ARITH!(2);
    TEST_SIMPLE_ASSIGN_ARITH!(-1);
    TEST_SIMPLE_ASSIGN_ARITH!(1u64 << 63);
    TEST_SIMPLE_ASSIGN_ARITH!(1.5);
    // Skip complex number tests
    static mut I: i32 = 0;
    TEST_SIMPLE_ASSIGN!(*mut i32, 0 as *mut i32);
    TEST_SIMPLE_ASSIGN!(*mut i32, &mut I as *mut i32);
    
    struct S {
        a: [i16; 1024],
    }
    
    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    
    let mut s1 = init;
    let mut s2 = init;
    
    let copy = s1;
    if mem::ne(&init, &copy) {
        abort();
    }
    let copy = s2;
    if mem::ne(&init, &copy) {
        abort();
    }
    let copy = s1;
    if mem::ne(&init, &copy) {
        abort();
    }
    let copy = s2;
    if mem::ne(&init, &copy) {
        abort();
    }
}

fn main() {
    test_simple_assign();
    process::exit(0);
}