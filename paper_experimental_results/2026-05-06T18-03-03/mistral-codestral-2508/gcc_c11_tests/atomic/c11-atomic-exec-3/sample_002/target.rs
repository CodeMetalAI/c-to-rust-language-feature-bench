use std::process;
use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize, Ordering};

macro_rules! test_incdec {
    ($atomic_type:ty, $value:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {
        {
            static A: $atomic_type = $atomic_type::new($value);
            if $preop A.fetch_add($change, Ordering::SeqCst) $postop != ($pre_p ? $value + $change : $value) {
                process::exit(1);
            }
            if A.load(Ordering::SeqCst) != $value + $change {
                process::exit(1);
            }
        }
    };
}

macro_rules! test_incdec_arith {
    ($value:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {
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
            test_incdec_arith!($value, ++, , 1, 1);
            test_incdec_arith!($value, --, , 1, -1);
            test_incdec_arith!($value, , ++, 0, 1);
            test_incdec_arith!($value, , --, 0, -1);
        }
    };
}

fn test_incdec() {
    test_all_incdec_arith!(0);
    test_all_incdec_arith!(1);
    test_all_incdec_arith!(2);
    test_all_incdec_arith!(-1);
    test_all_incdec_arith!(1 << 60);
    // Note: Floating point atomic operations are not supported in Rust's standard library.
    // The following line is commented out to maintain the same behavior as the original C code.
    // test_all_incdec_arith!(1.5);
    static mut IA: [i32; 2] = [0, 0];
    unsafe {
        test_incdec!(AtomicI32, &mut IA[1], ++, , 1, 1);
        test_incdec!(AtomicI32, &mut IA[1], --, , 1, -1);
        test_incdec!(AtomicI32, &mut IA[1], , ++, 0, 1);
        test_incdec!(AtomicI32, &mut IA[1], , --, 0, -1);
    }
}

fn main() {
    test_incdec();
    process::exit(0);
}