use std::mem;
use std::process;

fn abort() {
    process::exit(1);
}

macro_rules! test_simple_assign {
    ($type:ty, $value:expr) => {
        {
            static mut A: std::sync::atomic::AtomicPtr<()> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
            static mut B: std::sync::atomic::AtomicPtr<()> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
            
            // We'll use atomics for numeric types and pointers, but for complex types
            // we need a different approach since Rust doesn't have atomic complex numbers.
            // We'll handle this differently in the actual implementation.
        }
    };
}

fn test_simple_assign() {
    // Test basic numeric types with atomic operations
    {
        use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, 
                               AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize,
                               Ordering};
        
        // Test with value 0
        let a_bool = AtomicBool::new(false);
        let b_bool = AtomicBool::new(false);
        assert_eq!(a_bool.load(Ordering::SeqCst), false);
        assert_eq!(b_bool.load(Ordering::SeqCst), false);
        a_bool.store(b_bool.load(Ordering::SeqCst), Ordering::SeqCst);
        assert_eq!(a_bool.load(Ordering::SeqCst), false);
        
        // Test with value 1
        let a_bool = AtomicBool::new(false);
        let b_bool = AtomicBool::new(true);
        assert_eq!(a_bool.load(Ordering::SeqCst), false);
        assert_eq!(b_bool.load(Ordering::SeqCst), true);
        a_bool.store(b_bool.load(Ordering::SeqCst), Ordering::SeqCst);
        assert_eq!(a_bool.load(Ordering::SeqCst), true);
        
        // Test integer types with various values
        let test_values = [0i32, 1, 2, -1];
        for &val in &test_values {
            let a_i32 = AtomicI32::new(0);
            let b_i32 = AtomicI32::new(val);
            assert_eq!(a_i32.load(Ordering::SeqCst), 0);
            assert_eq!(b_i32.load(Ordering::SeqCst), val);
            a_i32.store(b_i32.load(Ordering::SeqCst), Ordering::SeqCst);
            assert_eq!(a_i32.load(Ordering::SeqCst), val);
        }
        
        // Test with large value (1 << 63 for i64)
        let a_i64 = AtomicI64::new(0);
        let b_i64 = AtomicI64::new(1i64 << 63);
        assert_eq!(a_i64.load(Ordering::SeqCst), 0);
        assert_eq!(b_i64.load(Ordering::SeqCst), 1i64 << 63);
        a_i64.store(b_i64.load(Ordering::SeqCst), Ordering::SeqCst);
        assert_eq!(a_i64.load(Ordering::SeqCst), 1i64 << 63);
    }
    
    // Test floating point types (Rust doesn't have atomic floats in std, so we use Mutex)
    {
        use std::sync::Mutex;
        
        let test_float_values = [0.0f32, 1.0, 2.0, -1.0, 1.5];
        for &val in &test_float_values {
            let a_float = Mutex::new(0.0f32);
            let b_float = Mutex::new(val);
            
            assert_eq!(*a_float.lock().unwrap(), 0.0);
            assert_eq!(*b_float.lock().unwrap(), val);
            
            let b_val = *b_float.lock().unwrap();
            *a_float.lock().unwrap() = b_val;
            assert_eq!(*a_float.lock().unwrap(), val);
        }
        
        // Test double precision
        for &val in &[0.0f64, 1.0, 2.0, -1.0, 1.5] {
            let a_double = Mutex::new(0.0f64);
            let b_double = Mutex::new(val);
            
            assert_eq!(*a_double.lock().unwrap(), 0.0);
            assert_eq!(*b_double.lock().unwrap(), val);
            
            let b_val = *b_double.lock().unwrap();
            *a_double.lock().unwrap() = b_val;
            assert_eq!(*a_double.lock().unwrap(), val);
        }
    }
    
    // Test complex numbers (using tuples as complex numbers)
    {
        use std::sync::Mutex;
        
        let complex_val = (2.5f32, 3.5f32);
        let a_complex = Mutex::new((0.0f32, 0.0f32));
        let b_complex = Mutex::new(complex_val);
        
        assert_eq!(*a_complex.lock().unwrap(), (0.0, 0.0));
        assert_eq!(*b_complex.lock().unwrap(), complex_val);
        
        let b_val = *b_complex.lock().unwrap();
        *a_complex.lock().unwrap() = b_val;
        assert_eq!(*a_complex.lock().unwrap(), complex_val);
    }
    
    // Test pointers
    {
        use std::sync::atomic::{AtomicPtr, Ordering};
        
        let mut i = 0;
        let a_ptr: AtomicPtr<i32> = AtomicPtr::new(std::ptr::null_mut());
        let b_ptr: AtomicPtr<i32> = AtomicPtr::new(std::ptr::null_mut());
        
        assert!(a_ptr.load(Ordering::SeqCst).is_null());
        assert!(b_ptr.load(Ordering::SeqCst).is_null());
        
        a_ptr.store(b_ptr.load(Ordering::SeqCst), Ordering::SeqCst);
        assert!(a_ptr.load(Ordering::SeqCst).is_null());
        
        let b_ptr2: AtomicPtr<i32> = AtomicPtr::new(&mut i as *mut i32);
        assert!(!b_ptr2.load(Ordering::SeqCst).is_null());
    }
    
    // Test struct assignment (using Mutex for atomicity)
    {
        use std::sync::Mutex;
        
        #[repr(C)]
        #[derive(Clone, Copy)]
        struct S {
            a: [i16; 1024],
        }
        
        impl PartialEq for S {
            fn eq(&self, other: &Self) -> bool {
                self.a.iter().zip(other.a.iter()).all(|(x, y)| x == y)
            }
        }
        
        let mut init = S { a: [0; 1024] };
        for j in 0..1024 {
            init.a[j] = j as i16;
        }
        
        let s1 = Mutex::new(S { a: [0; 1024] });
        let s2 = Mutex::new(S { a: [0; 1024] });
        
        // s1 = init
        *s1.lock().unwrap() = init;
        let copy1 = *s1.lock().unwrap();
        if copy1 != init {
            abort();
        }
        
        // s2 = s1
        let s1_val = *s1.lock().unwrap();
        *s2.lock().unwrap() = s1_val;
        let copy2 = *s2.lock().unwrap();
        if copy2 != init {
            abort();
        }
        
        // copy = s1
        let copy3 = *s1.lock().unwrap();
        if copy3 != init {
            abort();
        }
        
        // copy = s2
        let copy4 = *s2.lock().unwrap();
        if copy4 != init {
            abort();
        }
    }
}

fn main() {
    test_simple_assign();
    // exit(0) is implicit in Rust when main returns
}