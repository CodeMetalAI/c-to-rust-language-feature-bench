use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::process::exit;

macro_rules! test_simple_assign_atomic {
    ($atomic_type:ty, $value_type:ty, $value:expr) => {{
        static A: $atomic_type = <$atomic_type>::new(0);
        static B: $atomic_type = <$atomic_type>::new(0);
        
        let a_val = A.load(Ordering::SeqCst);
        let b_val = B.load(Ordering::SeqCst);
        
        // Reset for this test
        A.store(0, Ordering::SeqCst);
        let value: $value_type = $value;
        B.store(value, Ordering::SeqCst);
        
        if A.load(Ordering::SeqCst) != 0 {
            panic!("abort");
        }
        if B.load(Ordering::SeqCst) != value {
            panic!("abort");
        }
        let b_loaded = B.load(Ordering::SeqCst);
        A.store(b_loaded, Ordering::SeqCst);
        if A.load(Ordering::SeqCst) != value {
            panic!("abort");
        }
        if A.load(Ordering::SeqCst) != value {
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
        if *b.lock().unwrap() != ($value as $value_type) {
            panic!("abort");
        }
        let b_val = *b.lock().unwrap();
        *a.lock().unwrap() = b_val;
        if b_val != ($value as $value_type) {
            panic!("abort");
        }
        if *a.lock().unwrap() != ($value as $value_type) {
            panic!("abort");
        }
    }};
}

macro_rules! test_simple_assign_arith {
    ($value:expr) => {{
        // bool
        {
            let val: bool = $value != 0.0;
            let a: Mutex<bool> = Mutex::new(false);
            let b: Mutex<bool> = Mutex::new(val);
            if *a.lock().unwrap() != false { panic!("abort"); }
            if *b.lock().unwrap() != val { panic!("abort"); }
            *a.lock().unwrap() = *b.lock().unwrap();
            if *a.lock().unwrap() != val { panic!("abort"); }
        }
        // Various integer and float types using Mutex for simplicity
        test_simple_assign_mutex!(i8, $value as i8, 0i8);
        test_simple_assign_mutex!(u8, $value as u8, 0u8);
        test_simple_assign_mutex!(i16, $value as i16, 0i16);
        test_simple_assign_mutex!(u16, $value as u16, 0u16);
        test_simple_assign_mutex!(i32, $value as i32, 0i32);
        test_simple_assign_mutex!(u32, $value as u32, 0u32);
        test_simple_assign_mutex!(i64, $value as i64, 0i64);
        test_simple_assign_mutex!(u64, $value as u64, 0u64);
        test_simple_assign_mutex!(f32, $value as f32, 0.0f32);
        test_simple_assign_mutex!(f64, $value as f64, 0.0f64);
    }};
}

fn test_simple_assign() {
    test_simple_assign_arith!(0.0);
    test_simple_assign_arith!(1.0);
    test_simple_assign_arith!(2.0);
    test_simple_assign_arith!(-1.0);
    test_simple_assign_arith!((1u64 << 63) as f64);
    test_simple_assign_arith!(1.5);
    
    // Test with pointer-like behavior using Option<usize>
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
        let a: Mutex<usize> = Mutex::new(0);
        let b: Mutex<usize> = Mutex::new(addr);
        *a.lock().unwrap() = *b.lock().unwrap();
        if *a.lock().unwrap() != addr { panic!("abort"); }
    }
    
    // Test with struct
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