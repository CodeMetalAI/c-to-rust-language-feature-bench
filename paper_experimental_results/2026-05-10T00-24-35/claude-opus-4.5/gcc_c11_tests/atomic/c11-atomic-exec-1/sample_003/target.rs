use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::process::exit;

macro_rules! test_simple_assign_atomic {
    ($atomic_type:ty, $value_type:ty, $value:expr) => {{
        static A: $atomic_type = <$atomic_type>::new(0);
        static B: $atomic_type = <$atomic_type>::new(0);
        
        // Reset for each test
        A.store(0, Ordering::SeqCst);
        let val: $value_type = $value;
        B.store(val, Ordering::SeqCst);
        
        if A.load(Ordering::SeqCst) != 0 {
            panic!("abort");
        }
        if B.load(Ordering::SeqCst) != val {
            panic!("abort");
        }
        let b_val = B.load(Ordering::SeqCst);
        A.store(b_val, Ordering::SeqCst);
        if A.load(Ordering::SeqCst) != val {
            panic!("abort");
        }
    }};
}

macro_rules! test_simple_assign_mutex {
    ($value_type:ty, $value:expr, $zero:expr) => {{
        let a: Mutex<$value_type> = Mutex::new($zero);
        let val: $value_type = $value;
        let b: Mutex<$value_type> = Mutex::new(val);
        
        if *a.lock().unwrap() != $zero {
            panic!("abort");
        }
        if *b.lock().unwrap() != val {
            panic!("abort");
        }
        let b_val = *b.lock().unwrap();
        *a.lock().unwrap() = b_val;
        if *a.lock().unwrap() != val {
            panic!("abort");
        }
    }};
}

fn test_simple_assign_arith(value: i128) {
    // bool
    test_simple_assign_atomic!(AtomicBool, bool, value != 0);
    
    // char (i8)
    test_simple_assign_atomic!(AtomicI8, i8, value as i8);
    
    // signed char / unsigned char
    test_simple_assign_atomic!(AtomicI8, i8, value as i8);
    test_simple_assign_atomic!(AtomicU8, u8, value as u8);
    
    // short
    test_simple_assign_atomic!(AtomicI16, i16, value as i16);
    test_simple_assign_atomic!(AtomicU16, u16, value as u16);
    
    // int
    test_simple_assign_atomic!(AtomicI32, i32, value as i32);
    test_simple_assign_atomic!(AtomicU32, u32, value as u32);
    
    // long (assuming 64-bit)
    test_simple_assign_atomic!(AtomicI64, i64, value as i64);
    test_simple_assign_atomic!(AtomicU64, u64, value as u64);
    
    // long long
    test_simple_assign_atomic!(AtomicI64, i64, value as i64);
    test_simple_assign_atomic!(AtomicU64, u64, value as u64);
    
    // float, double, long double - use Mutex since no atomic float
    test_simple_assign_mutex!(f32, value as f32, 0.0f32);
    test_simple_assign_mutex!(f64, value as f64, 0.0f64);
    test_simple_assign_mutex!(f64, value as f64, 0.0f64); // long double as f64
}

fn test_simple_assign_arith_float(value: f64) {
    test_simple_assign_mutex!(f32, value as f32, 0.0f32);
    test_simple_assign_mutex!(f64, value, 0.0f64);
}

fn test_simple_assign() {
    test_simple_assign_arith(0);
    test_simple_assign_arith(1);
    test_simple_assign_arith(2);
    test_simple_assign_arith(-1);
    test_simple_assign_arith(1i128 << 63);
    test_simple_assign_arith_float(1.5);
    
    // Pointer tests
    test_simple_assign_atomic!(AtomicUsize, usize, 0);
    static I: i32 = 0;
    let i_ptr = &I as *const i32 as usize;
    let a: Mutex<usize> = Mutex::new(0);
    let b: Mutex<usize> = Mutex::new(i_ptr);
    if *a.lock().unwrap() != 0 {
        panic!("abort");
    }
    if *b.lock().unwrap() != i_ptr {
        panic!("abort");
    }
    *a.lock().unwrap() = *b.lock().unwrap();
    if *a.lock().unwrap() != i_ptr {
        panic!("abort");
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