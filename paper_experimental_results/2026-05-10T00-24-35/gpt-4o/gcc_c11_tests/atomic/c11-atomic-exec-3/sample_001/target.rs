use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize, Ordering};
use std::panic;

macro_rules! test_incdec {
    ($type:ty, $value:expr, $preop:ident, $postop:ident, $pre_p:expr, $change:expr) => {
        {
            let mut a = <$type>::new($value as $type);
            let preop_result = if $pre_p {
                a.$preop(Ordering::SeqCst) as $type
            } else {
                a.$postop(Ordering::SeqCst) as $type
            };
            if preop_result != if $pre_p { ($value as $type) + $change } else { $value as $type } {
                panic!("Preop failed for type: {}", std::any::type_name::<$type>());
            }
            if a.load(Ordering::SeqCst) != ($value as $type + $change) {
                panic!("Final value check failed for type: {}", std::any::type_name::<$type>());
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
            test_incdec!(AtomicIsize, $value, $preop, $postop, $pre_p, $change);
            test_incdec!(AtomicUsize, $value, $preop, $postop, $pre_p, $change);
        }
    };
}

macro_rules! test_all_incdec_arith {
    ($value:expr) => {
        {
            test_incdec_arith!($value, fetch_add, fetch_sub, true, 1);
            test_incdec_arith!($value, fetch_sub, fetch_add, true, -1);
            test_incdec_arith!($value, fetch_add, fetch_add, false, 1);
            test_incdec_arith!($value, fetch_sub, fetch_sub, false, -1);
        }
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
}