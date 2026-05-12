use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::process::exit;

macro_rules! test_simple_assign_atomic {
    ($atomic_type:ty, $value_type:ty, $value:expr) => {{
        static A: $atomic_type = <$atomic_type>::new(0);
        static B: $atomic_type = <$atomic_type>::new(0);
        
        let a_val = A.load(Ordering::SeqCst);
        let b_val = B.load(Ordering::SeqCst);
        let expected_val: $value_type = $value as $value_type;
        
        // Reset for this test
        A.store(0, Ordering::SeqCst);
        B.store(expected_val, Ordering::SeqCst);
        
        if A.load(Ordering::SeqCst) != 0 {
            panic!("abort");
        }
        if B.load(Ordering::SeqCst) != expected_val {
            panic!("abort");
        }
        let b_loaded = B.load(Ordering::SeqCst);
        A.store(b_loaded, Ordering::SeqCst);
        if b_loaded != expected_val {
            panic!("abort");
        }
        if A.load(Ordering::SeqCst) != expected_val {
            panic!("abort");
        }
    }};
}

macro_rules! test_simple_assign_mutex {
    ($value_type:ty, $value:expr, $default:expr) => {{
        let a: Mutex<$value_type> = Mutex::new($default);
        let b: Mutex<$value_type> = Mutex::new($value as $value_type);
        
        if *a.lock().unwrap() != $default {
            panic!("abort");
        }
        let expected: $value_type = $value as $value_type;
        if *b.lock().unwrap() != expected {
            panic!("abort");
        }
        let b_val = *b.lock().unwrap();
        *a.lock().unwrap() = b_val;
        if b_val != expected {
            panic!("abort");
        }
        if *a.lock().unwrap() != expected {
            panic!("abort");
        }
    }};
}

fn test_simple_assign_arith(value: i64) {
    // bool
    {
        let a: Mutex<bool> = Mutex::new(false);
        let b: Mutex<bool> = Mutex::new(value != 0);
        let expected = value != 0;
        if *a.lock().unwrap() != false {
            panic!("abort");
        }
        if *b.lock().unwrap() != expected {
            panic!("abort");
        }
        let b_val = *b.lock().unwrap();
        *a.lock().unwrap() = b_val;
        if b_val != expected {
            panic!("abort");
        }
        if *a.lock().unwrap() != expected {
            panic!("abort");
        }
    }
    
    test_simple_assign_mutex!(i8, value, 0i8);
    test_simple_assign_mutex!(u8, value, 0u8);
    test_simple_assign_mutex!(i16, value, 0i16);
    test_simple_assign_mutex!(u16, value, 0u16);
    test_simple_assign_mutex!(i32, value, 0i32);
    test_simple_assign_mutex!(u32, value, 0u32);
    test_simple_assign_mutex!(i64, value, 0i64);
    test_simple_assign_mutex!(u64, value, 0u64);
    test_simple_assign_mutex!(f32, value, 0.0f32);
    test_simple_assign_mutex!(f64, value, 0.0f64);
}

fn test_simple_assign() {
    test_simple_assign_arith(0);
    test_simple_assign_arith(1);
    test_simple_assign_arith(2);
    test_simple_assign_arith(-1);
    test_simple_assign_arith(1i64 << 63);
    
    // Test with floating point value
    test_simple_assign_mutex!(f32, 1.5f32, 0.0f32);
    test_simple_assign_mutex!(f64, 1.5f64, 0.0f64);
    
    // Test with struct
    #[derive(Clone, PartialEq)]
    struct S {
        a: [i16; 1024],
    }
    
    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    
    let s1: Mutex<S> = Mutex::new(S { a: [0; 1024] });
    let s2: Mutex<S> = Mutex::new(S { a: [0; 1024] });
    
    *s1.lock().unwrap() = init.clone();
    let copy = s1.lock().unwrap().clone();
    if init != copy {
        panic!("abort");
    }
    
    *s2.lock().unwrap() = s1.lock().unwrap().clone();
    let copy = s2.lock().unwrap().clone();
    if init != copy {
        panic!("abort");
    }
    
    let copy = s1.lock().unwrap().clone();
    if init != copy {
        panic!("abort");
    }
    
    let copy = s2.lock().unwrap().clone();
    if init != copy {
        panic!("abort");
    }
}

fn main() {
    test_simple_assign();
    exit(0);
}