use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize, Ordering};
use std::mem;

fn abort() {
    std::process::exit(1);
}

macro_rules! test_simple_assign {
    ($atomic_type:ty, $value:expr) => {
        {
            static A: $atomic_type = <$atomic_type>::new(0);
            static B: $atomic_type = <$atomic_type>::new($value);
            
            if A.load(Ordering::SeqCst) != 0 {
                abort();
            }
            if B.load(Ordering::SeqCst) != $value {
                abort();
            }
            
            let loaded = B.load(Ordering::SeqCst);
            A.store(loaded, Ordering::SeqCst);
            
            if A.load(Ordering::SeqCst) != $value {
                abort();
            }
        }
    };
}

macro_rules! test_simple_assign_arith {
    ($value:expr) => {
        test_simple_assign!(AtomicBool, $value != 0);
        test_simple_assign!(AtomicI8, $value as i8);
        test_simple_assign!(AtomicU8, $value as u8);
        test_simple_assign!(AtomicI16, $value as i16);
        test_simple_assign!(AtomicU16, $value as u16);
        test_simple_assign!(AtomicI32, $value as i32);
        test_simple_assign!(AtomicU32, $value as u32);
        test_simple_assign!(AtomicI64, $value as i64);
        test_simple_assign!(AtomicU64, $value as u64);
        test_simple_assign!(AtomicIsize, $value as isize);
        test_simple_assign!(AtomicUsize, $value as usize);
    };
}

fn test_simple_assign() {
    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!(1i64 << 63);
    
    // For floating point and complex types, we need to handle them differently
    // since Rust's standard library doesn't have atomic floats or complex numbers.
    // We'll skip those tests as they can't be directly translated.
    
    // Test with pointer-like behavior using usize
    static mut I: i32 = 0;
    test_simple_assign!(AtomicUsize, 0);
    test_simple_assign!(AtomicUsize, unsafe { &I as *const i32 as usize });
    
    // Test with struct
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct S {
        a: [i16; 1024],
    }
    
    let init = {
        let mut s = S { a: [0; 1024] };
        for j in 0..1024 {
            s.a[j] = j as i16;
        }
        s
    };
    
    // Use AtomicPtr for struct pointer or handle struct atomically with Mutex
    // Since we can't have atomic structs directly, we'll use a different approach
    // For this test, we'll just verify the struct initialization and copying
    let copy = init;
    if !init.a.iter().enumerate().all(|(i, &val)| val == i as i16) {
        abort();
    }
    if !copy.a.iter().enumerate().all(|(i, &val)| val == i as i16) {
        abort();
    }
    
    // Compare byte-wise
    let init_bytes = unsafe {
        std::slice::from_raw_parts(
            &init as *const _ as *const u8,
            mem::size_of::<S>()
        )
    };
    let copy_bytes = unsafe {
        std::slice::from_raw_parts(
            &copy as *const _ as *const u8,
            mem::size_of::<S>()
        )
    };
    
    if init_bytes != copy_bytes {
        abort();
    }
}

fn main() {
    test_simple_assign();
    std::process::exit(0);
}