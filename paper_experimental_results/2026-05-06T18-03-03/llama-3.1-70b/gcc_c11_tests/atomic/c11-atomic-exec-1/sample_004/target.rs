fn main() {
    test_simple_assign();
    std::process::exit(0);
}

fn test_simple_assign() {
    macro_rules! test_simple_assign {
        ($($t:ty),*) => {
            $(
                let mut a: std::sync::atomic::Atomic<$t> = std::sync::atomic::Atomic::new(0);
                let b: std::sync::atomic::Atomic<$t> = std::sync::atomic::Atomic::new(1);
                assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), 0);
                assert_eq!(b.load(std::sync::atomic::Ordering::SeqCst), 1);
                a.store(b.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst);
                assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), 1);
            )*
        };
    }

    macro_rules! test_simple_assign_arith {
        ($($v:expr),*) => {
            $(
                test_simple_assign!(bool, char, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, std::num::NonZeroU8, std::num::NonZeroU16, std::num::NonZeroU32, std::num::NonZeroU64, std::num::NonZeroI8, std::num::NonZeroI16, std::num::NonZeroI32, std::num::NonZeroI64, *const i32);
                test_simple_assign!(std::complex::Complex<f32>, std::complex::Complex<f64>, std::complex::Complex<std::num::NonZeroU8>);
            )*
        };
    }

    test_simple_assign_arith!(0, 1, 2, -1, 1 << 63, 1.5, std::complex::Complex::new(2.5, 3.5));

    let mut init = [0; 1024];
    for i in 0..1024 {
        init[i] = i as i16;
    }
    let mut s1 = std::sync::atomic::Atomic::new(init);
    let mut s2 = std::sync::atomic::Atomic::new(init);
    let mut copy = s1.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(init, copy);
    copy = s2.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(init, copy);
    s2.store(s1.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst);
    copy = s2.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(init, copy);
}