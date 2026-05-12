use std::process::exit;
use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicI128, AtomicU128, AtomicIsize, AtomicUsize, Ordering};

fn test_incdec<T>(value: T, pre_op: bool, post_op: bool, pre_p: bool, change: T)
where
    T: Copy + PartialEq + std::ops::AddAssign + std::ops::SubAssign + std::fmt::Debug,
{
    let mut a = value;

    if pre_op {
        a += change;
        if a != (value + change) {
            eprintln!("Test failed: pre_op {:?} {:?} {:?} {:?}", value, pre_op, post_op, change);
            exit(1);
        }
    } else if post_op {
        let old_a = a;
        a += change;
        if old_a != value {
            eprintln!("Test failed: post_op {:?} {:?} {:?} {:?}", value, pre_op, post_op, change);
            exit(1);
        }
        if a != (value + change) {
            eprintln!("Test failed: post_op {:?} {:?} {:?} {:?}", value, pre_op, post_op, change);
            exit(1);
        }
    } else {
        if pre_p {
            a += change;
            if a != (value + change) {
                eprintln!("Test failed: pre_p {:?} {:?} {:?} {:?}", value, pre_op, post_op, change);
                exit(1);
            }
        } else {
            a += change;
            if a != (value + change) {
                eprintln!("Test failed: pre_p {:?} {:?} {:?} {:?}", value, pre_op, post_op, change);
                exit(1);
            }
        }
    }
}

fn test_incdec_arith<T>(value: T, pre_op: bool, post_op: bool, pre_p: bool, change: T)
where
    T: Copy + PartialEq + std::ops::AddAssign + std::ops::SubAssign + std::fmt::Debug,
{
    test_incdec(value, pre_op, post_op, pre_p, change);
}

fn test_all_incdec_arith<T>(value: T)
where
    T: Copy + PartialEq + std::ops::AddAssign + std::ops::SubAssign + std::fmt::Debug,
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