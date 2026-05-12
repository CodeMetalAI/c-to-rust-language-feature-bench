use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::atomic::AtomicPtr;

fn test_incdec<T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + PartialEq + std::fmt::Debug>(value: T, change: T) {
    let a = std::sync::atomic::AtomicUsize::new(value as usize);
    let b = std::sync::atomic::AtomicUsize::new(value as usize);
    let c = std::sync::atomic::AtomicUsize::new(value as usize);
    let d = std::sync::atomic::AtomicUsize::new(value as usize);

    let pre = a.load(Ordering::SeqCst);
    a.fetch_add(change as usize, Ordering::SeqCst);
    let post = a.load(Ordering::SeqCst);
    assert_eq!(pre + change as usize, post);

    let pre = b.load(Ordering::SeqCst);
    b.fetch_sub(change as usize, Ordering::SeqCst);
    let post = b.load(Ordering::SeqCst);
    assert_eq!(pre - change as usize, post);

    let pre = c.load(Ordering::SeqCst);
    c.fetch_add(change as usize, Ordering::SeqCst);
    let post = c.load(Ordering::SeqCst);
    assert_eq!(pre + change as usize, post);

    let pre = d.load(Ordering::SeqCst);
    d.fetch_sub(change as usize, Ordering::SeqCst);
    let post = d.load(Ordering::SeqCst);
    assert_eq!(pre - change as usize, post);
}

fn test_all_incdec_arith<T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + PartialEq + std::fmt::Debug>(value: T) {
    test_incdec(value, T::from(1));
    test_incdec(value, T::from(-1));
}

fn test_incdec_ptr() {
    let ia = [0, 0];
    let a = AtomicPtr::new(&ia[1]);
    let b = AtomicPtr::new(&ia[1]);
    let c = AtomicPtr::new(&ia[1]);
    let d = AtomicPtr::new(&ia[1]);

    let pre = a.load(Ordering::SeqCst);
    a.fetch_add(1, Ordering::SeqCst);
    let post = a.load(Ordering::SeqCst);
    assert_eq!(pre.offset(1), post);

    let pre = b.load(Ordering::SeqCst);
    b.fetch_sub(1, Ordering::SeqCst);
    let post = b.load(Ordering::SeqCst);
    assert_eq!(pre.offset(-1), post);

    let pre = c.load(Ordering::SeqCst);
    c.fetch_add(1, Ordering::SeqCst);
    let post = c.load(Ordering::SeqCst);
    assert_eq!(pre.offset(1), post);

    let pre = d.load(Ordering::SeqCst);
    d.fetch_sub(1, Ordering::SeqCst);
    let post = d.load(Ordering::SeqCst);
    assert_eq!(pre.offset(-1), post);
}

fn main() {
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith(1 << 60);
    test_all_incdec_arith(1.5);
    test_incdec_ptr();
}