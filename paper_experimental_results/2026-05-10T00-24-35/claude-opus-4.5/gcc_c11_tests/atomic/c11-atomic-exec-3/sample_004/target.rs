use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::process::exit;

fn test_incdec_bool(value: bool, pre_p: bool, change: i32) {
    let a = AtomicBool::new(value);
    
    if pre_p {
        // pre-increment or pre-decrement
        let new_val = if change > 0 { true } else { !value };
        a.store(new_val, Ordering::SeqCst);
        let result = a.load(Ordering::SeqCst);
        let expected = if change > 0 { true } else { !value };
        if result != expected {
            panic!("abort");
        }
    } else {
        // post-increment or post-decrement
        let old_val = a.load(Ordering::SeqCst);
        let new_val = if change > 0 { true } else { !value };
        a.store(new_val, Ordering::SeqCst);
        if old_val != value {
            panic!("abort");
        }
    }
    
    let expected_final = if change > 0 { true } else { !value };
    if a.load(Ordering::SeqCst) != expected_final {
        panic!("abort");
    }
}

macro_rules! test_incdec_integer {
    ($atomic_type:ty, $value_type:ty, $value:expr, $pre_p:expr, $change:expr) => {{
        let val: $value_type = $value as $value_type;
        let a = <$atomic_type>::new(val);
        
        if $pre_p {
            let new_val = val.wrapping_add($change as $value_type);
            a.store(new_val, Ordering::SeqCst);
            let result = a.load(Ordering::SeqCst);
            if result != new_val {
                panic!("abort");
            }
        } else {
            let old_val = a.load(Ordering::SeqCst);
            let new_val = val.wrapping_add($change as $value_type);
            a.store(new_val, Ordering::SeqCst);
            if old_val != val {
                panic!("abort");
            }
        }
        
        let expected_final = val.wrapping_add($change as $value_type);
        if a.load(Ordering::SeqCst) != expected_final {
            panic!("abort");
        }
    }};
}

macro_rules! test_incdec_float {
    ($value_type:ty, $value:expr, $pre_p:expr, $change:expr) => {{
        let val: $value_type = $value as $value_type;
        let mut a: $value_type = val;
        
        if $pre_p {
            a = val + ($change as $value_type);
            let result = a;
            let expected = val + ($change as $value_type);
            if result != expected {
                panic!("abort");
            }
        } else {
            let old_val = a;
            a = val + ($change as $value_type);
            if old_val != val {
                panic!("abort");
            }
        }
        
        let expected_final = val + ($change as $value_type);
        if a != expected_final {
            panic!("abort");
        }
    }};
}

fn test_incdec_arith(value: i64, pre_p: bool, change: i32) {
    test_incdec_bool(value != 0, pre_p, change);
    test_incdec_integer!(AtomicI8, i8, value, pre_p, change);
    test_incdec_integer!(AtomicU8, u8, value, pre_p, change);
    test_incdec_integer!(AtomicI16, i16, value, pre_p, change);
    test_incdec_integer!(AtomicU16, u16, value, pre_p, change);
    test_incdec_integer!(AtomicI32, i32, value, pre_p, change);
    test_incdec_integer!(AtomicU32, u32, value, pre_p, change);
    test_incdec_integer!(AtomicI64, i64, value, pre_p, change);
    test_incdec_integer!(AtomicU64, u64, value, pre_p, change);
    test_incdec_float!(f32, value, pre_p, change);
    test_incdec_float!(f64, value, pre_p, change);
}

fn test_all_incdec_arith(value: i64) {
    test_incdec_arith(value, true, 1);   // ++a
    test_incdec_arith(value, true, -1);  // --a
    test_incdec_arith(value, false, 1);  // a++
    test_incdec_arith(value, false, -1); // a--
}

fn test_incdec_pointer(index: usize, pre_p: bool, change: i32) {
    let a = AtomicUsize::new(index);
    
    if pre_p {
        let new_val = (index as isize + change as isize) as usize;
        a.store(new_val, Ordering::SeqCst);
        let result = a.load(Ordering::SeqCst);
        if result != new_val {
            panic!("abort");
        }
    } else {
        let old_val = a.load(Ordering::SeqCst);
        let new_val = (index as isize + change as isize) as usize;
        a.store(new_val, Ordering::SeqCst);
        if old_val != index {
            panic!("abort");
        }
    }
    
    let expected_final = (index as isize + change as isize) as usize;
    if a.load(Ordering::SeqCst) != expected_final {
        panic!("abort");
    }
}

fn test_incdec() {
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith(1i64 << 60);
    
    // For 1.5, we only test float types
    test_incdec_float!(f32, 1.5, true, 1);
    test_incdec_float!(f32, 1.5, true, -1);
    test_incdec_float!(f32, 1.5, false, 1);
    test_incdec_float!(f32, 1.5, false, -1);
    test_incdec_float!(f64, 1.5, true, 1);
    test_incdec_float!(f64, 1.5, true, -1);
    test_incdec_float!(f64, 1.5, false, 1);
    test_incdec_float!(f64, 1.5, false, -1);
    
    // Pointer tests (using index 1 to represent &ia[1])
    test_incdec_pointer(1, true, 1);   // ++ptr
    test_incdec_pointer(1, true, -1);  // --ptr
    test_incdec_pointer(1, false, 1);  // ptr++
    test_incdec_pointer(1, false, -1); // ptr--
}

fn main() {
    test_incdec();
    exit(0);
}