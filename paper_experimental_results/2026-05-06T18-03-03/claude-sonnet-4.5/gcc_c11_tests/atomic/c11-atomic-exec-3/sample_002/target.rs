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
    test_all_incdec_arith(1i64 << 60);
    
    // Test with value 1.5 (floats)
    test_all_incdec_arith_float(1.5);
    
    // Pointer arithmetic tests (using indices instead)
    test_incdec_ptr();
}

fn test_all_incdec_arith(value: i64) {
    // Pre-increment tests (++a)
    test_incdec_bool(value, true, 1);
    test_incdec_i8(value, true, 1);
    test_incdec_u8(value, true, 1);
    test_incdec_i16(value, true, 1);
    test_incdec_u16(value, true, 1);
    test_incdec_i32(value, true, 1);
    test_incdec_u32(value, true, 1);
    test_incdec_i64(value, true, 1);
    test_incdec_u64(value, true, 1);
    test_incdec_isize(value, true, 1);
    test_incdec_usize(value, true, 1);
    
    // Pre-decrement tests (--a)
    test_incdec_bool(value, true, -1);
    test_incdec_i8(value, true, -1);
    test_incdec_u8(value, true, -1);
    test_incdec_i16(value, true, -1);
    test_incdec_u16(value, true, -1);
    test_incdec_i32(value, true, -1);
    test_incdec_u32(value, true, -1);
    test_incdec_i64(value, true, -1);
    test_incdec_u64(value, true, -1);
    test_incdec_isize(value, true, -1);
    test_incdec_usize(value, true, -1);
    
    // Post-increment tests (a++)
    test_incdec_bool(value, false, 1);
    test_incdec_i8(value, false, 1);
    test_incdec_u8(value, false, 1);
    test_incdec_i16(value, false, 1);
    test_incdec_u16(value, false, 1);
    test_incdec_i32(value, false, 1);
    test_incdec_u32(value, false, 1);
    test_incdec_i64(value, false, 1);
    test_incdec_u64(value, false, 1);
    test_incdec_isize(value, false, 1);
    test_incdec_usize(value, false, 1);
    
    // Post-decrement tests (a--)
    test_incdec_bool(value, false, -1);
    test_incdec_i8(value, false, -1);
    test_incdec_u8(value, false, -1);
    test_incdec_i16(value, false, -1);
    test_incdec_u16(value, false, -1);
    test_incdec_i32(value, false, -1);
    test_incdec_u32(value, false, -1);
    test_incdec_i64(value, false, -1);
    test_incdec_u64(value, false, -1);
    test_incdec_isize(value, false, -1);
    test_incdec_usize(value, false, -1);
}

fn test_all_incdec_arith_float(value: f64) {
    test_incdec_f32(value, true, 1.0);
    test_incdec_f64(value, true, 1.0);
    
    test_incdec_f32(value, true, -1.0);
    test_incdec_f64(value, true, -1.0);
    
    test_incdec_f32(value, false, 1.0);
    test_incdec_f64(value, false, 1.0);
    
    test_incdec_f32(value, false, -1.0);
    test_incdec_f64(value, false, -1.0);
}

fn test_incdec_bool(value: i64, pre: bool, change: i64) {
    let a = AtomicBool::new(value != 0);
    let expected_val = if value != 0 { 1u8 } else { 0u8 };
    let new_val = expected_val.wrapping_add(change as u8);
    
    let result = if pre {
        a.store(new_val != 0, Ordering::SeqCst);
        new_val != 0
    } else {
        let old = a.load(Ordering::SeqCst);
        a.store(new_val != 0, Ordering::SeqCst);
        old
    };
    
    let expected_result = if pre { new_val != 0 } else { expected_val != 0 };
    if result != expected_result {
        process::abort();
    }
    if a.load(Ordering::SeqCst) != (new_val != 0) {
        process::abort();
    }
}

macro_rules! test_incdec_signed {
    ($name:ident, $atomic_type:ty, $value_type:ty) => {
        fn $name(value: i64, pre: bool, change: i64) {
            let a = <$atomic_type>::new(value as $value_type);
            let expected_val = value as $value_type;
            let new_val = expected_val.wrapping_add(change as $value_type);
            
            let result = if pre {
                a.fetch_add(change as $value_type, Ordering::SeqCst).wrapping_add(change as $value_type)
            } else {
                a.fetch_add(change as $value_type, Ordering::SeqCst)
            };
            
            let expected_result = if pre { new_val } else { expected_val };
            if result != expected_result {
                process::abort();
            }
            if a.load(Ordering::SeqCst) != new_val {
                process::abort();
            }
        }
    };
}

macro_rules! test_incdec_unsigned {
    ($name:ident, $atomic_type:ty, $value_type:ty) => {
        fn $name(value: i64, pre: bool, change: i64) {
            let a = <$atomic_type>::new(value as $value_type);
            let expected_val = value as $value_type;
            let new_val = expected_val.wrapping_add(change as $value_type);
            
            let result = if pre {
                a.fetch_add(change as $value_type, Ordering::SeqCst).wrapping_add(change as $value_type)
            } else {
                a.fetch_add(change as $value_type, Ordering::SeqCst)
            };
            
            let expected_result = if pre { new_val } else { expected_val };
            if result != expected_result {
                process::abort();
            }
            if a.load(Ordering::SeqCst) != new_val {
                process::abort();
            }
        }
    };
}

test_incdec_signed!(test_incdec_i8, AtomicI8, i8);
test_incdec_unsigned!(test_incdec_u8, AtomicU8, u8);
test_incdec_signed!(test_incdec_i16, AtomicI16, i16);
test_incdec_unsigned!(test_incdec_u16, AtomicU16, u16);
test_incdec_signed!(test_incdec_i32, AtomicI32, i32);
test_incdec_unsigned!(test_incdec_u32, AtomicU32, u32);
test_incdec_signed!(test_incdec_i64, AtomicI64, i64);
test_incdec_unsigned!(test_incdec_u64, AtomicU64, u64);
test_incdec_signed!(test_incdec_isize, AtomicIsize, isize);
test_incdec_unsigned!(test_incdec_usize, AtomicUsize, usize);

fn test_incdec_f32(value: f64, pre: bool, change: f64) {
    let a = AtomicU32::new((value as f32).to_bits());
    let expected_val = value as f32;
    let new_val = expected_val + change as f32;
    
    let result = if pre {
        a.store(new_val.to_bits(), Ordering::SeqCst);
        new_val
    } else {
        let old = f32::from_bits(a.load(Ordering::SeqCst));
        a.store(new_val.to_bits(), Ordering::SeqCst);
        old
    };
    
    let expected_result = if pre { new_val } else { expected_val };
    if result != expected_result {
        process::abort();
    }
    if f32::from_bits(a.load(Ordering::SeqCst)) != new_val {
        process::abort();
    }
}

fn test_incdec_f64(value: f64, pre: bool, change: f64) {
    let a = AtomicU64::new(value.to_bits());
    let expected_val = value;
    let new_val = expected_val + change;
    
    let result = if pre {
        a.store(new_val.to_bits(), Ordering::SeqCst);
        new_val
    } else {
        let old = f64::from_bits(a.load(Ordering::SeqCst));
        a.store(new_val.to_bits(), Ordering::SeqCst);
        old
    };
    
    let expected_result = if pre { new_val } else { expected_val };
    if result != expected_result {
        process::abort();
    }
    if f64::from_bits(a.load(Ordering::SeqCst)) != new_val {
        process::abort();
    }
}

fn test_incdec_ptr() {
    // Pre-increment
    let a = AtomicUsize::new(1);
    let result = a.fetch_add(1, Ordering::SeqCst) + 1;
    if result != 2 {
        process::abort();
    }
    if a.load(Ordering::SeqCst) != 2 {
        process::abort();
    }
    
    // Pre-decrement
    let a = AtomicUsize::new(1);
    let result = a.fetch_sub(1, Ordering::SeqCst).wrapping_sub(1);
    if result != 0 {
        process::abort();
    }
    if a.load(Ordering::SeqCst) != 0 {
        process::abort();
    }
    
    // Post-increment
    let a = AtomicUsize::new(1);
    let result = a.fetch_add(1, Ordering::SeqCst);
    if result != 1 {
        process::abort();
    }
    if a.load(Ordering::SeqCst) != 2 {
        process::abort();
    }
    
    // Post-decrement
    let a = AtomicUsize::new(1);
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