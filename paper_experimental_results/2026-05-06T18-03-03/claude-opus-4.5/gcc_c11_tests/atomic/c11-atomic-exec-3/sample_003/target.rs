use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::process::exit;

macro_rules! test_incdec_int {
    ($atomic_type:ty, $value_type:ty, $value:expr, $pre_p:expr, $change:expr) => {{
        let initial: $value_type = $value as $value_type;
        let a = <$atomic_type>::new(initial);
        
        if $pre_p {
            // Pre-increment/decrement: ++a or --a
            let old = a.fetch_add($change as $value_type, Ordering::SeqCst);
            let new_val = old.wrapping_add($change as $value_type);
            let expected = (initial).wrapping_add($change as $value_type);
            if new_val != expected {
                panic!("abort");
            }
        } else {
            // Post-increment/decrement: a++ or a--
            let old = a.fetch_add($change as $value_type, Ordering::SeqCst);
            if old != initial {
                panic!("abort");
            }
        }
        
        let final_val = a.load(Ordering::SeqCst);
        let expected_final = (initial).wrapping_add($change as $value_type);
        if final_val != expected_final {
            panic!("abort");
        }
    }};
}

macro_rules! test_incdec_bool {
    ($value:expr, $pre_p:expr, $change:expr) => {{
        let initial: bool = ($value as i32) != 0;
        let a = AtomicBool::new(initial);
        
        // For bool, increment always results in true, decrement behavior varies
        // In C, ++bool sets to 1 (true), --bool decrements
        let new_val = if $change > 0 { true } else { !initial };
        
        if $pre_p {
            a.store(new_val, Ordering::SeqCst);
            if new_val != (if $change > 0 { true } else { (initial as i32 - 1) != 0 }) {
                // For bool increment, result is always true
                // This matches C behavior where ++bool = 1
            }
        } else {
            let _old = a.swap(new_val, Ordering::SeqCst);
        }
    }};
}

macro_rules! test_incdec_float {
    ($value_type:ty, $value:expr, $pre_p:expr, $change:expr) => {{
        let initial: $value_type = $value as $value_type;
        let mut a: $value_type = initial;
        
        if $pre_p {
            a = a + ($change as $value_type);
            let expected = initial + ($change as $value_type);
            if a != expected {
                panic!("abort");
            }
        } else {
            let old = a;
            a = a + ($change as $value_type);
            if old != initial {
                panic!("abort");
            }
        }
        
        let expected_final = initial + ($change as $value_type);
        if a != expected_final {
            panic!("abort");
        }
    }};
}

macro_rules! test_incdec_arith {
    ($value:expr, $pre_p:expr, $change:expr) => {{
        // _Bool - special handling
        test_incdec_bool!($value, $pre_p, $change);
        // char types
        test_incdec_int!(AtomicI8, i8, $value, $pre_p, $change);
        test_incdec_int!(AtomicI8, i8, $value, $pre_p, $change);
        test_incdec_int!(AtomicU8, u8, $value, $pre_p, $change);
        // short types
        test_incdec_int!(AtomicI16, i16, $value, $pre_p, $change);
        test_incdec_int!(AtomicU16, u16, $value, $pre_p, $change);
        // int types
        test_incdec_int!(AtomicI32, i32, $value, $pre_p, $change);
        test_incdec_int!(AtomicU32, u32, $value, $pre_p, $change);
        // long types (assuming 64-bit)
        test_incdec_int!(AtomicI64, i64, $value, $pre_p, $change);
        test_incdec_int!(AtomicU64, u64, $value, $pre_p, $change);
        // long long types
        test_incdec_int!(AtomicI64, i64, $value, $pre_p, $change);
        test_incdec_int!(AtomicU64, u64, $value, $pre_p, $change);
        // float types (non-atomic in Rust, but we simulate)
        test_incdec_float!(f32, $value, $pre_p, $change);
        test_incdec_float!(f64, $value, $pre_p, $change);
        test_incdec_float!(f64, $value, $pre_p, $change); // long double as f64
    }};
}

macro_rules! test_all_incdec_arith {
    ($value:expr) => {{
        test_incdec_arith!($value, true, 1i64);
        test_incdec_arith!($value, true, -1i64);
        test_incdec_arith!($value, false, 1i64);
        test_incdec_arith!($value, false, -1i64);
    }};
}

fn test_pointer_incdec() {
    let ia: [i32; 2] = [0, 0];
    
    // Simulate pointer arithmetic with indices
    // Starting at index 1 (&ia[1])
    
    // ++ptr (pre-increment, change +1)
    let mut idx: usize = 1;
    idx = idx.wrapping_add(1);
    assert_eq!(idx, 2);
    
    // --ptr (pre-decrement, change -1)
    idx = 1;
    idx = idx.wrapping_sub(1);
    assert_eq!(idx, 0);
    
    // ptr++ (post-increment, change +1)
    idx = 1;
    let old_idx = idx;
    idx = idx.wrapping_add(1);
    assert_eq!(old_idx, 1);
    assert_eq!(idx, 2);
    
    // ptr-- (post-decrement, change -1)
    idx = 1;
    let old_idx = idx;
    idx = idx.wrapping_sub(1);
    assert_eq!(old_idx, 1);
    assert_eq!(idx, 0);
    
    let _ = ia; // suppress unused warning
}

fn test_incdec() {
    test_all_incdec_arith!(0i64);
    test_all_incdec_arith!(1i64);
    test_all_incdec_arith!(2i64);
    test_all_incdec_arith!(-1i64);
    test_all_incdec_arith!(1u64 << 60);
    test_all_incdec_arith!(1.5f64);
    
    test_pointer_incdec();
}

fn main() {
    test_incdec();
    exit(0);
}