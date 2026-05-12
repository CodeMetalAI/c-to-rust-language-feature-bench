use std::sync::atomic::{AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::atomic::AtomicPtr;

fn test_incdec<T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Not + std::ops::AddAssign + std::ops::SubAssign + std::ops::Not<Output = T> + std::fmt::Debug>(value: T) {
    let a = std::sync::atomic::AtomicPtr::new(&value);
    let b = std::sync::atomic::AtomicPtr::new(&value);

    if a.load(Ordering::SeqCst)!= &value {
        panic!();
    }
    if b.load(Ordering::SeqCst)!= &value {
        panic!();
    }

    a.fetch_add(&value, Ordering::SeqCst);
    b.fetch_sub(&value, Ordering::SeqCst);

    if a.load(Ordering::SeqCst)!= &(value + value) {
        panic!();
    }
    if b.load(Ordering::SeqCst)!= &(value - value) {
        panic!();
    }
}

fn test_all_incdec_arith<T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Not + std::ops::AddAssign + std::ops::SubAssign + std::ops::Not<Output = T> + std::fmt::Debug>(value: T) {
    test_incdec(value);
    test_incdec(-value);
    test_incdec(value + value);
    test_incdec(value - value);
}

fn test_incdec_arith() {
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith(1 << 60);
    test_all_incdec_arith(1.5);
}

fn test_incdec_ptr() {
    let mut ia = [0, 0];
    let a = AtomicPtr::new(&ia[1]);
    let b = AtomicPtr::new(&ia[1]);

    if a.load(Ordering::SeqCst)!= &ia[1] {
        panic!();
    }
    if b.load(Ordering::SeqCst)!= &ia[1] {
        panic!();
    }

    a.fetch_add(1, Ordering::SeqCst);
    b.fetch_sub(1, Ordering::SeqCst);

    if a.load(Ordering::SeqCst)!= &ia[2] {
        panic!();
    }
    if b.load(Ordering::SeqCst)!= &ia[0] {
        panic!();
    }
}

fn main() {
    test_incdec_arith();
    test_incdec_ptr();
}