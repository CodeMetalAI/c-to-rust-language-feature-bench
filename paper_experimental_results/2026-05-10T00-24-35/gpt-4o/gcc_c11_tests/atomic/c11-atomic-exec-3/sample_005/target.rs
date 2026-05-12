use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize, Ordering};
use std::process;

macro_rules! test_incdec {
    ($ty:ty, $value:expr, $preop:ident, $postop:ident, $pre_p:expr, $change:expr) => {{
        let a = std::boxed::Box::leak(Box::new(std::sync::atomic::$ty::new($value as _)));
        let pre_value = if $pre_p {
            a.$preop(Ordering::SeqCst)
        } else {
            let post_value = a.$postop(Ordering::SeqCst);
            if <$ty>::from($value) == post_value {
                $value as _
            } else {
                post_value
            }
        };
        if pre_value != if $pre_p {
            ($value + $change) as _
        } else {
            $value as _
        } {
            process::abort();
        }
        if a.load(Ordering::SeqCst) != ($value + $change) as _ {
            process::abort();
        }
    }};
}

macro_rules! test_incdec_arith {
    ($value:expr, $preop:ident, $postop:ident, $pre_p:expr, $change:expr) => {{
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
    }};
}

macro_rules! test_all_incdec_arith {
    ($value:expr) => {{
        test_incdec_arith!($value, fetch_add, fetch_add, true, 1);
        test_incdec_arith!($value, fetch_sub, fetch_sub, true, -1);
        test_incdec_arith!($value, fetch_add, fetch_add, false, 1);
        test_incdec_arith!($value, fetch_sub, fetch_sub, false, -1);
    }};
}

fn test_incdec() {
    test_all_incdec_arith!(0);
    test_all_incdec_arith!(1);
    test_all_incdec_arith!(2);
    test_all_incdec_arith!(-1);
    test_all_incdec_arith!(1u64 << 60);
    test_all_incdec_arith!(1.5);
}

fn main() {
    test_incdec();
    process::exit(0);
}