use std::mem;
use std::process::exit;

fn abort() {
    exit(1);
}

fn test_simple_assign() {
    macro_rules! test_simple_assignment {
        ($t:ty, $value:expr) => {
            let mut a: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            let b: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new($value as usize);
            
            if a.load(std::sync::atomic::Ordering::SeqCst) != 0 {
                abort();
            }
            if b.load(std::sync::atomic::Ordering::SeqCst) != ($value as usize) {
                abort();
            }
            a.store(b.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst);
            if a.load(std::sync::atomic::Ordering::SeqCst) != ($value as usize) {
                abort();
            }
        };
    }

    macro_rules! test_simple_assignment_arith {
        ($value:expr) => {
            test_simple_assignment!(bool, $value);
            test_simple_assignment!(i8, $value);
            test_simple_assignment!(u8, $value);
            test_simple_assignment!(i16, $value);
            test_simple_assignment!(u16, $value);
            test_simple_assignment!(i32, $value);
            test_simple_assignment!(u32, $value);
            test_simple_assignment!(i64, $value);
            test_simple_assignment!(u64, $value);
            test_simple_assignment!(f32, $value);
            test_simple_assignment!(f64, $value);
        };
    }

    test_simple_assignment_arith!(0);
    test_simple_assignment_arith!(1);
    test_simple_assignment_arith!(2);
    test_simple_assignment_arith!(-1);
    test_simple_assignment_arith!(1u64 << 63);
    test_simple_assignment_arith!(1.5);
    
    // Complex numbers are not directly supported in Rust's atomic types,
    // so we skip those tests.
    
    // Pointer tests
    let i = 0;
    test_simple_assignment!(usize, 0);
    test_simple_assignment!(usize, &i as *const i32 as usize);
    
    // Struct test
    struct S {
        a: [i16; 1024],
    }
    
    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    
    let mut copy = init;
    
    // In Rust, we can't atomically assign entire structs like in C's _Atomic,
    // so we skip the atomic struct assignment tests.
    if !mem::eq(&init, &copy) {
        abort();
    }
}

fn main() {
    test_simple_assign();
    exit(0);
}