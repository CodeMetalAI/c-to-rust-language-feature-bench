use std::sync::atomic::{AtomicBool, AtomicChar, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, AtomicUsize, Ordering};

fn test_incdec<T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::marker::Copy>(
    value: T,
    preop: fn(T) -> T,
    postop: fn(T) -> T,
    pre_p: bool,
    change: T,
) {
    let a = Atomic::new(value);
    let expected = if pre_p { value + change } else { value };
    let result = postop(a.load(Ordering::SeqCst));
    if result != expected {
        panic!("Unexpected result");
    }
    if a.load(Ordering::SeqCst) != expected {
        panic!("Unexpected atomic value");
    }
}

fn test_incdec_arith<T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::marker::Copy>(
    value: T,
    preop: fn(T) -> T,
    postop: fn(T) -> T,
    pre_p: bool,
    change: T,
) {
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
}

fn test_all_incdec_arith<T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::marker::Copy>(
    value: T,
) {
    test_incdec_arith(value, |x| x + 1, |x| x + 1, true, 1);
    test_incdec_arith(value, |x| x - 1, |x| x - 1, true, -1);
    test_incdec_arith(value, |x| x + 1, |x| x + 1, false, 1);
    test_incdec_arith(value, |x| x - 1, |x| x - 1, false, -1);
}

fn test_incdec_ptr<T: Copy + std::marker::Copy>(
    value: T,
    preop: fn(T) -> T,
    postop: fn(T) -> T,
    pre_p: bool,
    change: T,
) {
    let a = Atomic::new(value);
    let expected = if pre_p { value + change } else { value };
    let result = postop(a.load(Ordering::SeqCst));
    if result != expected {
        panic!("Unexpected result");
    }
    if a.load(Ordering::SeqCst) != expected {
        panic!("Unexpected atomic value");
    }
}

fn test_incdec_ptr_arith<T: Copy + std::marker::Copy>(
    value: T,
    preop: fn(T) -> T,
    postop: fn(T) -> T,
    pre_p: bool,
    change: T,
) {
    test_incdec_ptr(value, preop, postop, pre_p, change);
    test_incdec_ptr(value, preop, postop, pre_p, change);
    test_incdec_ptr(value, preop, postop, pre_p, change);
    test_incdec_ptr(value, preop, postop, pre_p, change);
}

fn main() {
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