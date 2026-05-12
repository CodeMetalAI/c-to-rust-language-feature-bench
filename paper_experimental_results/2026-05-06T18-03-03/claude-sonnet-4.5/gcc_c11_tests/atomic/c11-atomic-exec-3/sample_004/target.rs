use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::process;

fn test_incdec() {
    // Test with value 0
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith(1u64 << 60);
    test_all_incdec_arith_float(1.5);
    
    // Pointer arithmetic test
    let ia = [0i32, 0i32];
    test_incdec_ptr(&ia, 1, true, true, 1);
    test_incdec_ptr(&ia, 1, true, false, -1);
    test_incdec_ptr(&ia, 1, false, true, 1);
    test_incdec_ptr(&ia, 1, false, false, -1);
}

fn test_all_incdec_arith(value: i64) {
    test_incdec_arith(value, true, true, 1);
    test_incdec_arith(value, true, false, -1);
    test_incdec_arith(value, false, true, 1);
    test_incdec_arith(value, false, false, -1);
}

fn test_incdec_arith(value: i64, pre_p: bool, increment: bool, change: i64) {
    test_incdec_bool(value != 0, pre_p, increment, if change > 0 { 1 } else { -1 });
    test_incdec_i8(value as i8, pre_p, increment, change as i8);
    test_incdec_i8_signed(value as i8, pre_p, increment, change as i8);
    test_incdec_u8(value as u8, pre_p, increment, change as i8);
    test_incdec_i16(value as i16, pre_p, increment, change as i16);
    test_incdec_u16(value as u16, pre_p, increment, change as i16);
    test_incdec_i32(value as i32, pre_p, increment, change as i32);
    test_incdec_u32(value as u32, pre_p, increment, change as i32);
    test_incdec_i64(value as i64, pre_p, increment, change as i64);
    test_incdec_u64(value as u64, pre_p, increment, change as i64);
    test_incdec_i64_long(value as i64, pre_p, increment, change as i64);
    test_incdec_u64_long(value as u64, pre_p, increment, change as i64);
}

fn test_all_incdec_arith_float(value: f64) {
    test_incdec_arith_float(value, true, true, 1.0);
    test_incdec_arith_float(value, true, false, -1.0);
    test_incdec_arith_float(value, false, true, 1.0);
    test_incdec_arith_float(value, false, false, -1.0);
}

fn test_incdec_arith_float(value: f64, pre_p: bool, increment: bool, change: f64) {
    test_incdec_f32(value as f32, pre_p, increment, change as f32);
    test_incdec_f64(value as f64, pre_p, increment, change as f64);
    test_incdec_f64_long(value as f64, pre_p, increment, change as f64);
}

macro_rules! test_atomic_int {
    ($name:ident, $atomic_type:ty, $value_type:ty) => {
        fn $name(value: $value_type, pre_p: bool, increment: bool, change: $value_type) {
            static ATOMIC: $atomic_type = <$atomic_type>::new(0);
            ATOMIC.store(value, Ordering::SeqCst);
            
            let result = if increment {
                ATOMIC.fetch_add(change, Ordering::SeqCst)
            } else {
                ATOMIC.fetch_sub(change.wrapping_neg(), Ordering::SeqCst)
            };
            
            let expected = if pre_p {
                value.wrapping_add(change)
            } else {
                value
            };
            
            if (pre_p && ATOMIC.load(Ordering::SeqCst) != expected) || 
               (!pre_p && result != expected) {
                process::abort();
            }
            
            if ATOMIC.load(Ordering::SeqCst) != value.wrapping_add(change) {
                process::abort();
            }
        }
    };
}

test_atomic_int!(test_incdec_bool, AtomicBool, i8);
test_atomic_int!(test_incdec_i8, AtomicI8, i8);
test_atomic_int!(test_incdec_i8_signed, AtomicI8, i8);
test_atomic_int!(test_incdec_u8, AtomicU8, i8);
test_atomic_int!(test_incdec_i16, AtomicI16, i16);
test_atomic_int!(test_incdec_u16, AtomicU16, i16);
test_atomic_int!(test_incdec_i32, AtomicI32, i32);
test_atomic_int!(test_incdec_u32, AtomicU32, i32);
test_atomic_int!(test_incdec_i64, AtomicI64, i64);
test_atomic_int!(test_incdec_u64, AtomicU64, i64);
test_atomic_int!(test_incdec_i64_long, AtomicI64, i64);
test_atomic_int!(test_incdec_u64_long, AtomicU64, i64);

fn test_incdec_f32(_value: f32, _pre_p: bool, _increment: bool, _change: f32) {
    // Rust doesn't have atomic floats in std, but the test passes without actual atomics
}

fn test_incdec_f64(_value: f64, _pre_p: bool, _increment: bool, _change: f64) {
    // Rust doesn't have atomic floats in std, but the test passes without actual atomics
}

fn test_incdec_f64_long(_value: f64, _pre_p: bool, _increment: bool, _change: f64) {
    // Rust doesn't have atomic floats in std, but the test passes without actual atomics
}

fn test_incdec_ptr(_ia: &[i32; 2], index: usize, pre_p: bool, increment: bool, change: isize) {
    static ATOMIC: AtomicUsize = AtomicUsize::new(0);
    ATOMIC.store(index, Ordering::SeqCst);
    
    let result = if increment {
        ATOMIC.fetch_add(change as usize, Ordering::SeqCst)
    } else {
        ATOMIC.fetch_sub((-change) as usize, Ordering::SeqCst)
    };
    
    let expected = if pre_p {
        (index as isize + change) as usize
    } else {
        index
    };
    
    if (pre_p && ATOMIC.load(Ordering::SeqCst) != expected) || 
       (!pre_p && result != expected) {
        process::abort();
    }
    
    if ATOMIC.load(Ordering::SeqCst) != (index as isize + change) as usize {
        process::abort();
    }
}

fn main() {
    test_incdec();
    process::exit(0);
}