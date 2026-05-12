use std::mem;
use std::process;

fn test_simple_assign() {
    fn test_simple_assign_arith<T: Copy + PartialEq + Default + std::fmt::Debug>(value: T) {
        let mut a: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let b: T = value;

        if a.load(std::sync::atomic::Ordering::SeqCst) != 0 {
            process::abort();
        }
        if b != value {
            process::abort();
        }
        a.store(b as usize, std::sync::atomic::Ordering::SeqCst);
        if a.load(std::sync::atomic::Ordering::SeqCst) != (value as usize) {
            process::abort();
        }
    }

    test_simple_assign_arith(0);
    test_simple_assign_arith(1);
    test_simple_assign_arith(2);
    test_simple_assign_arith(-1);
    test_simple_assign_arith(1u64 << 63);
    test_simple_assign_arith(1.5);
    test_simple_assign_arith((2.5, 3.5)); // Complex number representation

    let i: i32 = 0;
    test_simple_assign_arith(std::ptr::null::<i32>()); // Equivalent to C's NULL
    test_simple_assign_arith(&i as *const _); // Pointer to i

    struct S {
        a: [i16; 1024],
    }

    let mut init = S { a: [0; 1024] };
    let mut copy = S { a: [0; 1024] };
    let s1 = init.clone();
    let s2 = s1.clone();

    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    copy = s1.clone();
    if mem::size_of::<S>() != mem::size_of_val(&copy) {
        process::abort();
    }

    copy = s2.clone();
    if mem::size_of::<S>() != mem::size_of_val(&copy) {
        process::abort();
    }

    copy = s1.clone();
    if mem::size_of::<S>() != mem::size_of_val(&copy) {
        process::abort();
    }

    copy = s2.clone();
    if mem::size_of::<S>() != mem::size_of_val(&copy) {
        process::abort();
    }
}

fn main() {
    test_simple_assign();
    process::exit(0);
}