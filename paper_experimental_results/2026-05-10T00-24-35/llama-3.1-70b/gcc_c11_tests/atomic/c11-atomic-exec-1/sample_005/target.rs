fn main() {
    test_simple_assign();
    std::process::exit(0);
}

fn test_simple_assign() {
    macro_rules! test_simple_assign {
        ($value:expr) => {
            let mut a: std::sync::atomic::Atomic<$value> = std::sync::atomic::Atomic::new(0);
            let b: std::sync::atomic::Atomic<$value> = std::sync::atomic::Atomic::new($value);
            assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), 0);
            assert_eq!(b.load(std::sync::atomic::Ordering::SeqCst), $value);
            a.store(b.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst);
            assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), $value);
        };
    }

    macro_rules! test_simple_assign_arith {
        () => {
            test_simple_assign!(0);
            test_simple_assign!(1);
            test_simple_assign!(2);
            test_simple_assign!(-1);
            test_simple_assign!(1 << 63);
            test_simple_assign!(1.5f64);
            test_simple_assign!((2.5, 3.5));
            test_simple_assign!(std::ptr::null());
            test_simple_assign!(&0);
            let init: [u16; 1024] = [0; 1024].try_into().unwrap();
            let mut s1 = std::sync::atomic::Atomic::new(init);
            let mut s2 = std::sync::atomic::Atomic::new(init);
            for (i, x) in init.iter_mut().enumerate() {
                *x = i as u16;
            }
            let copy = s1.swap(init, std::sync::atomic::Ordering::SeqCst);
            assert_eq!(init, copy);
            let copy = s2.swap(s1.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst);
            assert_eq!(init, copy);
            let copy = s1.load(std::sync::atomic::Ordering::SeqCst);
            assert_eq!(init, copy);
            let copy = s2.load(std::sync::atomic::Ordering::SeqCst);
            assert_eq!(init, copy);
        };
    }

    test_simple_assign_arith!();
}