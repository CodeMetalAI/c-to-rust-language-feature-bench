use std::sync::atomic::{AtomicBool, AtomicChar, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, AtomicF32, AtomicF64, AtomicPtr};

macro_rules! test_simple_assign {
    ($type:ty, $value:expr) => {
        let a = Atomic::$type::new(0);
        let b = Atomic::$type::new($value);
        if a.load() != 0 {
            panic!();
        }
        if b.load() != $value {
            panic!();
        }
        if a.store(b.load()) != $value {
            panic!();
        }
        if a.load() != $value {
            panic!();
        }
    };
}

macro_rules! test_simple_assign_arith {
    ($($type:ty, $value:expr),*) => {
        $(test_simple_assign!($type, $value);)*
    };
}

fn test_simple_assign() {
    test_simple_assign_arith!(
        bool, 0,
        i8, 0,
        i16, 0,
        i32, 0,
        i64, 0,
        isize, 0,
        u8, 0,
        u16, 0,
        u32, 0,
        u64, 0,
        usize, 0,
        f32, 0.0,
        f64, 0.0,
        f64, 1.5,
        Complex<f32>, CMPLX(2.5, 3.5),
        Complex<f64>, CMPLX(2.5, 3.5),
    );
    let i = 0;
    test_simple_assign!(i32, 0);
    test_simple_assign!(i32, &i);
    let s = [0; 1024];
    let init = s.clone();
    let s1 = Atomic::new(s);
    let s2 = Atomic::new(s);
    for j in 0..1024 {
        init[j] = j;
    }
    let copy = s1.store(init);
    if init != copy {
        panic!();
    }
    let copy = s2.store(s1.load());
    if init != copy {
        panic!();
    }
    let copy = s1.load();
    if init != copy {
        panic!();
    }
    let copy = s2.load();
    if init != copy {
        panic!();
    }
}

fn main() {
    test_simple_assign();
    std::process::exit(0);
}

macro_rules! cmplx {
    ($re:expr, $im:expr) => {
        $re + $im * std::complex::Complex::new(0.0, 1.0)
    };
}

fn cmplx($re:expr, $im:expr) -> std::complex::Complex<f64> {
    cmplx!($re, $im)
}