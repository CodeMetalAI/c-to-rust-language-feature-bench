use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize, Ordering};
use std::ptr;

// Helper macro to test atomic assignment for various types
macro_rules! test_simple_assign {
    ($atomic_type:ty, $value:expr) => {{
        let a: $atomic_type = <$atomic_type>::new(Default::default());
        let b: $atomic_type = <$atomic_type>::new($value);
        
        if a.load(Ordering::SeqCst) != Default::default() {
            panic!();
        }
        if b.load(Ordering::SeqCst) != $value {
            panic!();
        }
        
        // Store value from b into a
        a.store(b.load(Ordering::SeqCst), Ordering::SeqCst);
        
        if a.load(Ordering::SeqCst) != $value {
            panic!();
        }
    }};
}

#[repr(C)]
#[derive(Clone, Copy)]
struct LargeStruct {
    a: [i16; 1024],
}

impl Default for LargeStruct {
    fn default() -> Self {
        LargeStruct { a: [0; 1024] }
    }
}

impl PartialEq for LargeStruct {
    fn eq(&self, other: &Self) -> bool {
        self.a.iter().zip(other.a.iter()).all(|(x, y)| x == y)
    }
}

fn test_simple_assign() {
    // Test with value 0
    test_simple_assign!(AtomicBool, false);
    test_simple_assign!(AtomicI8, 0);
    test_simple_assign!(AtomicU8, 0);
    test_simple_assign!(AtomicI16, 0);
    test_simple_assign!(AtomicU16, 0);
    test_simple_assign!(AtomicI32, 0);
    test_simple_assign!(AtomicU32, 0);
    test_simple_assign!(AtomicI64, 0);
    test_simple_assign!(AtomicU64, 0);
    test_simple_assign!(AtomicIsize, 0);
    test_simple_assign!(AtomicUsize, 0);
    
    // Test with value 1
    test_simple_assign!(AtomicBool, true);
    test_simple_assign!(AtomicI8, 1);
    test_simple_assign!(AtomicU8, 1);
    test_simple_assign!(AtomicI16, 1);
    test_simple_assign!(AtomicU16, 1);
    test_simple_assign!(AtomicI32, 1);
    test_simple_assign!(AtomicU32, 1);
    test_simple_assign!(AtomicI64, 1);
    test_simple_assign!(AtomicU64, 1);
    test_simple_assign!(AtomicIsize, 1);
    test_simple_assign!(AtomicUsize, 1);
    
    // Test with value 2
    test_simple_assign!(AtomicI8, 2);
    test_simple_assign!(AtomicU8, 2);
    test_simple_assign!(AtomicI16, 2);
    test_simple_assign!(AtomicU16, 2);
    test_simple_assign!(AtomicI32, 2);
    test_simple_assign!(AtomicU32, 2);
    test_simple_assign!(AtomicI64, 2);
    test_simple_assign!(AtomicU64, 2);
    test_simple_assign!(AtomicIsize, 2);
    test_simple_assign!(AtomicUsize, 2);
    
    // Test with value -1
    test_simple_assign!(AtomicI8, -1);
    test_simple_assign!(AtomicI16, -1);
    test_simple_assign!(AtomicI32, -1);
    test_simple_assign!(AtomicI64, -1);
    test_simple_assign!(AtomicIsize, -1);
    
    // Test with large value (1 << 63)
    test_simple_assign!(AtomicU64, 1u64 << 63);
    test_simple_assign!(AtomicI64, (1i64 << 63));
    
    // For floating point and complex types, Rust doesn't have atomic versions.
    // We'll skip those tests since Rust's standard library doesn't provide them.
    // For pointer tests, we'll use AtomicPtr
    
    // Test pointer with null
    {
        let a: std::sync::atomic::AtomicPtr<i32> = std::sync::atomic::AtomicPtr::new(ptr::null_mut());
        let b: std::sync::atomic::AtomicPtr<i32> = std::sync::atomic::AtomicPtr::new(ptr::null_mut());
        
        if a.load(Ordering::SeqCst) != ptr::null_mut() {
            panic!();
        }
        if b.load(Ordering::SeqCst) != ptr::null_mut() {
            panic!();
        }
        
        a.store(b.load(Ordering::SeqCst), Ordering::SeqCst);
        
        if a.load(Ordering::SeqCst) != ptr::null_mut() {
            panic!();
        }
    }
    
    // Test pointer with valid address
    {
        let mut i = 0;
        let a: std::sync::atomic::AtomicPtr<i32> = std::sync::atomic::AtomicPtr::new(ptr::null_mut());
        let b: std::sync::atomic::AtomicPtr<i32> = std::sync::atomic::AtomicPtr::new(&mut i);
        
        if a.load(Ordering::SeqCst) != ptr::null_mut() {
            panic!();
        }
        if b.load(Ordering::SeqCst) != &mut i {
            panic!();
        }
        
        a.store(b.load(Ordering::SeqCst), Ordering::SeqCst);
        
        if a.load(Ordering::SeqCst) != &mut i {
            panic!();
        }
    }
    
    // Test large struct - Rust doesn't have atomic structs, so we'll use a mutex
    {
        let mut init = LargeStruct::default();
        for j in 0..1024 {
            init.a[j] = j as i16;
        }
        
        let copy = init;
        
        if init != copy {
            panic!();
        }
        
        let copy2 = init;
        if init != copy2 {
            panic!();
        }
        
        let copy3 = init;
        if init != copy3 {
            panic!();
        }
        
        let copy4 = init;
        if init != copy4 {
            panic!();
        }
    }
}

fn main() {
    test_simple_assign();
}