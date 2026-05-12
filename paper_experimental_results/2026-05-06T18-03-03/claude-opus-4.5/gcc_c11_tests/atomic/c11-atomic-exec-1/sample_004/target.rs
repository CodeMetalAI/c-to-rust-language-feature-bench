use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::process::exit;

macro_rules! test_simple_assign_atomic {
    ($atomic_type:ty, $value_type:ty, $value:expr, $zero:expr) => {{
        static A: $atomic_type = <$atomic_type>::new($zero);
        static B: $atomic_type = <$atomic_type>::new($zero);
        
        // Reset for each test
        A.store($zero, Ordering::SeqCst);
        B.store($value as $value_type, Ordering::SeqCst);
        
        if A.load(Ordering::SeqCst) != $zero {
            panic!("abort");
        }
        if B.load(Ordering::SeqCst) != ($value as $value_type) {
            panic!("abort");
        }
        let b_val = B.load(Ordering::SeqCst);
        A.store(b_val, Ordering::SeqCst);
        if A.load(Ordering::SeqCst) != ($value as $value_type) {
            panic!("abort");
        }
    }};
}

macro_rules! test_simple_assign_mutex {
    ($value_type:ty, $value:expr, $zero:expr) => {{
        let a: Mutex<$value_type> = Mutex::new($zero);
        let b: Mutex<$value_type> = Mutex::new($value as $value_type);
        
        if *a.lock().unwrap() != $zero {
            panic!("abort");
        }
        if *b.lock().unwrap() != ($value as $value_type) {
            panic!("abort");
        }
        let b_val = *b.lock().unwrap();
        *a.lock().unwrap() = b_val;
        if *a.lock().unwrap() != ($value as $value_type) {
            panic!("abort");
        }
    }};
}

fn test_simple_assign_arith(value: i64) {
    // bool
    {
        let bool_val = value != 0;
        let a: Mutex<bool> = Mutex::new(false);
        let b: Mutex<bool> = Mutex::new(bool_val);
        if *a.lock().unwrap() != false { panic!("abort"); }
        if *b.lock().unwrap() != bool_val { panic!("abort"); }
        *a.lock().unwrap() = *b.lock().unwrap();
        if *a.lock().unwrap() != bool_val { panic!("abort"); }
    }
    
    test_simple_assign_mutex!(i8, value, 0i8);
    test_simple_assign_mutex!(i8, value, 0i8);
    test_simple_assign_mutex!(u8, value, 0u8);
    test_simple_assign_mutex!(i16, value, 0i16);
    test_simple_assign_mutex!(u16, value, 0u16);
    test_simple_assign_mutex!(i32, value, 0i32);
    test_simple_assign_mutex!(u32, value, 0u32);
    test_simple_assign_mutex!(i64, value, 0i64);
    test_simple_assign_mutex!(u64, value, 0u64);
    test_simple_assign_mutex!(i64, value, 0i64);
    test_simple_assign_mutex!(u64, value, 0u64);
    test_simple_assign_mutex!(f32, value as f32, 0.0f32);
    test_simple_assign_mutex!(f64, value as f64, 0.0f64);
    test_simple_assign_mutex!(f64, value as f64, 0.0f64);
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
    test_simple_assign_arith(1i64 << 63);
    test_simple_assign_arith_float(1.5);
    
    // Complex numbers as tuples
    {
        let value: (f64, f64) = (2.5, 3.5);
        let a: Mutex<(f64, f64)> = Mutex::new((0.0, 0.0));
        let b: Mutex<(f64, f64)> = Mutex::new(value);
        if *a.lock().unwrap() != (0.0, 0.0) { panic!("abort"); }
        if *b.lock().unwrap() != value { panic!("abort"); }
        *a.lock().unwrap() = *b.lock().unwrap();
        if *a.lock().unwrap() != value { panic!("abort"); }
    }
    
    // Pointer tests using Option<usize>
    {
        let a: Mutex<Option<usize>> = Mutex::new(None);
        let b: Mutex<Option<usize>> = Mutex::new(None);
        if *a.lock().unwrap() != None { panic!("abort"); }
        if *b.lock().unwrap() != None { panic!("abort"); }
        *a.lock().unwrap() = *b.lock().unwrap();
        if *a.lock().unwrap() != None { panic!("abort"); }
    }
    {
        let i: i32 = 0;
        let addr = &i as *const i32 as usize;
        let a: Mutex<Option<usize>> = Mutex::new(None);
        let b: Mutex<Option<usize>> = Mutex::new(Some(addr));
        if *a.lock().unwrap() != None { panic!("abort"); }
        if *b.lock().unwrap() != Some(addr) { panic!("abort"); }
        *a.lock().unwrap() = *b.lock().unwrap();
        if *a.lock().unwrap() != Some(addr) { panic!("abort"); }
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
    if init != copy { panic!("abort"); }
    
    *s2.lock().unwrap() = *s1.lock().unwrap();
    let copy = *s2.lock().unwrap();
    if init != copy { panic!("abort"); }
    
    let copy = *s1.lock().unwrap();
    if init != copy { panic!("abort"); }
    
    let copy = *s2.lock().unwrap();
    if init != copy { panic!("abort"); }
}

fn main() {
    test_simple_assign();
    exit(0);
}