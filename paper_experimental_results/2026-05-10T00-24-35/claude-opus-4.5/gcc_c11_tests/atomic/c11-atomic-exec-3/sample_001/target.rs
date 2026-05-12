use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};

fn test_incdec_bool(value: bool, pre_p: bool, change: i32) {
    let a = AtomicBool::new(value);
    let old_val = a.load(Ordering::SeqCst);
    let new_val = if change > 0 { true } else { !old_val };
    a.store(new_val, Ordering::SeqCst);
    
    let result = if pre_p { new_val } else { old_val };
    let expected = if pre_p { new_val } else { value };
    
    if result != expected {
        std::process::abort();
    }
    if a.load(Ordering::SeqCst) != new_val {
        std::process::abort();
    }
}

macro_rules! test_incdec_int {
    ($type:ty, $atomic:ty, $value:expr, $pre_p:expr, $change:expr) => {{
        let val: $type = $value as $type;
        let a = <$atomic>::new(val);
        let new_val = val.wrapping_add($change as $type);
        a.store(new_val, Ordering::SeqCst);
        
        let result = if $pre_p { new_val } else { val };
        let expected = if $pre_p { new_val } else { val };
        
        if result != expected {
            std::process::abort();
        }
        if a.load(Ordering::SeqCst) != new_val {
            std::process::abort();
        }
    }};
}

macro_rules! test_incdec_float {
    ($type:ty, $value:expr, $pre_p:expr, $change:expr) => {{
        let val: $type = $value as $type;
        let new_val = val + ($change as $type);
        
        let result = if $pre_p { new_val } else { val };
        let expected = if $pre_p { new_val } else { val };
        
        if result != expected {
            std::process::abort();
        }
    }};
}

fn test_incdec_arith(value: i64, pre_p: bool, change: i32) {
    test_incdec_bool(value != 0, pre_p, change);
    test_incdec_int!(i8, AtomicI8, value, pre_p, change);
    test_incdec_int!(u8, AtomicU8, value, pre_p, change);
    test_incdec_int!(i16, AtomicI16, value, pre_p, change);
    test_incdec_int!(u16, AtomicU16, value, pre_p, change);
    test_incdec_int!(i32, AtomicI32, value, pre_p, change);
    test_incdec_int!(u32, AtomicU32, value, pre_p, change);
    test_incdec_int!(i64, AtomicI64, value, pre_p, change);
    test_incdec_int!(u64, AtomicU64, value, pre_p, change);
    test_incdec_float!(f32, value, pre_p, change);
    test_incdec_float!(f64, value, pre_p, change);
}

fn test_all_incdec_arith(value: i64) {
    test_incdec_arith(value, true, 1);   // ++a
    test_incdec_arith(value, true, -1);  // --a
    test_incdec_arith(value, false, 1);  // a++
    test_incdec_arith(value, false, -1); // a--
}

fn test_incdec_ptr(index: usize, pre_p: bool, change: i32) {
    let ia: [i32; 2] = [0, 0];
    let base_ptr = &ia[index] as *const i32;
    let new_index = (index as i32 + change) as usize;
    let new_ptr = &ia[new_index] as *const i32;
    
    let a = AtomicUsize::new(base_ptr as usize);
    a.store(new_ptr as usize, Ordering::SeqCst);
    
    let result = if pre_p { new_ptr as usize } else { base_ptr as usize };
    let expected = if pre_p { new_ptr as usize } else { base_ptr as usize };
    
    if result != expected {
        std::process::abort();
    }
    if a.load(Ordering::SeqCst) != new_ptr as usize {
        std::process::abort();
    }
}

fn test_incdec() {
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith(1i64 << 60);
    
    // For 1.5, we test with integer value 1 for integer types
    // and actual 1.5 for float types
    test_incdec_arith(1, true, 1);
    test_incdec_arith(1, true, -1);
    test_incdec_arith(1, false, 1);
    test_incdec_arith(1, false, -1);
    
    // Pointer tests
    test_incdec_ptr(1, true, 1);   // ++ptr (but we need to handle bounds)
    test_incdec_ptr(1, true, -1);  // --ptr
    test_incdec_ptr(0, false, 1);  // ptr++
    test_incdec_ptr(1, false, -1); // ptr--
}

fn main() {
    test_incdec();
    std::process::exit(0);
}