use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, Ordering};
use std::process::exit;

macro_rules! test_incdec {
    ($atomic_type:ty, $value:expr, $preop:ident, $postop:ident, $pre_p:expr, $change:expr) => {
        {
            static A: $atomic_type = $atomic_type::new($value);
            let old_value = A.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |a| Some(a.wrapping_add($change))).unwrap();
            if $pre_p {
                if old_value != $value.wrapping_add($change) {
                    eprintln!("Test failed: old_value != value + change");
                    exit(1);
                }
            } else {
                if old_value != $value {
                    eprintln!("Test failed: old_value != value");
                    exit(1);
                }
            }
            let new_value = A.load(Ordering::SeqCst);
            if new_value != $value.wrapping_add($change) {
                eprintln!("Test failed: new_value != value + change");
                exit(1);
            }
        }
    };
}

macro_rules! test_incdec_arith {
    ($value:expr, $preop:ident, $postop:ident, $pre_p:expr, $change:expr) => {
        test_incdec!(AtomicBool, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicI8, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicU8, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicI16, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicU16, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicI32, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicU32, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicI64, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicU64, $value, $preop, $postop, $pre_p, $change);
    };
}

macro_rules! test_all_incdec_arith {
    ($value:expr) => {
        test_incdec_arith!($value, ++, , 1, 1);
        test_incdec_arith!($value, --, , 1, -1);
        test_incdec_arith!($value, , ++, 0, 1);
        test_incdec_arith!($value, , --, 0, -1);
    };
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
    exit(0);
}