fn test_simple_assign() {
    macro_rules! test_simple_assign {
        ($type:ty, $value:expr) => {{
            let mut a: std::sync::atomic::Atomic<$type> = std::sync::atomic::Atomic::new(0);
            let b: std::sync::atomic::Atomic<$type> = std::sync::atomic::Atomic::new($value);
            assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), 0);
            assert_eq!(b.load(std::sync::atomic::Ordering::SeqCst), $value);
            assert_eq!(a.store(b.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst), $value);
            assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), $value);
        }};
    }

    macro_rules! test_simple_assign_arith {
        ($value:expr) => {{
            test_simple_assign!(bool, $value!= 0);
            test_simple_assign!(i8, $value as i8);
            test_simple_assign!(u8, $value as u8);
            test_simple_assign!(i16, $value as i16);
            test_simple_assign!(u16, $value as u16);
            test_simple_assign!(i32, $value as i32);
            test_simple_assign!(u32, $value as u32);
            test_simple_assign!(i64, $value as i64);
            test_simple_assign!(u64, $value as u64);
            test_simple_assign!(f32, $value as f32);
            test_simple_assign!(f64, $value as f64);
            // no direct equivalent of _Complex float, _Complex double, _Complex long double in Rust
        }};
    }

    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!(1 << 63);
    test_simple_assign_arith!(1.5);
    // no direct equivalent of CMPLX(2.5, 3.5) in Rust
    test_simple_assign!(*const i32, &0 as *const i32);
    let i = 0;
    test_simple_assign!(*const i32, &i as *const i32);
    let init = [0; 1024];
    let mut copy = [0; 1024];
    let s1 = std::sync::atomic::Atomic::new(init);
    let s2 = std::sync::atomic::Atomic::new(init);
    copy = s1.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(init, copy);
    copy = s2.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(init, copy);
    copy = s1.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(init, copy);
    copy = s2.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(init, copy);
}

fn main() {
    test_simple_assign();
    std::process::exit(0);
}