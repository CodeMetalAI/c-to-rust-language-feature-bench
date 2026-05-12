use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16,
                       AtomicI32, AtomicU32, AtomicI64, AtomicU64,
                       AtomicIsize, AtomicUsize,
                       AtomicPtr, Ordering};
use std::ptr;

fn abort() {
    std::process::exit(1);
}

macro_rules! test_incdec {
    ($atomic_type:ty, $value:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {{
        static A: $atomic_type = <$atomic_type>::new($value);
        A.store($value, Ordering::SeqCst);
        
        let result = if $pre_p {
            // Pre-increment/decrement
            match stringify!($preop) {
                "++" => A.fetch_add($change, Ordering::SeqCst) + $change,
                "--" => A.fetch_sub(-$change, Ordering::SeqCst) - $change,
                _ => unreachable!(),
            }
        } else {
            // Post-increment/decrement
            let old = A.load(Ordering::SeqCst);
            match stringify!($postop) {
                "++" => {
                    A.fetch_add($change, Ordering::SeqCst);
                    old
                }
                "--" => {
                    A.fetch_sub(-$change, Ordering::SeqCst);
                    old
                }
                _ => unreachable!(),
            }
        };
        
        let expected = if $pre_p {
            $value + $change
        } else {
            $value
        };
        
        if result != expected {
            abort();
        }
        
        let final_val = A.load(Ordering::SeqCst);
        if final_val != $value + $change {
            abort();
        }
    }};
}

macro_rules! test_incdec_arith {
    ($value:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {{
        test_incdec!(AtomicBool, $value != 0, $preop, $postop, $pre_p, $change as u8);
        test_incdec!(AtomicI8, $value as i8, $preop, $postop, $pre_p, $change as i8);
        test_incdec!(AtomicU8, $value as u8, $preop, $postop, $pre_p, $change as u8);
        test_incdec!(AtomicI16, $value as i16, $preop, $postop, $pre_p, $change as i16);
        test_incdec!(AtomicU16, $value as u16, $preop, $postop, $pre_p, $change as u16);
        test_incdec!(AtomicI32, $value as i32, $preop, $postop, $pre_p, $change as i32);
        test_incdec!(AtomicU32, $value as u32, $preop, $postop, $pre_p, $change as u32);
        test_incdec!(AtomicI64, $value as i64, $preop, $postop, $pre_p, $change as i64);
        test_incdec!(AtomicU64, $value as u64, $preop, $postop, $pre_p, $change as u64);
        test_incdec!(AtomicIsize, $value as isize, $preop, $postop, $pre_p, $change as isize);
        test_incdec!(AtomicUsize, $value as usize, $preop, $postop, $pre_p, $change as usize);
    }};
}

macro_rules! test_all_incdec_arith {
    ($value:expr) => {{
        test_incdec_arith!($value, ++, , true, 1);
        test_incdec_arith!($value, --, , true, -1);
        test_incdec_arith!($value, , ++, false, 1);
        test_incdec_arith!($value, , --, false, -1);
    }};
}

fn test_incdec() {
    test_all_incdec_arith!(0);
    test_all_incdec_arith!(1);
    test_all_incdec_arith!(2);
    test_all_incdec_arith!(-1);
    test_all_incdec_arith!(1u64 << 60);
    
    // Note: Floating point atomics aren't available in Rust's standard library.
    // The original C code includes float/double/long double tests, but since
    // Rust doesn't have atomic floats, we skip them as they would require
    // unsafe code or different synchronization primitives.
    
    // Test pointer arithmetic
    let mut ia = [0i32; 2];
    let base_ptr = ia.as_mut_ptr();
    
    // Pre-increment pointer
    {
        static A: AtomicPtr<i32> = AtomicPtr::new(ptr::null_mut());
        A.store(unsafe { base_ptr.offset(1) }, Ordering::SeqCst);
        let result = A.fetch_add(1, Ordering::SeqCst);
        if result != unsafe { base_ptr.offset(1) } {
            abort();
        }
        let final_val = A.load(Ordering::SeqCst);
        if final_val != unsafe { base_ptr.offset(2) } {
            abort();
        }
    }
    
    // Pre-decrement pointer
    {
        static A: AtomicPtr<i32> = AtomicPtr::new(ptr::null_mut());
        A.store(unsafe { base_ptr.offset(1) }, Ordering::SeqCst);
        let result = A.fetch_sub(1, Ordering::SeqCst);
        if result != unsafe { base_ptr.offset(1) } {
            abort();
        }
        let final_val = A.load(Ordering::SeqCst);
        if final_val != base_ptr {
            abort();
        }
    }
    
    // Post-increment pointer
    {
        static A: AtomicPtr<i32> = AtomicPtr::new(ptr::null_mut());
        A.store(unsafe { base_ptr.offset(1) }, Ordering::SeqCst);
        let old = A.load(Ordering::SeqCst);
        A.fetch_add(1, Ordering::SeqCst);
        if old != unsafe { base_ptr.offset(1) } {
            abort();
        }
        let final_val = A.load(Ordering::SeqCst);
        if final_val != unsafe { base_ptr.offset(2) } {
            abort();
        }
    }
    
    // Post-decrement pointer
    {
        static A: AtomicPtr<i32> = AtomicPtr::new(ptr::null_mut());
        A.store(unsafe { base_ptr.offset(1) }, Ordering::SeqCst);
        let old = A.load(Ordering::SeqCst);
        A.fetch_sub(1, Ordering::SeqCst);
        if old != unsafe { base_ptr.offset(1) } {
            abort();
        }
        let final_val = A.load(Ordering::SeqCst);
        if final_val != base_ptr {
            abort();
        }
    }
}

fn main() {
    test_incdec();
    // exit(0) is implicit in Rust when main returns
}