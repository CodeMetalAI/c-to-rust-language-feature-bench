use std::process::{abort, exit};

#[derive(Copy, Clone, PartialEq, Eq, Default)]
struct Complex<T> {
    re: T,
    im: T,
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct S {
    a: [i16; 1024],
}

impl Default for S {
    fn default() -> Self {
        S { a: [0; 1024] }
    }
}

macro_rules! test_simple_assign {
    ($t:ty, $v:expr) => {
        {
            let mut a: $t = Default::default();
            let b: $t = $v;
            if a != <$t>::default() {
                abort();
            }
            if b != $v {
                abort();
            }
            a = b;
            if a != $v {
                abort();
            }
        }
    };
}

fn test_simple_assign_arith() {
    // 0
    test_simple_assign!(bool, false);
    test_simple_assign!(i8, 0i8);
    test_simple_assign!(u8, 0u8);
    test_simple_assign!(i16, 0i16);
    test_simple_assign!(u16, 0u16);
    test_simple_assign!(i32, 0i32);
    test_simple_assign!(u32, 0u32);
    test_simple_assign!(isize, 0isize);
    test_simple_assign!(usize, 0usize);
    test_simple_assign!(i64, 0i64);
    test_simple_assign!(u64, 0u64);
    test_simple_assign!(f32, 0.0f32);
    test_simple_assign!(f64, 0.0f64);
    test_simple_assign!(Complex<f32>, Complex { re: 0.0, im: 0.0 });
    test_simple_assign!(Complex<f64>, Complex { re: 0.0, im: 0.0 });

    // 1
    test_simple_assign!(bool, true);
    test_simple_assign!(i8, 1i8);
    test_simple_assign!(u8, 1u8);
    test_simple_assign!(i16, 1i16);
    test_simple_assign!(u16, 1u16);
    test_simple_assign!(i32, 1i32);
    test_simple_assign!(u32, 1u32);
    test_simple_assign!(isize, 1isize);
    test_simple_assign!(usize, 1usize);
    test_simple_assign!(i64, 1i64);
    test_simple_assign!(u64, 1u64);
    test_simple_assign!(f32, 1.0f32);
    test_simple_assign!(f64, 1.0f64);
    test_simple_assign!(Complex<f32>, Complex { re: 1.0, im: 0.0 });
    test_simple_assign!(Complex<f64>, Complex { re: 1.0, im: 0.0 });

    // 2
    test_simple_assign!(bool, true);
    test_simple_assign!(i8, 2i8);
    test_simple_assign!(u8, 2u8);
    test_simple_assign!(i16, 2i16);
    test_simple_assign!(u16, 2u16);
    test_simple_assign!(i32, 2i32);
    test_simple_assign!(u32, 2u32);
    test_simple_assign!(isize, 2isize);
    test_simple_assign!(usize, 2usize);
    test_simple_assign!(i64, 2i64);
    test_simple_assign!(u64, 2u64);
    test_simple_assign!(f32, 2.0f32);
    test_simple_assign!(f64, 2.0f64);
    test_simple_assign!(Complex<f32>, Complex { re: 2.0, im: 0.0 });
    test_simple_assign!(Complex<f64>, Complex { re: 2.0, im: 0.0 });

    // -1
    test_simple_assign!(bool, true);
    test_simple_assign!(i8, -1i8);
    test_simple_assign!(u8, (-1i8) as u8);
    test_simple_assign!(i16, -1i16);
    test_simple_assign!(u16, (-1i16) as u16);
    test_simple_assign!(i32, -1i32);
    test_simple_assign!(u32, (-1i32) as u32);
    test_simple_assign!(isize, -1isize);
    test_simple_assign!(usize, (-1isize) as usize);
    test_simple_assign!(i64, -1i64);
    test_simple_assign!(u64, (-1i64) as u64);
    test_simple_assign!(f32, -1.0f32);
    test_simple_assign!(f64, -1.0f64);
    test_simple_assign!(Complex<f32>, Complex { re: -1.0, im: 0.0 });
    test_simple_assign!(Complex<f64>, Complex { re: -1.0, im: 0.0 });

    // 1u64 << 63
    let big = 1u64 << 63;
    test_simple_assign!(bool, true);
    test_simple_assign!(i8, big as i8);
    test_simple_assign!(u8, big as u8);
    test_simple_assign!(i16, big as i16);
    test_simple_assign!(u16, big as u16);
    test_simple_assign!(i32, big as i32);
    test_simple_assign!(u32, big as u32);
    test_simple_assign!(isize, big as isize);
    test_simple_assign!(usize, big as usize);
    test_simple_assign!(i64, big as i64);
    test_simple_assign!(u64, big);
    test_simple_assign!(f32, big as f32);
    test_simple_assign!(f64, big as f64);
    test_simple_assign!(Complex<f32>, Complex { re: big as f32, im: 0.0 });
    test_simple_assign!(Complex<f64>, Complex { re: big as f64, im: 0.0 });

    // 1.5
    test_simple_assign!(bool, true);
    test_simple_assign!(i8, 1.5f32 as i8);
    test_simple_assign!(u8, 1.5f32 as u8);
    test_simple_assign!(i16, 1.5f32 as i16);
    test_simple_assign!(u16, 1.5f32 as u16);
    test_simple_assign!(i32, 1.5f32 as i32);
    test_simple_assign!(u32, 1.5f32 as u32);
    test_simple_assign!(isize, 1.5f64 as isize);
    test_simple_assign!(usize, 1.5f64 as usize);
    test_simple_assign!(i64, 1.5f64 as i64);
    test_simple_assign!(u64, 1.5f64 as u64);
    test_simple_assign!(f32, 1.5f32);
    test_simple_assign!(f64, 1.5f64);
    test_simple_assign!(Complex<f32>, Complex { re: 1.5, im: 0.0 });
    test_simple_assign!(Complex<f64>, Complex { re: 1.5, im: 0.0 });

    // CMPLX(2.5, 3.5)
    test_simple_assign!(bool, true);
    test_simple_assign!(i8, 2.5f32 as i8);
    test_simple_assign!(u8, 2.5f32 as u8);
    test_simple_assign!(i16, 2.5f32 as i16);
    test_simple_assign!(u16, 2.5f32 as u16);
    test_simple_assign!(i32, 2.5f32 as i32);
    test_simple_assign!(u32, 2.5f32 as u32);
    test_simple_assign!(isize, 2.5f64 as isize);
    test_simple_assign!(usize, 2.5f64 as usize);
    test_simple_assign!(i64, 2.5f64 as i64);
    test_simple_assign!(u64, 2.5f64 as u64);
    test_simple_assign!(f32, 2.5f32);
    test_simple_assign!(f64, 2.5f64);
    test_simple_assign!(Complex<f32>, Complex { re: 2.5, im: 3.5 });
    test_simple_assign!(Complex<f64>, Complex { re: 2.5, im: 3.5 });
}

fn test_simple_assign() {
    test_simple_assign_arith();
    static mut I: i32 = 0;
    test_simple_assign!(*const i32, std::ptr::null());
    test_simple_assign!(*const i32, unsafe { &I as *const i32 });

    let mut s1: S = Default::default();
    let mut s2: S = Default::default();
    let mut init: S = Default::default();
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    let mut copy = { s1 = init; s1 };
    if init != copy {
        abort();
    }
    copy = { s2 = s1; s2 };
    if init != copy {
        abort();
    }
    copy = s1;
    if init != copy {
        abort();
    }
    copy = s2;
    if init != copy {
        abort();
    }
}

fn main() {
    test_simple_assign();
    exit(0);
}