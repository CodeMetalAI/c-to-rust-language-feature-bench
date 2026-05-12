use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize, Ordering};
use std::process::exit;

macro_rules! test_incdec {
    ($type:ty, $value:expr, $preop:ident, $postop:ident, $pre_p:expr, $change:expr) => {
        {
            static volatile ATOMIC_VAR: std::sync::atomic::$type = std::sync::atomic::$type::new($value as $type);
            let preop_result = if $pre_p {
                $preop(&ATOMIC_VAR, $change as $type)
            } else {
                let prev = ATOMIC_VAR.load(Ordering::SeqCst);
                $postop(&ATOMIC_VAR, $change as $type);
                prev
            };
            let expected = if $pre_p {
                $value + $change
            } else {
                $value
            };

            if preop_result != expected as $type {
                panic!("Test failed: expected {:?}, got {:?}", expected, preop_result);
            }
            
            let final_value = ATOMIC_VAR.load(Ordering::SeqCst);
            if final_value != (expected as $type) {
                panic!("Test failed: final value expected {:?}, got {:?}", (expected as $type), final_value);
            }
        }
    };
}

macro_rules! test_incdec_arith {
    ($value:expr, $preop:ident, $postop:ident, $pre_p:expr, $change:expr) => {
        {
            test_incdec!(AtomicBool, $value, $preop, $postop, $pre_p, $change);
            test_incdec!(AtomicI8, $value, $preop, $postop, $pre_p, $change);
            test_incdec!(AtomicU8, $value, $preop, $postop, $pre_p, $change);
            test_incdec!(AtomicI16, $value, $preop, $postop, $pre_p, $change);
            test_incdec!(AtomicU16, $value, $preop, $postop, $pre_p, $change);
            test_incdec!(AtomicI32, $value, $preop, $postop, $pre_p, $change);
            test_incdec!(AtomicU32, $value, $preop, $postop, $pre_p, $change);
            test_incdec!(AtomicI64, $value, $preop, $postop, $pre_p, $change);
            test_incdec!(AtomicU64, $value, $preop, $postop, $pre_p, $change);
        }
    };
}

macro_rules! test_all_incdec_arith {
    ($value:expr) => {
        {
            test_incdec_arith!($value, atomic_add, atomic_sub, true, 1);
            test_incdec_arith!($value, atomic_sub, atomic_add, true, -1);
            test_incdec_arith!($value, atomic_add, atomic_sub, false, 1);
            test_incdec_arith!($value, atomic_sub, atomic_add, false, -1);
        }
    };
}

fn atomic_add<T>(atomic: &std::sync::atomic::AtomicI32, value: T) -> i32
where T: Into<i32> {
    atomic.fetch_add(value.into(), Ordering::SeqCst)
}

fn atomic_sub<T>(atomic: &std::sync::atomic::AtomicI32, value: T) -> i32
where T: Into<i32> {
    atomic.fetch_sub(value.into(), Ordering::SeqCst)
}

fn test_incdec() {
    test_all_incdec_arith!(0);
    test_all_incdec_arith!(1);
    test_all_incdec_arith!(2);
    test_all_incdec_arith!(-1);
    test_all_incdec_arith!(1u64 << 60);
    test_all_incdec_arith!(1.5f32);
}

fn main() {
    test_incdec();
    exit(0);
}