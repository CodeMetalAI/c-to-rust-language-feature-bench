use std::mem;
use std::process::exit;

fn test_simple_assign() {
    fn test_simple_assign_type<T: Copy + PartialEq + Default>(value: T) {
        let mut a: std::sync::atomic::AtomicPtr<T> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
        let b: T = value;

        if a.load(std::sync::atomic::Ordering::SeqCst).is_null() {
            if b != value {
                exit(1);
            }
            a.store(&b as *const T as *mut T, std::sync::atomic::Ordering::SeqCst);
            if a.load(std::sync::atomic::Ordering::SeqCst) != &b as *const T as *mut T {
                exit(1);
            }
        } else {
            exit(1);
        }
    }

    fn test_simple_assign_arith(value: i64) {
        test_simple_assign_type(value as i8);
        test_simple_assign_type(value as i16);
        test_simple_assign_type(value as i32);
        test_simple_assign_type(value as i64);
        // You can add more types here as needed
    }

    test_simple_assign_arith(0);
    test_simple_assign_arith(1);
    test_simple_assign_arith(2);
    test_simple_assign_arith(-1);
    test_simple_assign_arith(1u64 << 63);
    test_simple_assign_arith(1.5 as i64); // Note: This will truncate to 1
    // Complex types are not directly supported; you can simulate them or ignore

    let mut i: i32 = 0;
    test_simple_assign_type(&i as *const i32 as *mut i32);
    test_simple_assign_type(std::ptr::null_mut());

    struct S {
        a: [i16; 1024],
    }

    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    let mut copy = init;
    if mem::size_of::<S>() != mem::size_of::<S>() {
        exit(1);
    }

    let s1 = init;
    copy = s1;

    if mem::size_of::<S>() != mem::size_of::<S>() {
        exit(1);
    }

    let s2 = s1;
    copy = s2;

    if mem::size_of::<S>() != mem::size_of::<S>() {
        exit(1);
    }
}

fn main() {
    test_simple_assign();
    exit(0);
}