use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize, Ordering};
use std::process::exit;
use std::mem::size_of;
use std::ptr;

fn test_simple_assign<T: Default + Copy + PartialEq + std::fmt::Debug>(value: T) {
    let mut a = T::default();
    let b = value;
    assert_eq!(a, T::default());
    assert_eq!(b, value);
    a = b;
    assert_eq!(a, value);
}

fn test_simple_assign_atomic<T: Default + Copy + PartialEq + std::fmt::Debug>(value: T) {
    let a = std::sync::atomic::AtomicPtr::new(&mut T::default());
    let b = std::sync::atomic::AtomicPtr::new(&mut value);
    assert_eq!(unsafe { *a.load(Ordering::SeqCst) }, T::default());
    assert_eq!(unsafe { *b.load(Ordering::SeqCst) }, value);
    a.store(b.load(Ordering::SeqCst), Ordering::SeqCst);
    assert_eq!(unsafe { *a.load(Ordering::SeqCst) }, value);
}

fn test_simple_assign_arith(value: impl Copy) {
    test_simple_assign(value);
    test_simple_assign_atomic(value);
}

fn test_struct() {
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct S {
        a: [i16; 1024],
    }

    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    let mut copy = init;
    assert_eq!(init, copy);
    copy = init;
    assert_eq!(init, copy);
    copy = init;
    assert_eq!(init, copy);
    copy = init;
    assert_eq!(init, copy);
}

fn main() {
    test_simple_assign_arith(0);
    test_simple_assign_arith(1);
    test_simple_assign_arith(2);
    test_simple_assign_arith(-1);
    test_simple_assign_arith(1u64 << 63);
    test_simple_assign_arith(1.5f32);
    test_simple_assign_arith(1.5f64);
    test_simple_assign_arith((2.5f32, 3.5f32));
    test_simple_assign_arith((2.5f64, 3.5f64));
    test_simple_assign_arith((2.5f64, 3.5f64));
    test_struct();
    exit(0);
}