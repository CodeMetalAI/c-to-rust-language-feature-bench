use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::process;

fn test_incdec() {
    // Test with value 0
    test_all_incdec_arith(0);
    
    // Test with value 1
    test_all_incdec_arith(1);
    
    // Test with value 2
    test_all_incdec_arith(2);
    
    // Test with value -1
    test_all_incdec_arith(-1);
    
    // Test with value 1ULL << 60
    test_all_incdec_arith(1u64 << 60);
    
    // Test with value 1.5
    test_all_incdec_arith_float(1.5);
    
    // Pointer tests (using indices instead)
    test_incdec_ptr();
}

fn test_all_incdec_arith(value: i64) {
    test_incdec_arith(value, true, 1);   // pre-increment
    test_incdec_arith(value, true, -1);  // pre-decrement
    test_incdec_arith(value, false, 1);  // post-increment
    test_incdec_arith(value, false, -1); // post-decrement
}

fn test_all_incdec_arith_float(value: f64) {
    test_incdec_arith_float_impl(value, true, 1.0);   // pre-increment
    test_incdec_arith_float_impl(value, true, -1.0);  // pre-decrement
    test_incdec_arith_float_impl(value, false, 1.0);  // post-increment
    test_incdec_arith_float_impl(value, false, -1.0); // post-decrement
}

fn test_incdec_arith(value: i64, pre_p: bool, change: i64) {
    // bool
    test_incdec_bool(value != 0, pre_p, change);
    
    // signed types
    test_incdec_i8(value as i8, pre_p, change as i8);
    test_incdec_i16(value as i16, pre_p, change as i16);
    test_incdec_i32(value as i32, pre_p, change as i32);
    test_incdec_i64(value, pre_p, change);
    
    // unsigned types
    test_incdec_u8(value as u8, pre_p, change as i8 as u8);
    test_incdec_u16(value as u16, pre_p, change as i16 as u16);
    test_incdec_u32(value as u32, pre_p, change as i32 as u32);
    test_incdec_u64(value as u64, pre_p, change as u64);
}

fn test_incdec_bool(value: bool, pre_p: bool, change: i64) {
    let a = AtomicBool::new(value);
    let result = if change > 0 {
        if pre_p {
            a.fetch_or(true, Ordering::SeqCst) || true
        } else {
            a.fetch_or(true, Ordering::SeqCst)
        }
    } else {
        if pre_p {
            a.fetch_and(false, Ordering::SeqCst) && false
        } else {
            a.fetch_and(false, Ordering::SeqCst)
        }
    };
    
    let expected = if pre_p {
        (value as i64 + change) != 0
    } else {
        value
    };
    
    if result != expected {
        process::abort();
    }
    
    let final_val = a.load(Ordering::SeqCst);
    if final_val != ((value as i64 + change) != 0) {
        process::abort();
    }
}

macro_rules! test_incdec_int {
    ($name:ident, $atomic_type:ty, $type:ty) => {
        fn $name(value: $type, pre_p: bool, change: $type) {
            let a = <$atomic_type>::new(value);
            let result = if change > 0 {
                if pre_p {
                    a.fetch_add(change, Ordering::SeqCst).wrapping_add(change)
                } else {
                    a.fetch_add(change, Ordering::SeqCst)
                }
            } else {
                if pre_p {
                    a.fetch_sub(change.wrapping_neg(), Ordering::SeqCst).wrapping_sub(change.wrapping_neg())
                } else {
                    a.fetch_sub(change.wrapping_neg(), Ordering::SeqCst)
                }
            };
            
            let expected = if pre_p {
                value.wrapping_add(change)
            } else {
                value
            };
            
            if result != expected {
                process::abort();
            }
            
            let final_val = a.load(Ordering::SeqCst);
            if final_val != value.wrapping_add(change) {
                process::abort();
            }
        }
    };
}

test_incdec_int!(test_incdec_i8, AtomicI8, i8);
test_incdec_int!(test_incdec_i16, AtomicI16, i16);
test_incdec_int!(test_incdec_i32, AtomicI32, i32);
test_incdec_int!(test_incdec_i64, AtomicI64, i64);
test_incdec_int!(test_incdec_u8, AtomicU8, u8);
test_incdec_int!(test_incdec_u16, AtomicU16, u16);
test_incdec_int!(test_incdec_u32, AtomicU32, u32);
test_incdec_int!(test_incdec_u64, AtomicU64, u64);

fn test_incdec_arith_float_impl(value: f64, pre_p: bool, change: f64) {
    // Simulate atomic float operations using compare-and-swap
    let a = AtomicU64::new(value.to_bits());
    
    let old_bits = a.load(Ordering::SeqCst);
    let old_val = f64::from_bits(old_bits);
    let new_val = old_val + change;
    a.store(new_val.to_bits(), Ordering::SeqCst);
    
    let result = if pre_p { new_val } else { old_val };
    let expected = if pre_p { value + change } else { value };
    
    if result != expected {
        process::abort();
    }
    
    let final_val = f64::from_bits(a.load(Ordering::SeqCst));
    if final_val != value + change {
        process::abort();
    }
}

fn test_incdec_ptr() {
    let a = AtomicUsize::new(1);
    
    // pre-increment
    let result = a.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 2 {
        process::abort();
    }
    if a.load(Ordering::SeqCst) != 2 {
        process::abort();
    }
    
    a.store(1, Ordering::SeqCst);
    
    // pre-decrement
    let result = a.fetch_sub(1, Ordering::SeqCst) - 1;
    if result != 0 {
        process::abort();
    }
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    
    a.store(1, Ordering::SeqCst);
    
    // post-increment
    let result = a.fetch_add(1, Ordering::SeqCst);
    if result != 1 {
        process::abort();
    }
    if a.load(Ordering::SeqCst) != 2 {
        process::abort();
    }
    
    a.store(1, Ordering::SeqCst);
    
    // post-decrement
    let result = a.fetch_sub(1, Ordering::SeqCst);
    if result != 1 {
        process::abort();
    }
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
}

fn main() {
    test_incdec();
    process::exit(0);
}