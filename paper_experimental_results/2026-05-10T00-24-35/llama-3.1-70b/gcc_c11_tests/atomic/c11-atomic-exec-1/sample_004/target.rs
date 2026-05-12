#![allow(unused)]

fn main() {
    test_simple_assign();
    std::process::exit(0);
}

fn test_simple_assign() {
    macro_rules! test_simple_assign {
        ($t:ty, $v:expr) => {
            let mut a: std::sync::atomic::Atomic<$t> = std::sync::atomic::Atomic::new(std::mem::MaybeUninit::uninit());
            let b: std::sync::atomic::Atomic<$t> = std::sync::atomic::Atomic::new($v);
            if a.load(std::sync::atomic::Ordering::SeqCst)!= std::mem::MaybeUninit::uninit() {
                panic!();
            }
            if b.load(std::sync::atomic::Ordering::SeqCst)!= $v {
                panic!();
            }
            a.store(b.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst);
            if a.load(std::sync::atomic::Ordering::SeqCst)!= $v {
                panic!();
            }
        };
    }

    macro_rules! test_simple_assign_arith {
        ($v:expr) => {
            test_simple_assign!(bool, $v as bool);
            test_simple_assign!(i8, $v as i8);
            test_simple_assign!(u8, $v as u8);
            test_simple_assign!(i16, $v as i16);
            test_simple_assign!(u16, $v as u16);
            test_simple_assign!(i32, $v as i32);
            test_simple_assign!(u32, $v as u32);
            test_simple_assign!(i64, $v as i64);
            test_simple_assign!(u64, $v as u64);
            test_simple_assign!(i128, $v as i128);
            test_simple_assign!(u128, $v as u128);
            test_simple_assign!(f32, $v as f32);
            test_simple_assign!(f64, $v as f64);
            test_simple_assign!(std::num::NonZeroU8, $v as std::num::NonZeroU8);
            test_simple_assign!(std::num::NonZeroU16, $v as std::num::NonZeroU16);
            test_simple_assign!(std::num::NonZeroU32, $v as std::num::NonZeroU32);
            test_simple_assign!(std::num::NonZeroU64, $v as std::num::NonZeroU64);
            test_simple_assign!(std::num::NonZeroU128, $v as std::num::NonZeroU128);
        };
    }

    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!(1u64 << 63);
    test_simple_assign_arith!(1.5);
    test_simple_assign!(std::complex::Complex64, std::complex::Complex64::new(2.5, 3.5));

    let mut i: i32 = 0;
    test_simple_assign!(*const i32, 0 as *const i32);
    test_simple_assign!(*const i32, &i as *const i32);

    #[repr(C)]
    struct S {
        a: [i16; 1024],
    }

    let init = S {
        a: [0; 1024],
    };
    let mut s1 = std::sync::atomic::Atomic::new(init);
    let mut s2 = std::sync::atomic::Atomic::new(init);
    let mut copy = S {
        a: [0; 1024],
    };

    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    copy = s1.swap(init, std::sync::atomic::Ordering::SeqCst);
    if init.a!= copy.a {
        panic!();
    }

    copy = s2.swap(s1.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst);
    if init.a!= copy.a {
        panic!();
    }

    copy = s1.load(std::sync::atomic::Ordering::SeqCst);
    if init.a!= copy.a {
        panic!();
    }

    copy = s2.load(std::sync::atomic::Ordering::SeqCst);
    if init.a!= copy.a {
        panic!();
    }
}