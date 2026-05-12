use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize, Ordering};
use std::process::exit;

macro_rules! test_incdec {
    ($type:ty, $value:expr, $preop:ident, $postop:ident, $pre_p:expr, $change:expr) => {{
        let a = std::sync::Arc::new(std::sync::atomic::Atomic::<$type>::new($value as $type));
        let a_clone = a.clone();
        let result = if $pre_p {
            a_clone.$preop(Ordering::SeqCst)
        } else {
            a_clone.$postop(Ordering::SeqCst)
        };
        if result != if $pre_p { ($value as $type) + $change } else { $value as $type } {
            panic!("Test failed");
        }
        if a.load(Ordering::SeqCst) != ($value as $type) + $change {
            panic!("Test failed");
        }
    }};
}

macro_rules! test_incdec_arith {
    ($value:expr, $preop:ident, $postop:ident, $pre_p:expr, $change:expr) => {{
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
        test_incdec_arith!($value, fetch_add, fetch_sub, true, 1);
        test_incdec_arith!($value, fetch_sub, fetch_add, true, -1);
        test_incdec_arith!($value, fetch_add, fetch_sub, false, 1);
        test_incdec_arith!($value, fetch_sub, fetch_add, false, -1);
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