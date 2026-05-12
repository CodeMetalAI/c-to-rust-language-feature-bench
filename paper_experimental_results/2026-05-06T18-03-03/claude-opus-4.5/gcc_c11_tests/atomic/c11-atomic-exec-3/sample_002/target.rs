use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::process::exit;

macro_rules! test_incdec_int {
    ($atomic_type:ty, $value_type:ty, $value:expr, pre_inc) => {{
        let a = <$atomic_type>::new($value as $value_type);
        let old = a.fetch_add(1, Ordering::SeqCst);
        let result = old.wrapping_add(1);
        let expected = ($value as $value_type).wrapping_add(1);
        if result != expected {
            panic!("pre_inc failed");
        }
        if a.load(Ordering::SeqCst) != expected {
            panic!("pre_inc value check failed");
        }
    }};
    ($atomic_type:ty, $value_type:ty, $value:expr, pre_dec) => {{
        let a = <$atomic_type>::new($value as $value_type);
        let old = a.fetch_sub(1, Ordering::SeqCst);
        let result = old.wrapping_sub(1);
        let expected = ($value as $value_type).wrapping_sub(1);
        if result != expected {
            panic!("pre_dec failed");
        }
        if a.load(Ordering::SeqCst) != expected {
            panic!("pre_dec value check failed");
        }
    }};
    ($atomic_type:ty, $value_type:ty, $value:expr, post_inc) => {{
        let a = <$atomic_type>::new($value as $value_type);
        let result = a.fetch_add(1, Ordering::SeqCst);
        let expected_result = $value as $value_type;
        let expected_final = ($value as $value_type).wrapping_add(1);
        if result != expected_result {
            panic!("post_inc failed");
        }
        if a.load(Ordering::SeqCst) != expected_final {
            panic!("post_inc value check failed");
        }
    }};
    ($atomic_type:ty, $value_type:ty, $value:expr, post_dec) => {{
        let a = <$atomic_type>::new($value as $value_type);
        let result = a.fetch_sub(1, Ordering::SeqCst);
        let expected_result = $value as $value_type;
        let expected_final = ($value as $value_type).wrapping_sub(1);
        if result != expected_result {
            panic!("post_dec failed");
        }
        if a.load(Ordering::SeqCst) != expected_final {
            panic!("post_dec value check failed");
        }
    }};
}

macro_rules! test_incdec_bool {
    ($value:expr, $op:ident) => {{
        let val: bool = ($value as i64) != 0;
        let a = AtomicBool::new(val);
        match stringify!($op) {
            "pre_inc" | "post_inc" => {
                // In C, incrementing bool always results in true (1)
                let old = a.swap(true, Ordering::SeqCst);
                let result = if stringify!($op) == "pre_inc" { true } else { old };
                let expected = if stringify!($op) == "pre_inc" { true } else { val };
                if result != expected {
                    panic!("bool inc failed");
                }
            }
            "pre_dec" | "post_dec" => {
                // In C, decrementing bool: true->false, false->true (wraps)
                let old = a.load(Ordering::SeqCst);
                let new_val = !old;
                a.store(new_val, Ordering::SeqCst);
                let result = if stringify!($op) == "pre_dec" { new_val } else { old };
                let expected = if stringify!($op) == "pre_dec" { !val } else { val };
                if result != expected {
                    panic!("bool dec failed");
                }
            }
            _ => {}
        }
    }};
}

macro_rules! test_incdec_float {
    ($value_type:ty, $value:expr, $op:ident) => {{
        let val: $value_type = $value as $value_type;
        let a = std::cell::Cell::new(val);
        let (result, expected_result, expected_final) = match stringify!($op) {
            "pre_inc" => {
                let new_val = a.get() + 1.0;
                a.set(new_val);
                (new_val, val + 1.0, val + 1.0)
            }
            "pre_dec" => {
                let new_val = a.get() - 1.0;
                a.set(new_val);
                (new_val, val - 1.0, val - 1.0)
            }
            "post_inc" => {
                let old = a.get();
                a.set(old + 1.0);
                (old, val, val + 1.0)
            }
            "post_dec" => {
                let old = a.get();
                a.set(old - 1.0);
                (old, val, val - 1.0)
            }
            _ => panic!("unknown op"),
        };
        if result != expected_result {
            panic!("float op failed");
        }
        if a.get() != expected_final {
            panic!("float value check failed");
        }
    }};
}

macro_rules! test_incdec_arith {
    ($value:expr, $op:ident) => {{
        test_incdec_bool!($value, $op);
        test_incdec_int!(AtomicI8, i8, $value, $op);
        test_incdec_int!(AtomicU8, u8, $value, $op);
        test_incdec_int!(AtomicI16, i16, $value, $op);
        test_incdec_int!(AtomicU16, u16, $value, $op);
        test_incdec_int!(AtomicI32, i32, $value, $op);
        test_incdec_int!(AtomicU32, u32, $value, $op);
        test_incdec_int!(AtomicI64, i64, $value, $op);
        test_incdec_int!(AtomicU64, u64, $value, $op);
        test_incdec_float!(f32, $value, $op);
        test_incdec_float!(f64, $value, $op);
    }};
}

macro_rules! test_all_incdec_arith {
    ($value:expr) => {{
        test_incdec_arith!($value, pre_inc);
        test_incdec_arith!($value, pre_dec);
        test_incdec_arith!($value, post_inc);
        test_incdec_arith!($value, post_dec);
    }};
}

macro_rules! test_incdec_ptr {
    ($op:ident) => {{
        let ia: [i32; 2] = [0, 0];
        let base_ptr = ia.as_ptr() as usize;
        let ptr_to_1 = base_ptr + std::mem::size_of::<i32>();
        let a = AtomicUsize::new(ptr_to_1);
        
        match stringify!($op) {
            "pre_inc" => {
                let old = a.fetch_add(std::mem::size_of::<i32>(), Ordering::SeqCst);
                let result = old + std::mem::size_of::<i32>();
                let expected = ptr_to_1 + std::mem::size_of::<i32>();
                if result != expected {
                    panic!("ptr pre_inc failed");
                }
            }
            "pre_dec" => {
                let old = a.fetch_sub(std::mem::size_of::<i32>(), Ordering::SeqCst);
                let result = old - std::mem::size_of::<i32>();
                let expected = ptr_to_1 - std::mem::size_of::<i32>();
                if result != expected {
                    panic!("ptr pre_dec failed");
                }
            }
            "post_inc" => {
                let result = a.fetch_add(std::mem::size_of::<i32>(), Ordering::SeqCst);
                if result != ptr_to_1 {
                    panic!("ptr post_inc failed");
                }
            }
            "post_dec" => {
                let result = a.fetch_sub(std::mem::size_of::<i32>(), Ordering::SeqCst);
                if result != ptr_to_1 {
                    panic!("ptr post_dec failed");
                }
            }
            _ => {}
        }
    }};
}

fn test_incdec() {
    test_all_incdec_arith!(0);
    test_all_incdec_arith!(1);
    test_all_incdec_arith!(2);
    test_all_incdec_arith!(-1);
    test_all_incdec_arith!(1u64 << 60);
    test_all_incdec_arith!(1.5);
    
    test_incdec_ptr!(pre_inc);
    test_incdec_ptr!(pre_dec);
    test_incdec_ptr!(post_inc);
    test_incdec_ptr!(post_dec);
}

fn main() {
    test_incdec();
    exit(0);
}