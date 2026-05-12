use std::mem;

#[derive(Debug, PartialEq, Eq)]
struct S {
    a: [i16; 1024],
}

fn test_simple_assign() {
    macro_rules! test_simple_assign {
        ($value:expr) => {{
            let mut a: std::sync::atomic::AtomicUsize> = std::sync::atomic::Atomic::new(0);
            let b: std::sync::atomic::AtomicUsize> = std::sync::atomic::Atomic::new($value);
            assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), 0);
            assert_eq!(b.load(std::sync::atomic::Ordering::SeqCst), $value);
            assert_eq!(a.store(b.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst), $value);
            assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), $value);
        }};
    }

    macro_rules! test_simple_assign_arith {
        () => {{
            test_simple_assign!(0);
            test_simple_assign!(1);
            test_simple_assign!(2);
            test_simple_assign!(-1);
            test_simple_assign!(1 << 63);
            test_simple_assign!(1.5 as usize);
        }};
    }

    test_simple_assign_arith!();

    let mut i = 0;
    let mut a: std::sync::atomic::Atomic<*mut i32> = std::sync::atomic::Atomic::new(&mut i as *mut i32);
    let b: std::sync::atomic::Atomic<*mut i32> = std::sync::atomic::Atomic::new(std::ptr::null_mut());
    assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), &mut i as *mut i32);
    assert_eq!(b.load(std::sync::atomic::Ordering::SeqCst), std::ptr::null_mut());
    assert_eq!(a.store(b.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst), std::ptr::null_mut());
    assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), std::ptr::null_mut());

    let mut init = S {
        a: [0; 1024],
    };
    let mut copy = S {
        a: [0; 1024],
    };
    let mut s1: std::sync::atomic::Atomic<S> = std::sync::atomic::Atomic::new(init);
    let mut s2: std::sync::atomic::Atomic<S> = std::sync::atomic::Atomic::new(init);
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    copy = s1.store(init, std::sync::atomic::Ordering::SeqCst);
    assert_eq!(init, copy);
    copy = s2.store(s1.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst);
    assert_eq!(init, copy);
    copy = s1.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(init, copy);
    copy = s2.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(init, copy);
}

fn main() {
    test_simple_assign();
}