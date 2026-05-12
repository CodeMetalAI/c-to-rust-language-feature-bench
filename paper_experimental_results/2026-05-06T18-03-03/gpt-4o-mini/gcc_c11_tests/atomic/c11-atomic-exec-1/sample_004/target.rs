use std::mem;
use std::process;

fn test_simple_assign() {
    fn test_simple_assign_arith<T: Copy + PartialEq + Default + std::fmt::Debug>(value: T) {
        let mut a: std::sync::atomic::AtomicPtr<T> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
        let b = value;

        if a.load(std::sync::atomic::Ordering::SeqCst).is_null() {
            panic!();
        }
        if b != value {
            panic!();
        }
        a.store(&b as *const _ as *mut _, std::sync::atomic::Ordering::SeqCst);
        if a.load(std::sync::atomic::Ordering::SeqCst) != &b as *const _ as *mut _ {
            panic!();
        }
        if a.load(std::sync::atomic::Ordering::SeqCst).is_null() {
            panic!();
        }
    }

    let values: Vec<Box<dyn std::any::Any>> = vec![
        Box::new(0u8),
        Box::new(1u8),
        Box::new(2u8),
        Box::new(-1i8),
        Box::new(1u64 << 63),
        Box::new(1.5f32),
        Box::new((2.5f32, 3.5f32)),
        Box::new(std::ptr::null_mut()),
        Box::new(&0 as *const _ as *mut _),
    ];

    for value in values {
        if let Some(v) = value.downcast_ref::<u8>() {
            test_simple_assign_arith(*v);
        } else if let Some(v) = value.downcast_ref::<i8>() {
            test_simple_assign_arith(*v);
        } else if let Some(v) = value.downcast_ref::<u64>() {
            test_simple_assign_arith(*v);
        } else if let Some(v) = value.downcast_ref::<f32>() {
            test_simple_assign_arith(*v);
        } else if let Some(v) = value.downcast_ref::<(f32, f32)>() {
            test_simple_assign_arith(*v);
        }
    }

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
        panic!();
    }
    copy = s2.clone();
    if mem::size_of::<S>() != mem::size_of_val(&copy) {
        panic!();
    }
}

fn main() {
    test_simple_assign();
    process::exit(0);
}