use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::process;

macro_rules! test_incdec {
    ($atomic_type:ty, $value_type:ty, $value:expr, pre_inc) => {{
        static A: $atomic_type = <$atomic_type>::new($value as $value_type);
        let result = A.fetch_add(1, Ordering::SeqCst).wrapping_add(1);
        if result != ($value as $value_type).wrapping_add(1) {
            process::abort();
        }
        if A.load(Ordering::SeqCst) != ($value as $value_type).wrapping_add(1) {
            process::abort();
        }
    }};
    ($atomic_type:ty, $value_type:ty, $value:expr, pre_dec) => {{
        static A: $atomic_type = <$atomic_type>::new($value as $value_type);
        let result = A.fetch_sub(1, Ordering::SeqCst).wrapping_sub(1);
        if result != ($value as $value_type).wrapping_sub(1) {
            process::abort();
        }
        if A.load(Ordering::SeqCst) != ($value as $value_type).wrapping_sub(1) {
            process::abort();
        }
    }};
    ($atomic_type:ty, $value_type:ty, $value:expr, post_inc) => {{
        static A: $atomic_type = <$atomic_type>::new($value as $value_type);
        let result = A.fetch_add(1, Ordering::SeqCst);
        if result != $value as $value_type {
            process::abort();
        }
        if A.load(Ordering::SeqCst) != ($value as $value_type).wrapping_add(1) {
            process::abort();
        }
    }};
    ($atomic_type:ty, $value_type:ty, $value:expr, post_dec) => {{
        static A: $atomic_type = <$atomic_type>::new($value as $value_type);
        let result = A.fetch_sub(1, Ordering::SeqCst);
        if result != $value as $value_type {
            process::abort();
        }
        if A.load(Ordering::SeqCst) != ($value as $value_type).wrapping_sub(1) {
            process::abort();
        }
    }};
}

macro_rules! test_incdec_arith {
    ($value:expr, $op:ident) => {{
        test_incdec!(AtomicBool, bool, $value, $op);
        test_incdec!(AtomicI8, i8, $value, $op);
        test_incdec!(AtomicI8, i8, $value, $op);
        test_incdec!(AtomicU8, u8, $value, $op);
        test_incdec!(AtomicI16, i16, $value, $op);
        test_incdec!(AtomicU16, u16, $value, $op);
        test_incdec!(AtomicI32, i32, $value, $op);
        test_incdec!(AtomicU32, u32, $value, $op);
        test_incdec!(AtomicI64, i64, $value, $op);
        test_incdec!(AtomicU64, u64, $value, $op);
        test_incdec!(AtomicI64, i64, $value, $op);
        test_incdec!(AtomicU64, u64, $value, $op);
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

fn test_incdec() {
    test_all_incdec_arith!(0);
    test_all_incdec_arith!(1);
    test_all_incdec_arith!(2);
    test_all_incdec_arith!(-1);
    test_all_incdec_arith!(1u64 << 60);
}

fn main() {
    test_incdec();
    process::exit(0);
}