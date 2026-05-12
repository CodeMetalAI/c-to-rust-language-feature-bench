use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::process::exit;

macro_rules! test_simple_assign_atomic {
    ($atomic_type:ty, $value_type:ty, $value:expr, $zero:expr) => {{
        static A: $atomic_type = <$atomic_type>::new($zero);
        static B: $atomic_type = <$atomic_type>::new($zero);
        
        let a_val = A.load(Ordering::SeqCst);
        let b_init: $value_type = $value;
        B.store(b_init, Ordering::SeqCst);
        let b_val = B.load(Ordering::SeqCst);
        
        if a_val != $zero {
            panic!("abort");
        }
        if b_val != b_init {
            panic!("abort");
        }
        A.store(b_val, Ordering::SeqCst);
        if A.load(Ordering::SeqCst) != b_init {
            panic!("abort");
        }
        if A.load(Ordering::SeqCst) != b_init {
            panic!("abort");
        }
        // Reset for next test
        A.store($zero, Ordering::SeqCst);
        B.store($zero, Ordering::SeqCst);
    }};
}

macro_rules! test_simple_assign_mutex {
    ($value_type:ty, $value:expr, $zero:expr) => {{
        let a: Mutex<$value_type> = Mutex::new($zero);
        let b: Mutex<$value_type> = Mutex::new($value);
        
        if *a.lock().unwrap() != $zero {
            panic!("abort");
        }
        if *b.lock().unwrap() != $value {
            panic!("abort");
        }
        let b_val = *b.lock().unwrap();
        *a.lock().unwrap() = b_val;
        if *a.lock().unwrap() != $value {
            panic!("abort");
        }
        if *a.lock().unwrap() != $value {
            panic!("abort");
        }
    }};
}

fn test_simple_assign_arith(value: i64, fvalue: f64, creal: f64, cimag: f64) {
    // Bool
    test_simple_assign_atomic!(AtomicBool, bool, value != 0, false);
    
    // Integer types
    test_simple_assign_atomic!(AtomicI8, i8, value as i8, 0i8);
    test_simple_assign_atomic!(AtomicU8, u8, value as u8, 0u8);
    test_simple_assign_atomic!(AtomicI16, i16, value as i16, 0i16);
    test_simple_assign_atomic!(AtomicU16, u16, value as u16, 0u16);
    test_simple_assign_atomic!(AtomicI32, i32, value as i32, 0i32);
    test_simple_assign_atomic!(AtomicU32, u32, value as u32, 0u32);
    test_simple_assign_atomic!(AtomicI64, i64, value, 0i64);
    test_simple_assign_atomic!(AtomicU64, u64, value as u64, 0u64);
    
    // Floating point types using Mutex
    test_simple_assign_mutex!(f32, fvalue as f32, 0.0f32);
    test_simple_assign_mutex!(f64, fvalue, 0.0f64);
    
    // Complex types using Mutex (represented as tuples)
    test_simple_assign_mutex!((f32, f32), (creal as f32, cimag as f32), (0.0f32, 0.0f32));
    test_simple_assign_mutex!((f64, f64), (creal, cimag), (0.0f64, 0.0f64));
}

fn test_simple_assign() {
    test_simple_assign_arith(0, 0.0, 0.0, 0.0);
    test_simple_assign_arith(1, 1.0, 1.0, 0.0);
    test_simple_assign_arith(2, 2.0, 2.0, 0.0);
    test_simple_assign_arith(-1, -1.0, -1.0, 0.0);
    test_simple_assign_arith(1i64 << 63, (1u64 << 63) as f64, (1u64 << 63) as f64, 0.0);
    test_simple_assign_arith(1, 1.5, 1.5, 0.0);
    test_simple_assign_arith(2, 2.5, 2.5, 3.5);
    
    // Pointer test using AtomicUsize
    static I: AtomicI32 = AtomicI32::new(0);
    {
        let a: Mutex<usize> = Mutex::new(0);
        let b: Mutex<usize> = Mutex::new(0);
        
        if *a.lock().unwrap() != 0 {
            panic!("abort");
        }
        if *b.lock().unwrap() != 0 {
            panic!("abort");
        }
        let b_val = *b.lock().unwrap();
        *a.lock().unwrap() = b_val;
        if *a.lock().unwrap() != 0 {
            panic!("abort");
        }
    }
    {
        let addr = &I as *const AtomicI32 as usize;
        let a: Mutex<usize> = Mutex::new(0);
        let b: Mutex<usize> = Mutex::new(addr);
        
        if *a.lock().unwrap() != 0 {
            panic!("abort");
        }
        if *b.lock().unwrap() != addr {
            panic!("abort");
        }
        let b_val = *b.lock().unwrap();
        *a.lock().unwrap() = b_val;
        if *a.lock().unwrap() != addr {
            panic!("abort");
        }
    }
    
    // Struct test
    #[derive(Clone, Copy, PartialEq)]
    struct S {
        a: [i16; 1024],
    }
    
    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    
    let s1: Mutex<S> = Mutex::new(S { a: [0; 1024] });
    let s2: Mutex<S> = Mutex::new(S { a: [0; 1024] });
    
    *s1.lock().unwrap() = init;
    let copy = *s1.lock().unwrap();
    if init != copy {
        panic!("abort");
    }
    
    *s2.lock().unwrap() = *s1.lock().unwrap();
    let copy = *s2.lock().unwrap();
    if init != copy {
        panic!("abort");
    }
    
    let copy = *s1.lock().unwrap();
    if init != copy {
        panic!("abort");
    }
    
    let copy = *s2.lock().unwrap();
    if init != copy {
        panic!("abort");
    }
}

fn main() {
    test_simple_assign();
    exit(0);
}