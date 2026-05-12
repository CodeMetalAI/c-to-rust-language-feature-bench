use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, 
                       AtomicI32, AtomicU32, AtomicIsize, AtomicUsize, 
                       AtomicI64, AtomicU64, Ordering};
use std::sync::atomic::{AtomicPtr};
use std::mem;

// Helper macro similar to C's TEST_SIMPLE_ASSIGN
macro_rules! test_simple_assign {
    ($atomic_type:ty, $value:expr) => {
        {
            let mut a = <$atomic_type>::new(Default::default());
            let b = <$atomic_type>::new($value);
            
            // Check initial values
            if a.load(Ordering::SeqCst) != Default::default() {
                std::process::exit(1);
            }
            if b.load(Ordering::SeqCst) != $value {
                std::process::exit(1);
            }
            
            // Store value from b into a
            a.store(b.load(Ordering::SeqCst), Ordering::SeqCst);
            
            // Check a's value after assignment
            if a.load(Ordering::SeqCst) != $value {
                std::process::exit(1);
            }
        }
    };
}

// Macro to test all arithmetic types
macro_rules! test_simple_assign_arith {
    ($value:expr) => {
        test_simple_assign!(AtomicBool, $value != 0);
        test_simple_assign!(AtomicI8, $value as i8);
        test_simple_assign!(AtomicU8, $value as u8);
        test_simple_assign!(AtomicI16, $value as i16);
        test_simple_assign!(AtomicU16, $value as u16);
        test_simple_assign!(AtomicI32, $value as i32);
        test_simple_assign!(AtomicU32, $value as u32);
        test_simple_assign!(AtomicIsize, $value as isize);
        test_simple_assign!(AtomicUsize, $value as usize);
        test_simple_assign!(AtomicI64, $value as i64);
        test_simple_assign!(AtomicU64, $value as u64);
        
        // For floating point and complex types, we use a different approach
        // since Rust's standard library doesn't have atomic floats or complex numbers
        // We'll handle these separately in the test function
    };
}

fn test_simple_assign() {
    // Test integer values
    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!(1u64 << 63);
    
    // Test pointer types
    let mut i = 0;
    test_simple_assign!(AtomicPtr<i32>, std::ptr::null_mut());
    test_simple_assign!(AtomicPtr<i32>, &mut i as *mut i32);
    
    // Test struct assignment (using non-atomic for structs since Rust doesn't have atomic structs)
    #[derive(Clone, Copy)]
    struct S {
        a: [i16; 1024],
    }
    
    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    
    // Simulate atomic struct operations with normal assignment and memory barriers
    let s1 = init;
    let mut copy = s1;
    
    if !s1.a.iter().zip(init.a.iter()).all(|(x, y)| x == y) {
        std::process::exit(1);
    }
    
    let s2 = s1;
    copy = s2;
    
    if !copy.a.iter().zip(init.a.iter()).all(|(x, y)| x == y) {
        std::process::exit(1);
    }
    
    copy = s1;
    if !copy.a.iter().zip(init.a.iter()).all(|(x, y)| x == y) {
        std::process::exit(1);
    }
    
    copy = s2;
    if !copy.a.iter().zip(init.a.iter()).all(|(x, y)| x == y) {
        std::process::exit(1);
    }
}

fn main() {
    test_simple_assign();
    std::process::exit(0);
}