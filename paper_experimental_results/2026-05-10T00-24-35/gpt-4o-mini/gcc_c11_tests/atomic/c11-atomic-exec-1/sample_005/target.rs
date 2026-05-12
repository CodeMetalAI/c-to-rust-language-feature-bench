use std::mem;
use std::process;

fn test_simple_assign() {
    fn test_simple_assign_arith<T: PartialEq + Copy + From<u8>>(value: T) {
        let mut a: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let b: T = value;

        if a.load(std::sync::atomic::Ordering::SeqCst) != 0 {
            process::abort();
        }

        if b != value {
            process::abort();
        }

        a.store(b.into(), std::sync::atomic::Ordering::SeqCst);
        if a.load(std::sync::atomic::Ordering::SeqCst) != b.into() {
            process::abort();
        }

        if a.load(std::sync::atomic::Ordering::SeqCst) != b.into() {
            process::abort();
        }
    }

    test_simple_assign_arith(0u8);
    test_simple_assign_arith(1u8);
    test_simple_assign_arith(2u8);
    test_simple_assign_arith(255u8); // Testing max value of u8
    test_simple_assign_arith(1.5f32);
    test_simple_assign_arith((2.5f32, 3.5f32)); // Complex-like tuple

    let i: i32 = 0;
    test_simple_assign_arith(std::ptr::null::<i32>());
    test_simple_assign_arith(&i);

    struct S {
        a: [i16; 1024],
    }

    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    let s1: std::sync::atomic::AtomicPtr<S> = std::sync::atomic::AtomicPtr::new(&init as *const _ as *mut _);
    let copy = unsafe { s1.load(std::sync::atomic::Ordering::SeqCst) };

    if mem::size_of_val(&init) != mem::size_of_val(&copy) {
        process::abort();
    }
}

fn main() {
    test_simple_assign();
    process::exit(0);
}