use std::process::exit;
use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicI128, AtomicU128, AtomicIsize, AtomicUsize, Ordering};

fn test_incdec<T>(value: T, pre_op: bool, post_op: bool, pre_p: bool, change: T)
where
    T: Copy + PartialEq + std::ops::AddAssign + std::ops::SubAssign,
{
    let mut a = value;
    let result = if pre_op {
        if post_op {
            a += change;
            a
        } else {
            a += change;
            a
        }
    } else if post_op {
        let temp = a;
        a += change;
        temp
    } else {
        let temp = a;
        a += change;
        temp
    };

    if result != if pre_p { value + change } else { value } {
        exit(1);
    }
    if a != value + change {
        exit(1);
    }
}

fn test_incdec_arith<T>(value: T, pre_op: bool, post_op: bool, pre_p: bool, change: T)
where
    T: Copy + PartialEq + std::ops::AddAssign + std::ops::SubAssign,
{
    test_incdec(value, pre_op, post_op, pre_p, change);
}

fn test_all_incdec_arith<T>(value: T)
where
    T: Copy + PartialEq + std::ops::AddAssign + std::ops::SubAssign,
{
    test_incdec_arith(value, true, false, true, value + value);
    test_incdec_arith(value, true, false, true, value - value);
    test_incdec_arith(value, false, true, false, value + value);
    test_incdec_arith(value, false, true, false, value - value);
}

fn main() {
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith(1 << 60);
    test_all_incdec_arith(1.5);

    let mut ia = [0, 0];
    test_incdec(&mut ia[1], true, false, true, 1);
    test_incdec(&mut ia[1], true, false, true, -1);
    test_incdec(&mut ia[1], false, true, false, 1);
    test_incdec(&mut ia[1], false, true, false, -1);

    exit(0);
}