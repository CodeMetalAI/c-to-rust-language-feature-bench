use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, Ordering};
use std::process;

macro_rules! test_incdec {
    ($a:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {
        if $preop $a $postop != if $pre_p { $a.load(Ordering::SeqCst) + $change } else { $a.load(Ordering::SeqCst) } {
            process::exit(1);
        }
        if $a.load(Ordering::SeqCst) != $a.load(Ordering::SeqCst) + $change {
            process::exit(1);
        }
    };
}

macro_rules! test_incdec_arith {
    ($value:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {
        let a_bool: AtomicBool = AtomicBool::new($value != 0);
        test_incdec!(a_bool, $preop, $postop, $pre_p, $change);

        let a_i8: AtomicI8 = AtomicI8::new($value as i8);
        test_incdec!(a_i8, $preop, $postop, $pre_p, $change);

        let a_u8: AtomicU8 = AtomicU8::new($value as u8);
        test_incdec!(a_u8, $preop, $postop, $pre_p, $change);

        let a_i16: AtomicI16 = AtomicI16::new($value as i16);
        test_incdec!(a_i16, $preop, $postop, $pre_p, $change);

        let a_u16: AtomicU16 = AtomicU16::new($value as u16);
        test_incdec!(a_u16, $preop, $postop, $pre_p, $change);

        let a_i32: AtomicI32 = AtomicI32::new($value as i32);
        test_incdec!(a_i32, $preop, $postop, $pre_p, $change);

        let a_u32: AtomicU32 = AtomicU32::new($value as u32);
        test_incdec!(a_u32, $preop, $postop, $pre_p, $change);

        let a_i64: AtomicI64 = AtomicI64::new($value as i64);
        test_incdec!(a_i64, $preop, $postop, $pre_p, $change);

        let a_u64: AtomicU64 = AtomicU64::new($value as u64);
        test_incdec!(a_u64, $preop, $postop, $pre_p, $change);
    };
}

macro_rules! test_all_incdec_arith {
    ($value:expr) => {
        test_incdec_arith!($value, ++, , true, 1);
        test_incdec_arith!($value, --, , true, -1);
        test_incdec_arith!($value, , ++, false, 1);
        test_incdec_arith!($value, , --, false, -1);
    };
}

fn test_incdec() {
    test_all_incdec_arith!(0);
    test_all_incdec_arith!(1);
    test_all_incdec_arith!(2);
    test_all_incdec_arith!(-1);
    test_all_incdec_arith!(1 << 60);
    // Floating point values are not supported in Rust's atomic types.
    // Pointer values are not supported in Rust's atomic types.
}

fn main() {
    test_incdec();
    process::exit(0);
}