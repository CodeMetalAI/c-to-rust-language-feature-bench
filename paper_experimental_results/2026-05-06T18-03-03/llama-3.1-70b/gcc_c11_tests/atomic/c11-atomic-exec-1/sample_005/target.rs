fn main() {
    test_simple_assign();
}

fn test_simple_assign() {
    macro_rules! test_simple_assign {
        ($value:expr) => {{
            let mut a: std::sync::atomic::AtomicUsize> = std::sync::atomic::Atomic::new(0);
            let b: std::sync::atomic::AtomicUsize> = std::sync::atomic::Atomic::new($value);
            assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), 0);
            assert_eq!(b.load(std::sync::atomic::Ordering::SeqCst), $value);
            a.store($value, std::sync::atomic::Ordering::SeqCst);
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
            test_simple_assign!(2.5 as usize);
            test_simple_assign!(3.5 as usize);
            let i = 0;
            let mut a: std::sync::atomic::Atomic<*const i32> = std::sync::atomic::Atomic::new(0 as *const i32);
            let b: std::sync::atomic::Atomic<*const i32> = std::sync::atomic::Atomic::new(&i as *const i32);
            assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), 0 as *const i32);
            assert_eq!(b.load(std::sync::atomic::Ordering::SeqCst), &i as *const i32);
            a.store(&i as *const i32, std::sync::atomic::Ordering::SeqCst);
            assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), &i as *const i32);
        }};
    }

    test_simple_assign_arith!();
    let mut s1 = std::sync::atomic::Atomic::new([0; 1024]);
    let mut s2 = std::sync::atomic::Atomic::new([0; 1024]);
    let mut init = [0; 1024];
    let mut copy = [0; 1024];
    for j in 0..1024 {
        init[j] = j;
    }
    copy = s1.swap(init, std::sync::atomic::Ordering::SeqCst);
    assert_eq!(init, copy);
    copy = s2.swap(s1.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst);
    assert_eq!(init, copy);
    copy = s1.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(init, copy);
    copy = s2.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(init, copy);
}