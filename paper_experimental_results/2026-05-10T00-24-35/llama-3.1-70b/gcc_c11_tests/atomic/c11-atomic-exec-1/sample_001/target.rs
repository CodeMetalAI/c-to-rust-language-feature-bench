use std::mem::size_of;

fn test_simple_assign() {
    macro_rules! test_simple_assign {
        ($t:ty, $value:expr) => {
            let mut a: std::sync::atomic::Atomic<$t> = std::sync::atomic::Atomic::new(0);
            let b: std::sync::atomic::Atomic<$t> = std::sync::atomic::Atomic::new($value);
            assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), 0);
            assert_eq!(b.load(std::sync::atomic::Ordering::SeqCst), $value);
            a.store(b.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst);
            assert_eq!(a.load(std::sync::atomic::Atomic::<$t>::load(&a, std::sync::atomic::Ordering::SeqCst), $value);
        };
    }

    macro_rules! test_simple_assign_arith {
        ($value:expr) => {
            test_simple_assign!(bool, $value != 0);
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
            test_simple_assign!(std::complex::Complex64, std::complex::Complex64::new($value as f32, 0.0));
            test_simple_assign!(std::complex::Complex128, std::complex::Complex128::new($value as f64, 0.0));
        };
    }

    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!(1 << 63);
    test_simple_assign_arith!(1.5);
    test_simple_assign_arith!(std::complex::Complex64::new(2.5, 3.5).re);

    let i: i32 = 0;
    test_simple_assign!(*const i32, &i as *const i32);

    #[repr(C)]
    struct S {
        a: [i16; 1024],
    }

    let mut init = S {
        a: [0; 1024],
    };
    let mut copy = S {
        a: [0; 1024],
    };
    let mut s1 = std::sync::atomic::Atomic::new(init);
    let mut s2 = std::sync::atomic::Atomic::new(init);

    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    copy = s1.swap(init, std::sync::atomic::Ordering::SeqCst);
    assert_eq!(&init as *const _ as usize, &copy as *const _ as usize);
    copy = s2.swap(s1.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst);
    assert_eq!(&init as *const _ as usize, &copy as *const _ as usize);
    copy = s1.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(&init as *const _ as usize, &copy as *const _ as usize);
    copy = s2.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(&init as *const _ as usize, &copy as *const _ as usize);
}

fn main() {
    test_simple_assign();
}