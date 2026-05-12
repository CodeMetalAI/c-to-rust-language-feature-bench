use std::sync::atomic::{AtomicBool, AtomicChar, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};

fn test_incdec<T: std::fmt::Debug + Copy + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign + std::ops::Add + std::ops::Sub + std::ops::Mul + std::ops::Div + std::cmp::PartialEq + std::cmp::Eq + std::marker::Copy>(value: T, preop: impl FnOnce(T) -> T, postop: impl FnOnce(T) -> T, pre_p: bool, change: T) {
    let a = Atomic::new(value);
    let expected = if pre_p { value + change } else { value };
    let result = postop(a.load(Ordering::SeqCst));
    if result!= expected {
        panic!("Expected {} but got {}", expected, result);
    }
    if a.load(Ordering::SeqCst)!= expected {
        panic!("Expected {} but got {}", expected, a.load(Ordering::SeqCst));
    }
}

fn test_incdec_arith<T: std::fmt::Debug + Copy + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign + std::ops::Add + std::ops::Sub + std::ops::Mul + std::ops::Div + std::cmp::PartialEq + std::cmp::Eq + std::marker::Copy>(value: T, preop: impl FnOnce(T) -> T, postop: impl FnOnce(T) -> T, pre_p: bool, change: T) {
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, |x| x + change, postop, pre_p, change);
    test_incdec(value, preop, |x| x + change, pre_p, change);
    test_incdec(value, |x| x - change, postop, pre_p, change);
    test_incdec(value, preop, |x| x - change, pre_p, change);
}

fn test_all_incdec_arith<T: std::fmt::Debug + Copy + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign + std::ops::Add + std::ops::Sub + std::ops::Mul + std::ops::Div + std::cmp::PartialEq + std::cmp::Eq + std::marker::Copy>(value: T) {
    test_incdec_arith(value, |x| x + 1, |x| x + 1, true, 1);
    test_incdec_arith(value, |x| x - 1, |x| x - 1, true, -1);
    test_incdec_arith(value, |x| x + 1, |x| x + 1, false, 1);
    test_incdec_arith(value, |x| x - 1, |x| x - 1, false, -1);
}

fn test_incdec_ptr<T: std::fmt::Debug + Copy + std::marker::Copy>(value: T, preop: impl FnOnce(T) -> T, postop: impl FnOnce(T) -> T, pre_p: bool, change: T) {
    let a = Atomic::new(value);
    let expected = if pre_p { value + change } else { value };
    let result = postop(a.load(Ordering::SeqCst));
    if result!= expected {
        panic!("Expected {} but got {}", expected, result);
    }
    if a.load(Ordering::SeqCst)!= expected {
        panic!("Expected {} but got {}", expected, a.load(Ordering::SeqCst));
    }
}

fn test_incdec_ptr_arith<T: std::fmt::Debug + Copy + std::marker::Copy>(value: T, preop: impl FnOnce(T) -> T, postop: impl FnOnce(T) -> T, pre_p: bool, change: T) {
    test_incdec_ptr(value, preop, postop, pre_p, change);
    test_incdec_ptr(value, |x| x + change, postop, pre_p, change);
    test_incdec_ptr(value, preop, |x| x + change, pre_p, change);
    test_incdec_ptr(value, |x| x - change, postop, pre_p, change);
    test_incdec_ptr(value, preop, |x| x - change, pre_p, change);
}

fn test_incdec() {
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith(1u64 << 60);
    test_all_incdec_arith(1.5);
    let ia = [0, 0];
    test_incdec_ptr_arith(&ia[1], |x| x + 1, |x| x + 1, true, 1);
    test_incdec_ptr_arith(&ia[1], |x| x - 1, |x| x - 1, true, -1);
    test_incdec_ptr_arith(&ia[1], |x| x + 1, |x| x + 1, false, 1);
    test_incdec_ptr_arith(&ia[1], |x| x - 1, |x| x - 1, false, -1);
}

fn main() {
    test_incdec();
    std::process::exit(0);
}