use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize, Ordering};
use std::process;

macro_rules! test_incdec {
    ($type:ty, $value:expr, $preop:ident, $postop:ident, $pre_p:expr, $change:expr) => {
        {
            let a: &std::sync::atomic::Atomic<$type> = &std::sync::atomic::Atomic::new($value as $type);
            let mut pre_val = a.load(Ordering::Relaxed);
            if $preop {
                pre_val = a.fetch_add($change as $type, Ordering::Relaxed) + $change as $type;
            } else if $postop {
                pre_val = a.fetch_add($change as $type, Ordering::Relaxed);
            }
            if pre_val != if $pre_p { $value as $type + $change as $type } else { $value as $type } {
                abort();
            }
            if a.load(Ordering::Relaxed) != $value as $type + $change as $type {
                abort();
            }
        }
    };
}

macro_rules! test_incdec_arith {
    ($value:expr, $preop:ident, $postop:ident, $pre_p:expr, $change:expr) => {
        {
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
        }
    };
}

macro_rules! test_all_incdec_arith {
    ($value:expr) => {
        {
            test_incdec_arith!($value, true, false, true, 1);
            test_incdec_arith!($value, true, false, true, -1);
            test_incdec_arith!($value, false, true, false, 1);
            test_incdec_arith!($value, false, true, false, -1);
        }
    };
}

fn abort() -> ! {
    process::abort();
}

fn test_incdec() {
    test_all_incdec_arith!(0);
    test_all_incdec_arith!(1);
    test_all_incdec_arith!(2);
    test_all_incdec_arith!(-1);
    test_all_incdec_arith!(1u64 << 60);
    test_all_incdec_arith!(1.5_f64); // Floating-point types not supported in atomic operations in Rust
}

fn main() {
    test_incdec();
    process::exit(0);
}