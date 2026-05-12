use std::process;

fn test_simple_assign() {
    fn test_simple_assign_arith<T: Copy + PartialEq + Default + std::fmt::Debug>(value: T) {
        let mut a: std::sync::atomic::AtomicPtr<T> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
        let b = std::sync::atomic::AtomicPtr::new(Box::into_raw(Box::new(value)));

        if !a.is_null() {
            process::abort();
        }
        if unsafe { *b.get_mut() } != value {
            process::abort();
        }
        a.store(b.as_ptr(), std::sync::atomic::Ordering::SeqCst);
        if unsafe { *a.load(std::sync::atomic::Ordering::SeqCst) } != value {
            process::abort();
        }
    }

    test_simple_assign_arith(0);
    test_simple_assign_arith(1);
    test_simple_assign_arith(2);
    test_simple_assign_arith(-1);
    test_simple_assign_arith(1u64 << 63);
    test_simple_assign_arith(1.5);
    test_simple_assign_arith((2.5, 3.5));

    let mut i = 0;
    test_simple_assign_arith(std::ptr::null_mut::<i32>());
    test_simple_assign_arith(&mut i);

    struct S {
        a: [i16; 1024],
    }
    let mut init = S { a: [0; 1024] };
    let mut copy = S { a: [0; 1024] };
    
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    
    let s1 = init.clone();
    copy = s1.clone();
    if copy != init {
        process::abort();
    }
    
    let s2 = s1.clone();
    copy = s2.clone();
    if copy != init {
        process::abort();
    }
    
    copy = s1.clone();
    if copy != init {
        process::abort();
    }
    
    copy = s2.clone();
    if copy != init {
        process::abort();
    }
}

fn main() {
    test_simple_assign();
    process::exit(0);
}