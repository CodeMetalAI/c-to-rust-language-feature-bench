use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};

fn test_incdec_bool(value: bool, pre_p: bool, change: i32) {
    let a = AtomicBool::new(value);
    
    if pre_p {
        // pre-increment or pre-decrement
        let new_val = if change > 0 { true } else { !value };
        a.store(new_val, Ordering::SeqCst);
        let result = a.load(Ordering::SeqCst);
        let expected = new_val;
        if result != expected {
            std::process::abort();
        }
    } else {
        // post-increment or post-decrement
        let old_val = a.load(Ordering::SeqCst);
        let new_val = if change > 0 { true } else { !old_val };
        a.store(new_val, Ordering::SeqCst);
        if old_val != value {
            std::process::abort();
        }
    }
    
    let expected_final = if change > 0 { true } else { !value };
    if a.load(Ordering::SeqCst) != expected_final {
        std::process::abort();
    }
}

macro_rules! test_incdec_int {
    ($type:ty, $atomic:ty, $value:expr, $pre_p:expr, $change:expr) => {{
        let val: $type = $value as $type;
        let a = <$atomic>::new(val);
        
        if $pre_p {
            let new_val = val.wrapping_add($change as $type);
            a.store(new_val, Ordering::SeqCst);
            let result = a.load(Ordering::SeqCst);
            if result != new_val {
                std::process::abort();
            }
        } else {
            let old_val = a.load(Ordering::SeqCst);
            let new_val = old_val.wrapping_add($change as $type);
            a.store(new_val, Ordering::SeqCst);
            if old_val != val {
                std::process::abort();
            }
        }
        
        let expected_final = val.wrapping_add($change as $type);
        if a.load(Ordering::SeqCst) != expected_final {
            std::process::abort();
        }
    }};
}

macro_rules! test_incdec_float {
    ($type:ty, $value:expr, $pre_p:expr, $change:expr) => {{
        let val: $type = $value as $type;
        let mut a = val;
        
        if $pre_p {
            a = val + ($change as $type);
            let result = a;
            let expected = val + ($change as $type);
            if result != expected {
                std::process::abort();
            }
        } else {
            let old_val = a;
            a = old_val + ($change as $type);
            if old_val != val {
                std::process::abort();
            }
        }
        
        let expected_final = val + ($change as $type);
        if a != expected_final {
            std::process::abort();
        }
    }};
}

macro_rules! test_incdec_arith {
    ($value:expr, $pre_p:expr, $change:expr) => {{
        test_incdec_bool($value != 0, $pre_p, $change);
        test_incdec_int!(i8, AtomicI8, $value, $pre_p, $change);
        test_incdec_int!(i8, AtomicI8, $value, $pre_p, $change);
        test_incdec_int!(u8, AtomicU8, $value, $pre_p, $change);
        test_incdec_int!(i16, AtomicI16, $value, $pre_p, $change);
        test_incdec_int!(u16, AtomicU16, $value, $pre_p, $change);
        test_incdec_int!(i32, AtomicI32, $value, $pre_p, $change);
        test_incdec_int!(u32, AtomicU32, $value, $pre_p, $change);
        test_incdec_int!(i64, AtomicI64, $value, $pre_p, $change);
        test_incdec_int!(u64, AtomicU64, $value, $pre_p, $change);
        test_incdec_int!(i64, AtomicI64, $value, $pre_p, $change);
        test_incdec_int!(u64, AtomicU64, $value, $pre_p, $change);
        test_incdec_float!(f32, $value, $pre_p, $change);
        test_incdec_float!(f64, $value, $pre_p, $change);
        test_incdec_float!(f64, $value, $pre_p, $change);
    }};
}

macro_rules! test_all_incdec_arith {
    ($value:expr) => {{
        test_incdec_arith!($value, true, 1);
        test_incdec_arith!($value, true, -1);
        test_incdec_arith!($value, false, 1);
        test_incdec_arith!($value, false, -1);
    }};
}

fn test_incdec_ptr(start_index: usize, pre_p: bool, change: i32) {
    let a = AtomicUsize::new(start_index);
    
    if pre_p {
        let new_val = (start_index as isize + change as isize) as usize;
        a.store(new_val, Ordering::SeqCst);
        let result = a.load(Ordering::SeqCst);
        if result != new_val {
            std::process::abort();
        }
    } else {
        let old_val = a.load(Ordering::SeqCst);
        let new_val = (old_val as isize + change as isize) as usize;
        a.store(new_val, Ordering::SeqCst);
        if old_val != start_index {
            std::process::abort();
        }
    }
    
    let expected_final = (start_index as isize + change as isize) as usize;
    if a.load(Ordering::SeqCst) != expected_final {
        std::process::abort();
    }
}

fn test_incdec() {
    test_all_incdec_arith!(0);
    test_all_incdec_arith!(1);
    test_all_incdec_arith!(2);
    test_all_incdec_arith!(-1);
    test_all_incdec_arith!(1u64 << 60);
    test_all_incdec_arith!(1.5);
    
    // Pointer tests - using index 1 as the starting point
    test_incdec_ptr(1, true, 1);
    test_incdec_ptr(1, true, -1);
    test_incdec_ptr(1, false, 1);
    test_incdec_ptr(1, false, -1);
}

fn main() {
    test_incdec();
    std::process::exit(0);
}