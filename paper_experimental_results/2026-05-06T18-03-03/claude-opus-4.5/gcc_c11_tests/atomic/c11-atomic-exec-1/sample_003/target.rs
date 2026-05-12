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
        let cast_value: $value_type = $value as $value_type;
        B.store(cast_value, Ordering::SeqCst);
        
        if A.load(Ordering::SeqCst) != 0 {
            panic!("abort");
        }
        if B.load(Ordering::SeqCst) != cast_value {
            panic!("abort");
        }
        let b_loaded = B.load(Ordering::SeqCst);
        A.store(b_loaded, Ordering::SeqCst);
        if A.load(Ordering::SeqCst) != cast_value {
            panic!("abort");
        }
        if A.load(Ordering::SeqCst) != cast_value {
            panic!("abort");
        }
    }};
}

macro_rules! test_simple_assign_mutex {
    ($value_type:ty, $value:expr, $default:expr) => {{
        let a: Mutex<$value_type> = Mutex::new($default);
        let cast_value: $value_type = $value;
        let b: Mutex<$value_type> = Mutex::new(cast_value);
        
        if *a.lock().unwrap() != $default {
            panic!("abort");
        }
        if *b.lock().unwrap() != cast_value {
            panic!("abort");
        }
        *a.lock().unwrap() = *b.lock().unwrap();
        if *a.lock().unwrap() != cast_value {
            panic!("abort");
        }
        if *a.lock().unwrap() != cast_value {
            panic!("abort");
        }
    }};
}

fn test_simple_assign_arith(value: f64) {
    // bool
    {
        let a: Mutex<bool> = Mutex::new(false);
        let cast_value: bool = value != 0.0;
        let b: Mutex<bool> = Mutex::new(cast_value);
        if *a.lock().unwrap() != false { panic!("abort"); }
        if *b.lock().unwrap() != cast_value { panic!("abort"); }
        *a.lock().unwrap() = *b.lock().unwrap();
        if *a.lock().unwrap() != cast_value { panic!("abort"); }
    }
    
    test_simple_assign_mutex!(i8, value as i8, 0);
    test_simple_assign_mutex!(u8, value as u8, 0);
    test_simple_assign_mutex!(i16, value as i16, 0);
    test_simple_assign_mutex!(u16, value as u16, 0);
    test_simple_assign_mutex!(i32, value as i32, 0);
    test_simple_assign_mutex!(u32, value as u32, 0);
    test_simple_assign_mutex!(i64, value as i64, 0);
    test_simple_assign_mutex!(u64, value as u64, 0);
    test_simple_assign_mutex!(f32, value as f32, 0.0);
    test_simple_assign_mutex!(f64, value, 0.0);
}

fn test_simple_assign() {
    test_simple_assign_arith(0.0);
    test_simple_assign_arith(1.0);
    test_simple_assign_arith(2.0);
    test_simple_assign_arith(-1.0);
    test_simple_assign_arith((1u64 << 63) as f64);
    test_simple_assign_arith(1.5);
    
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
        let a: Mutex<Option<usize>> = Mutex::new(None);
        let b: Mutex<Option<usize>> = Mutex::new(Some(42));
        if *a.lock().unwrap() != None { panic!("abort"); }
        if *b.lock().unwrap() != Some(42) { panic!("abort"); }
        *a.lock().unwrap() = *b.lock().unwrap();
        if *a.lock().unwrap() != Some(42) { panic!("abort"); }
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