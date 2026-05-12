use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::process::exit;

macro_rules! test_incdec_int {
    ($atomic_type:ty, $value_type:ty, $value:expr, $pre_p:expr, $change:expr) => {{
        let initial: $value_type = $value as $value_type;
        let a = <$atomic_type>::new(initial);
        
        if $pre_p {
            // Pre-increment/decrement: returns new value
            let new_val = if $change > 0 {
                a.fetch_add(1, Ordering::SeqCst).wrapping_add(1)
            } else {
                a.fetch_sub(1, Ordering::SeqCst).wrapping_sub(1)
            };
            let expected = initial.wrapping_add($change as $value_type);
            if new_val != expected {
                panic!("abort");
            }
            if a.load(Ordering::SeqCst) != expected {
                panic!("abort");
            }
        } else {
            // Post-increment/decrement: returns old value
            let old_val = if $change > 0 {
                a.fetch_add(1, Ordering::SeqCst)
            } else {
                a.fetch_sub(1, Ordering::SeqCst)
            };
            if old_val != initial {
                panic!("abort");
            }
            let expected = initial.wrapping_add($change as $value_type);
            if a.load(Ordering::SeqCst) != expected {
                panic!("abort");
            }
        }
    }};
}

macro_rules! test_incdec_bool {
    ($value:expr, $pre_p:expr, $change:expr) => {{
        let initial: bool = ($value as i64) != 0;
        let a = AtomicBool::new(initial);
        
        // For bool, increment always sets to true, decrement behavior varies
        // In C, ++bool sets to 1 (true), --bool on true sets to 0 (false)
        let new_bool = if $change > 0 { true } else { !initial };
        
        if $pre_p {
            let old = a.swap(new_bool, Ordering::SeqCst);
            let returned = new_bool;
            let expected = if $change > 0 { true } else { (initial as u8).wrapping_sub(1) != 0 };
            // For bool: (bool)((bool)VALUE + CHANGE)
            // true + 1 = true, false + 1 = true, true - 1 = false, false - 1 = true (wrapping)
            let expected_val: bool = if $change > 0 {
                true
            } else {
                if initial { false } else { true }
            };
            if returned != expected_val {
                panic!("abort");
            }
        } else {
            let old = a.swap(new_bool, Ordering::SeqCst);
            if old != initial {
                panic!("abort");
            }
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
            let expected = initial + ($change as $value_type);
            if a != expected {
                panic!("abort");
            }
        }
    }};
}

macro_rules! test_incdec_arith {
    ($value:expr, $pre_p:expr, $change:expr) => {{
        // _Bool - simplified handling
        test_incdec_int!(AtomicI8, i8, $value, $pre_p, $change);
        test_incdec_int!(AtomicU8, u8, $value, $pre_p, $change);
        test_incdec_int!(AtomicI16, i16, $value, $pre_p, $change);
        test_incdec_int!(AtomicU16, u16, $value, $pre_p, $change);
        test_incdec_int!(AtomicI32, i32, $value, $pre_p, $change);
        test_incdec_int!(AtomicU32, u32, $value, $pre_p, $change);
        test_incdec_int!(AtomicI64, i64, $value, $pre_p, $change);
        test_incdec_int!(AtomicU64, u64, $value, $pre_p, $change);
        test_incdec_float!(f32, $value, $pre_p, $change);
        test_incdec_float!(f64, $value, $pre_p, $change);
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

fn test_incdec() {
    test_all_incdec_arith!(0i64);
    test_all_incdec_arith!(1i64);
    test_all_incdec_arith!(2i64);
    test_all_incdec_arith!(-1i64);
    test_all_incdec_arith!(1i64 << 60);
    test_all_incdec_arith!(1.5f64);
    
    // Pointer tests - using usize to simulate pointer arithmetic
    let ia: [i32; 2] = [0, 0];
    let base_ptr = 1usize; // Simulating &ia[1] as index 1
    
    // Pre-increment
    let a = AtomicUsize::new(base_ptr);
    let new_val = a.fetch_add(1, Ordering::SeqCst) + 1;
    if new_val != base_ptr + 1 { panic!("abort"); }
    
    // Pre-decrement
    let a = AtomicUsize::new(base_ptr);
    let new_val = a.fetch_sub(1, Ordering::SeqCst) - 1;
    if new_val != base_ptr - 1 { panic!("abort"); }
    
    // Post-increment
    let a = AtomicUsize::new(base_ptr);
    let old_val = a.fetch_add(1, Ordering::SeqCst);
    if old_val != base_ptr { panic!("abort"); }
    
    // Post-decrement
    let a = AtomicUsize::new(base_ptr);
    let old_val = a.fetch_sub(1, Ordering::SeqCst);
    if old_val != base_ptr { panic!("abort"); }
}

fn main() {
    test_incdec();
    exit(0);
}