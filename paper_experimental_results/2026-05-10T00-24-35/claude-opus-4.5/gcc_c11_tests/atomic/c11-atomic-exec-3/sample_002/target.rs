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
        // Final check
        if new_val != val + ($change as $type) {
            std::process::abort();
        }
    }};
}

fn test_incdec_arith(value: i64, pre_p: bool, change: i32) {
    // _Bool
    test_incdec_bool(value != 0, pre_p, change);
    
    // char (i8)
    test_incdec_int!(i8, AtomicI8, value, pre_p, change);
    // signed char (i8)
    test_incdec_int!(i8, AtomicI8, value, pre_p, change);
    // unsigned char (u8)
    test_incdec_int!(u8, AtomicU8, value, pre_p, change);
    // signed short (i16)
    test_incdec_int!(i16, AtomicI16, value, pre_p, change);
    // unsigned short (u16)
    test_incdec_int!(u16, AtomicU16, value, pre_p, change);
    // signed int (i32)
    test_incdec_int!(i32, AtomicI32, value, pre_p, change);
    // unsigned int (u32)
    test_incdec_int!(u32, AtomicU32, value, pre_p, change);
    // signed long (i64)
    test_incdec_int!(i64, AtomicI64, value, pre_p, change);
    // unsigned long (u64)
    test_incdec_int!(u64, AtomicU64, value, pre_p, change);
    // signed long long (i64)
    test_incdec_int!(i64, AtomicI64, value, pre_p, change);
    // unsigned long long (u64)
    test_incdec_int!(u64, AtomicU64, value, pre_p, change);
    
    // float
    test_incdec_float!(f32, value, pre_p, change);
    // double
    test_incdec_float!(f64, value, pre_p, change);
    // long double (f64 in Rust)
    test_incdec_float!(f64, value, pre_p, change);
}

fn test_incdec_arith_float(value: f64, pre_p: bool, change: i32) {
    test_incdec_bool(value != 0.0, pre_p, change);
    test_incdec_int!(i8, AtomicI8, value as i64, pre_p, change);
    test_incdec_int!(i8, AtomicI8, value as i64, pre_p, change);
    test_incdec_int!(u8, AtomicU8, value as i64, pre_p, change);
    test_incdec_int!(i16, AtomicI16, value as i64, pre_p, change);
    test_incdec_int!(u16, AtomicU16, value as i64, pre_p, change);
    test_incdec_int!(i32, AtomicI32, value as i64, pre_p, change);
    test_incdec_int!(u32, AtomicU32, value as i64, pre_p, change);
    test_incdec_int!(i64, AtomicI64, value as i64, pre_p, change);
    test_incdec_int!(u64, AtomicU64, value as i64, pre_p, change);
    test_incdec_int!(i64, AtomicI64, value as i64, pre_p, change);
    test_incdec_int!(u64, AtomicU64, value as i64, pre_p, change);
    test_incdec_float!(f32, value, pre_p, change);
    test_incdec_float!(f64, value, pre_p, change);
    test_incdec_float!(f64, value, pre_p, change);
}

fn test_all_incdec_arith(value: i64) {
    test_incdec_arith(value, true, 1);   // ++a
    test_incdec_arith(value, true, -1);  // --a
    test_incdec_arith(value, false, 1);  // a++
    test_incdec_arith(value, false, -1); // a--
}

fn test_all_incdec_arith_float(value: f64) {
    test_incdec_arith_float(value, true, 1);
    test_incdec_arith_float(value, true, -1);
    test_incdec_arith_float(value, false, 1);
    test_incdec_arith_float(value, false, -1);
}

fn test_incdec_ptr(value: usize, pre_p: bool, change: isize) {
    let a = AtomicUsize::new(value);
    let new_val = (value as isize + change) as usize;
    a.store(new_val, Ordering::SeqCst);
    let result = if pre_p { new_val } else { value };
    let expected = if pre_p { new_val } else { value };
    if result != expected {
        std::process::abort();
    }
    if a.load(Ordering::SeqCst) != new_val {
        std::process::abort();
    }
}

fn test_incdec() {
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith(1i64 << 60);
    test_all_incdec_arith_float(1.5);
    
    // Pointer tests - using index 1 of a 2-element array
    let ptr_val: usize = 1; // index into array
    test_incdec_ptr(ptr_val, true, 1);   // ++ptr
    test_incdec_ptr(ptr_val, true, -1);  // --ptr
    test_incdec_ptr(ptr_val, false, 1);  // ptr++
    test_incdec_ptr(ptr_val, false, -1); // ptr--
}

fn main() {
    test_incdec();
    std::process::exit(0);
}