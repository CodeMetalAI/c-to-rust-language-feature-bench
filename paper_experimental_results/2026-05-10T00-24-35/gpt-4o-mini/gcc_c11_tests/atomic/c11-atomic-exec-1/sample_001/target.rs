use std::mem;
use std::process;

fn test_simple_assign() {
    fn test_simple_assign_arith<T: PartialEq + Copy + Default + std::fmt::Debug>(value: T) {
        let mut a: std::sync::atomic::AtomicPtr<T> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
        let b = value;

        if a.load(std::sync::atomic::Ordering::SeqCst) != std::ptr::null_mut() {
            process::abort();
        }
        if b != value {
            process::abort();
        }

        a.store(&b as *const T as *mut T, std::sync::atomic::Ordering::SeqCst);
        if a.load(std::sync::atomic::Ordering::SeqCst) != &b as *const T as *mut T {
            process::abort();
        }
    }

    test_simple_assign_arith(0u8);
    test_simple_assign_arith(1u8);
    test_simple_assign_arith(2u8);
    test_simple_assign_arith(255u8); // -1 would be 255 for unsigned
    test_simple_assign_arith(1u64 << 63);
    test_simple_assign_arith(1.5f32);

    let complex_value = (2.5f32, 3.5f32);
    test_simple_assign_arith(complex_value);

    static mut I: i32 = 0;
    test_simple_assign_arith(std::ptr::null_mut());
    test_simple_assign_arith(&mut I as *mut _);

    struct S {
        a: [i16; 1024],
    }

    let mut init = S { a: [0; 1024] };
    let mut copy = S { a: [0; 1024] };
    
    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    let s1 = init;
    copy = s1;
    if mem::size_of::<S>() != mem::size_of_val(&copy) {
        process::abort();
    }

    let s2 = s1;
    copy = s2;
    if mem::size_of::<S>() != mem::size_of_val(&copy) {
        process::abort();
    }

    copy = s1;
    if mem::size_of::<S>() != mem::size_of_val(&copy) {
        process::abort();
    }

    copy = s2;
    if mem::size_of::<S>() != mem::size_of_val(&copy) {
        process::abort();
    }
}

fn main() {
    test_simple_assign();
    process::exit(0);
}