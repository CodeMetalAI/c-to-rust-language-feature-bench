use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;

macro_rules! test_simple_assign_atomic {
    ($atomic_type:ty, $value_type:ty, $value:expr, $zero:expr) => {{
        static A: $atomic_type = <$atomic_type>::new($zero);
        static B: $atomic_type = <$atomic_type>::new($zero);
        
        let a_val = A.load(Ordering::SeqCst);
        let b_init: $value_type = $value;
        B.store(b_init, Ordering::SeqCst);
        
        if a_val != $zero {
            std::process::exit(1);
        }
        let b_val = B.load(Ordering::SeqCst);
        if b_val != b_init {
            std::process::exit(1);
        }
        A.store(b_val, Ordering::SeqCst);
        let a_after = A.load(Ordering::SeqCst);
        if a_after != b_init {
            std::process::exit(1);
        }
        if A.load(Ordering::SeqCst) != b_init {
            std::process::exit(1);
        }
        // Reset for next call
        A.store($zero, Ordering::SeqCst);
        B.store($zero, Ordering::SeqCst);
    }};
}

macro_rules! test_simple_assign_mutex {
    ($value_type:ty, $value:expr, $zero:expr) => {{
        let a: Mutex<$value_type> = Mutex::new($zero);
        let b: Mutex<$value_type> = Mutex::new($value);
        
        if *a.lock().unwrap() != $zero {
            std::process::exit(1);
        }
        if *b.lock().unwrap() != $value {
            std::process::exit(1);
        }
        let b_val = *b.lock().unwrap();
        *a.lock().unwrap() = b_val;
        if *a.lock().unwrap() != $value {
            std::process::exit(1);
        }
        if *a.lock().unwrap() != $value {
            std::process::exit(1);
        }
    }};
}

fn test_simple_assign_arith(value: i64) {
    // bool
    let bool_val = value != 0;
    test_simple_assign_mutex!(bool, bool_val, false);
    
    // i8
    test_simple_assign_mutex!(i8, value as i8, 0i8);
    
    // u8
    test_simple_assign_mutex!(u8, value as u8, 0u8);
    
    // i16
    test_simple_assign_mutex!(i16, value as i16, 0i16);
    
    // u16
    test_simple_assign_mutex!(u16, value as u16, 0u16);
    
    // i32
    test_simple_assign_mutex!(i32, value as i32, 0i32);
    
    // u32
    test_simple_assign_mutex!(u32, value as u32, 0u32);
    
    // i64
    test_simple_assign_mutex!(i64, value, 0i64);
    
    // u64
    test_simple_assign_mutex!(u64, value as u64, 0u64);
    
    // f32
    test_simple_assign_mutex!(f32, value as f32, 0.0f32);
    
    // f64
    test_simple_assign_mutex!(f64, value as f64, 0.0f64);
}

fn test_simple_assign_arith_float(value: f64) {
    test_simple_assign_mutex!(f32, value as f32, 0.0f32);
    test_simple_assign_mutex!(f64, value, 0.0f64);
}

fn test_simple_assign_arith_complex(re: f64, im: f64) {
    test_simple_assign_mutex!((f32, f32), (re as f32, im as f32), (0.0f32, 0.0f32));
    test_simple_assign_mutex!((f64, f64), (re, im), (0.0f64, 0.0f64));
}

fn test_simple_assign() {
    test_simple_assign_arith(0);
    test_simple_assign_arith(1);
    test_simple_assign_arith(2);
    test_simple_assign_arith(-1);
    test_simple_assign_arith(1i64 << 63);
    test_simple_assign_arith_float(1.5);
    test_simple_assign_arith_complex(2.5, 3.5);
    
    // Pointer tests - using Option<usize> to simulate
    test_simple_assign_mutex!(Option<usize>, None, None);
    let i: i32 = 0;
    let i_addr = &i as *const i32 as usize;
    test_simple_assign_mutex!(Option<usize>, Some(i_addr), None);
    
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
        std::process::exit(1);
    }
    
    let s1_val = *s1.lock().unwrap();
    *s2.lock().unwrap() = s1_val;
    let copy = *s2.lock().unwrap();
    if init != copy {
        std::process::exit(1);
    }
    
    let copy = *s1.lock().unwrap();
    if init != copy {
        std::process::exit(1);
    }
    
    let copy = *s2.lock().unwrap();
    if init != copy {
        std::process::exit(1);
    }
}

fn main() {
    test_simple_assign();
    std::process::exit(0);
}