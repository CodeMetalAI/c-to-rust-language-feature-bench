use std::sync::atomic::{AtomicBool, AtomicU8, AtomicI8, AtomicU16, AtomicI16, AtomicU32, AtomicI32, AtomicU64, AtomicI64, AtomicUsize, AtomicIsize, Ordering};

fn test_incdec<T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + From<u8> + std::fmt::Debug + PartialEq>(value: T, change: T, pre_op: fn(&Atomic<T>), post_op: fn(&Atomic<T>)) {
    let a = Atomic::new(value);
    let expected_pre = pre_op(&a);
    let expected_post = post_op(&a);
    
    assert_eq!(expected_pre, if change == T::from(1) { value + change } else { value });
    assert_eq!(a.load(Ordering::SeqCst), value + change);
}

fn test_all_incdec_arith(value: i32) {
    test_incdec(value, 1, |a| { a.fetch_add(1, Ordering::SeqCst) }, |a| { a.load(Ordering::SeqCst) });
    test_incdec(value, -1, |a| { a.fetch_sub(1, Ordering::SeqCst) }, |a| { a.load(Ordering::SeqCst) });
    test_incdec(value, 1, |a| { a.load(Ordering::SeqCst) }, |a| { a.fetch_add(1, Ordering::SeqCst) });
    test_incdec(value, -1, |a| { a.load(Ordering::SeqCst) }, |a| { a.fetch_sub(1, Ordering::SeqCst) });
}

fn test_incdec() {
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith(1 << 60);
    test_all_incdec_arith(1.5);
    
    let ia = vec![0; 2];
    let a = Atomic::new(&ia[1]);
    test_incdec(&a, 1, |a| { a.fetch_add(1, Ordering::SeqCst) }, |a| { a.load(Ordering::SeqCst) });
    test_incdec(&a, -1, |a| { a.fetch_sub(1, Ordering::SeqCst) }, |a| { a.load(Ordering::SeqCst) });
    test_incdec(&a, 1, |a| { a.load(Ordering::SeqCst) }, |a| { a.fetch_add(1, Ordering::SeqCst) });
    test_incdec(&a, -1, |a| { a.load(Ordering::SeqCst) }, |a| { a.fetch_sub(1, Ordering::SeqCst) });
}

fn main() {
    test_incdec();
    std::process::exit(0);
}