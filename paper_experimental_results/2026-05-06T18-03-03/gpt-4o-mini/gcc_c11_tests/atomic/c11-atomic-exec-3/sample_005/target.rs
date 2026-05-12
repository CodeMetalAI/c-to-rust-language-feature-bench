use std::sync::atomic::{AtomicBool, AtomicU8, AtomicI8, AtomicU16, AtomicI16, AtomicU32, AtomicI32, AtomicU64, AtomicI64, AtomicUsize, AtomicIsize, AtomicF32, AtomicF64};
use std::sync::atomic::Ordering;

fn test_incdec<T>(value: T, pre_op: fn(&Atomic<T>), post_op: fn(&Atomic<T>), pre_p: bool, change: T)
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::marker::Sized + std::fmt::Debug,
{
    let a = Atomic::<T>::new(value);
    let expected = if pre_p { value + change } else { value };
    pre_op(&a);
    assert_eq!(a.load(Ordering::SeqCst), expected);
    assert_eq!(a.load(Ordering::SeqCst), value + change);
}

fn test_all_incdec_arith(value: i32) {
    test_incdec(value, |a| { a.fetch_add(1, Ordering::SeqCst); }, |a| {}, true, 1);
    test_incdec(value, |a| { a.fetch_sub(1, Ordering::SeqCst); }, |a| {}, true, -1);
    test_incdec(value, |a| {}, |a| { a.fetch_add(1, Ordering::SeqCst); }, false, 1);
    test_incdec(value, |a| {}, |a| { a.fetch_sub(1, Ordering::SeqCst); }, false, -1);
}

fn test_incdec() {
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith(1 << 60);
    test_incdec(1.5, |a| { a.fetch_add(1, Ordering::SeqCst); }, |a| {}, true, 1.0);
    test_incdec(1.5, |a| { a.fetch_sub(1, Ordering::SeqCst); }, |a| {}, true, -1.0);
}

fn main() {
    test_incdec();
    std::process::exit(0);
}