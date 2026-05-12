use std::mem;
use std::process;

fn test_simple_assign() {
    fn test_simple_assign_arith<T: Copy + PartialEq + Default + std::fmt::Debug>(value: T) {
        let mut a: std::sync::atomic::AtomicPtr<T> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
        let b: T = value;

        if a.load(std::sync::atomic::Ordering::SeqCst).is_null() {
            panic!();
        }
        if b != value {
            panic!();
        }
        a.store(&b as *const T as *mut T, std::sync::atomic::Ordering::SeqCst);
        if a.load(std::sync::atomic::Ordering::SeqCst) != &b as *const T as *mut T {
            panic!();
        }
        if a.load(std::sync::atomic::Ordering::SeqCst).is_null() {
            panic!();
        }
    }

    test_simple_assign_arith(0);
    test_simple_assign_arith(1);
    test_simple_assign_arith(2);
    test_simple_assign_arith(-1);
    test_simple_assign_arith(1u64 << 63);
    test_simple_assign_arith(1.5);
    test_simple_assign_arith((2.5, 3.5));

    let mut i: i32 = 0;
    test_simple_assign_arith(std::ptr::null_mut());
    test_simple_assign_arith(&i as *const i32);

    struct S {
        a: [i16; 1024],
    }

    let mut init = S { a: [0; 1024] };
    let mut copy = S { a: [0; 1024] };
    let s1: S;
    let s2: S;

    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    copy = init;
    if mem::size_of::<S>() != mem::size_of_val(&copy) {
        panic!();
    }

    copy = s1;
    if mem::size_of::<S>() != mem::size_of_val(&copy) {
        panic!();
    }

    copy = s2;
    if mem::size_of::<S>() != mem::size_of_val(&copy) {
        panic!();
    }
}

fn main() {
    test_simple_assign();
    process::exit(0);
}