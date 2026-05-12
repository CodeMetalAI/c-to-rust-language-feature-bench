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
    
    // Pointer tests - using indices instead of raw pointers
    test_incdec_ptr(1, 1, true, 1);   // ++ptr
    test_incdec_ptr(1, -1, true, -1);  // --ptr
    test_incdec_ptr(1, 1, false, 1);   // ptr++
    test_incdec_ptr(1, -1, false, -1); // ptr--
}

fn test_all_incdec_arith(value: i64) {
    test_incdec_arith(value, 1, true, 1);   // ++
    test_incdec_arith(value, -1, true, -1); // --
    test_incdec_arith(value, 1, false, 1);  // postfix ++
    test_incdec_arith(value, -1, false, -1); // postfix --
}

fn test_all_incdec_arith_float(value: f64) {
    test_incdec_float(value, 1.0, true, 1.0);   // ++
    test_incdec_float(value, -1.0, true, -1.0); // --
    test_incdec_float(value, 1.0, false, 1.0);  // postfix ++
    test_incdec_float(value, -1.0, false, -1.0); // postfix --
}

fn test_incdec_arith(value: i64, _op: i64, pre_p: bool, change: i64) {
    // bool
    test_incdec_bool(value != 0, change > 0, pre_p, change > 0);
    
    // i8
    test_incdec_i8(value as i8, change as i8, pre_p, change as i8);
    
    // u8
    test_incdec_u8(value as u8, change as i8, pre_p, change as i8);
    
    // i16
    test_incdec_i16(value as i16, change as i16, pre_p, change as i16);
    
    // u16
    test_incdec_u16(value as u16, change as i16, pre_p, change as i16);
    
    // i32
    test_incdec_i32(value as i32, change as i32, pre_p, change as i32);
    
    // u32
    test_incdec_u32(value as u32, change as i32, pre_p, change as i32);
    
    // i64
    test_incdec_i64(value, change, pre_p, change);
    
    // u64
    test_incdec_u64(value as u64, change, pre_p, change);
    
    // isize
    test_incdec_isize(value as isize, change as isize, pre_p, change as isize);
    
    // usize
    test_incdec_usize(value as usize, change as isize, pre_p, change as isize);
}

fn test_incdec_bool(value: bool, _op: bool, pre_p: bool, change: bool) {
    static ATOMIC: AtomicBool = AtomicBool::new(false);
    ATOMIC.store(value, Ordering::SeqCst);
    
    let result = if pre_p {
        ATOMIC.store(change, Ordering::SeqCst);
        change
    } else {
        let old = ATOMIC.load(Ordering::SeqCst);
        ATOMIC.store(change, Ordering::SeqCst);
        old
    };
    
    let expected = if pre_p { change } else { value };
    if result != expected {
        process::abort();
    }
    if ATOMIC.load(Ordering::SeqCst) != change {
        process::abort();
    }
}

macro_rules! test_incdec_signed {
    ($name:ident, $atomic_type:ty, $value_type:ty) => {
        fn $name(value: $value_type, _op: $value_type, pre_p: bool, change: $value_type) {
            static ATOMIC: $atomic_type = <$atomic_type>::new(0);
            ATOMIC.store(value, Ordering::SeqCst);
            
            let result = if pre_p {
                ATOMIC.fetch_add(change, Ordering::SeqCst).wrapping_add(change)
            } else {
                ATOMIC.fetch_add(change, Ordering::SeqCst)
            };
            
            let expected = if pre_p { value.wrapping_add(change) } else { value };
            if result != expected {
                process::abort();
            }
            if ATOMIC.load(Ordering::SeqCst) != value.wrapping_add(change) {
                process::abort();
            }
        }
    };
}

macro_rules! test_incdec_unsigned {
    ($name:ident, $atomic_type:ty, $value_type:ty, $change_type:ty) => {
        fn $name(value: $value_type, _op: $change_type, pre_p: bool, change: $change_type) {
            static ATOMIC: $atomic_type = <$atomic_type>::new(0);
            ATOMIC.store(value, Ordering::SeqCst);
            
            let result = if pre_p {
                if change > 0 {
                    ATOMIC.fetch_add(change as $value_type, Ordering::SeqCst).wrapping_add(change as $value_type)
                } else {
                    ATOMIC.fetch_sub((-change) as $value_type, Ordering::SeqCst).wrapping_sub((-change) as $value_type)
                }
            } else {
                if change > 0 {
                    ATOMIC.fetch_add(change as $value_type, Ordering::SeqCst)
                } else {
                    ATOMIC.fetch_sub((-change) as $value_type, Ordering::SeqCst)
                }
            };
            
            let expected_final = if change > 0 {
                value.wrapping_add(change as $value_type)
            } else {
                value.wrapping_sub((-change) as $value_type)
            };
            let expected = if pre_p { expected_final } else { value };
            
            if result != expected {
                process::abort();
            }
            if ATOMIC.load(Ordering::SeqCst) != expected_final {
                process::abort();
            }
        }
    };
}

test_incdec_signed!(test_incdec_i8, AtomicI8, i8);
test_incdec_unsigned!(test_incdec_u8, AtomicU8, u8, i8);
test_incdec_signed!(test_incdec_i16, AtomicI16, i16);
test_incdec_unsigned!(test_incdec_u16, AtomicU16, u16, i16);
test_incdec_signed!(test_incdec_i32, AtomicI32, i32);
test_incdec_unsigned!(test_incdec_u32, AtomicU32, u32, i32);
test_incdec_signed!(test_incdec_i64, AtomicI64, i64);
test_incdec_unsigned!(test_incdec_u64, AtomicU64, u64, i64);
test_incdec_signed!(test_incdec_isize, AtomicIsize, isize);
test_incdec_unsigned!(test_incdec_usize, AtomicUsize, usize, isize);

fn test_incdec_float(value: f64, _op: f64, pre_p: bool, change: f64) {
    static ATOMIC: AtomicU64 = AtomicU64::new(0);
    ATOMIC.store(value.to_bits(), Ordering::SeqCst);
    
    let result = if pre_p {
        let new_val = value + change;
        ATOMIC.store(new_val.to_bits(), Ordering::SeqCst);
        new_val
    } else {
        let old = f64::from_bits(ATOMIC.load(Ordering::SeqCst));
        let new_val = value + change;
        ATOMIC.store(new_val.to_bits(), Ordering::SeqCst);
        old
    };
    
    let expected = if pre_p { value + change } else { value };
    if result != expected {
        process::abort();
    }
    if f64::from_bits(ATOMIC.load(Ordering::SeqCst)) != value + change {
        process::abort();
    }
}

fn test_incdec_ptr(value: isize, _op: isize, pre_p: bool, change: isize) {
    static ATOMIC: AtomicIsize = AtomicIsize::new(0);
    ATOMIC.store(value, Ordering::SeqCst);
    
    let result = if pre_p {
        ATOMIC.fetch_add(change, Ordering::SeqCst) + change
    } else {
        ATOMIC.fetch_add(change, Ordering::SeqCst)
    };
    
    let expected = if pre_p { value + change } else { value };
    if result != expected {
        process::abort();
    }
    if ATOMIC.load(Ordering::SeqCst) != value + change {
        process::abort();
    }
}

fn main() {
    test_incdec();
    process::exit(0);
}