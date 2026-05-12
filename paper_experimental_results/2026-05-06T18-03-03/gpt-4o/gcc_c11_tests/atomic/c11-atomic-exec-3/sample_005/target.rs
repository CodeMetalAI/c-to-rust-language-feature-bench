use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize, Ordering};
use std::process::exit;

macro_rules! test_incdec {
    ($type:ty, $value:expr, $preop:expr, $postop:expr, $pre_p:expr, $change:expr) => {{
        let a = std::sync::Arc::new(std::sync::atomic::AtomicPtr::new(Box::into_raw(Box::new($value as $type))));
        let a_clone = a.clone();
        let a_ref = unsafe { &*a_clone.load(Ordering::SeqCst) };
        let preop_result = $preop(a_ref);
        let postop_result = $postop(a_ref);
        let expected = if $pre_p {
            ($value as $type) + $change
        } else {
            $value as $type
        };
        if preop_result != expected {
            panic!("Test failed");
        }
        if postop_result != expected + $change {
            panic!("Test failed");
        }
    }};
}

macro_rules! test_incdec_arith {
    ($value:expr, $preop:expr, $postop:expr, $pre_p:expr, $change:expr) => {{
        test_incdec!(bool, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(i8, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(u8, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(i16, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(u16, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(i32, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(u32, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(i64, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(u64, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(isize, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(usize, $value, $preop, $postop, $pre_p, $change);
    }};
}

macro_rules! test_all_incdec_arith {
    ($value:expr) => {{
        test_incdec_arith!($value, |x| x.wrapping_add(1), |x| x.wrapping_add(1), true, 1);
        test_incdec_arith!($value, |x| x.wrapping_sub(1), |x| x.wrapping_sub(1), true, -1);
        test_incdec_arith!($value, |x| x.wrapping_add(1), |x| x.wrapping_add(1), false, 1);
        test_incdec_arith!($value, |x| x.wrapping_sub(1), |x| x.wrapping_sub(1), false, -1);
    }};
}

fn test_incdec() {
    test_all_incdec_arith!(0);
    test_all_incdec_arith!(1);
    test_all_incdec_arith!(2);
    test_all_incdec_arith!(-1);
    test_all_incdec_arith!(1u64 << 60);
    test_all_incdec_arith!(1.5_f64);
}

fn main() {
    test_incdec();
    exit(0);
}