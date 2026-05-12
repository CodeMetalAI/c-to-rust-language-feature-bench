use std::process;

#[derive(Copy, Clone, PartialEq)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Self {
        Self { re, im }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct S {
    a: [i16; 1024],
}

fn abort() -> ! {
    process::abort()
}

fn test_assign<T: Copy + PartialEq>(zero: T, value: T) {
    let mut a = zero;
    let b = value;
    if a != zero {
        abort();
    }
    if b != value {
        abort();
    }
    a = b;
    if a != value {
        abort();
    }
}

macro_rules! test_int_type {
    ($ty:ty) => {{
        test_assign(0 as $ty, 0 as $ty);
        test_assign(0 as $ty, 1 as $ty);
        test_assign(0 as $ty, 2 as $ty);
        test_assign(0 as $ty, (-1i64) as $ty);
        test_assign(0 as $ty, (1u64 << 63) as $ty);
        test_assign(0 as $ty, (1.5f64) as $ty);
        test_assign(0 as $ty, (2.5f64) as $ty);
    }};
}

macro_rules! test_float_type {
    ($ty:ty) => {{
        test_assign(0 as $ty, 0 as $ty);
        test_assign(0 as $ty, 1 as $ty);
        test_assign(0 as $ty, 2 as $ty);
        test_assign(0 as $ty, (-1i32) as $ty);
        test_assign(0 as $ty, (1u64 << 63) as $ty);
        test_assign(0 as $ty, 1.5 as $ty);
        test_assign(0 as $ty, 2.5 as $ty);
    }};
}

macro_rules! test_complex_type {
    ($ty:ty) => {{
        let zero = Complex::<$ty>::new(0 as $ty, 0 as $ty);
        test_assign(zero, Complex::<$ty>::new(0 as $ty, 0 as $ty));
        test_assign(zero, Complex::<$ty>::new(1 as $ty, 0 as $ty));
        test_assign(zero, Complex::<$ty>::new(2 as $ty, 0 as $ty));
        test_assign(zero, Complex::<$ty>::new((-1i32) as $ty, 0 as $ty));
        test_assign(zero, Complex::<$ty>::new((1u64 << 63) as $ty, 0 as $ty));
        test_assign(zero, Complex::<$ty>::new(1.5 as $ty, 0 as $ty));
        test_assign(zero, Complex::<$ty>::new(2.5 as $ty, 3.5 as $ty));
    }};
}

fn test_simple_assign() {
    // _Bool
    test_assign(false, false);
    test_assign(false, true);
    test_assign(false, true);
    test_assign(false, true);
    test_assign(false, true);
    test_assign(false, true);
    test_assign(false, true);

    test_int_type!(i8); // char
    test_int_type!(i8); // signed char
    test_int_type!(u8); // unsigned char
    test_int_type!(i16); // signed short
    test_int_type!(u16); // unsigned short
    test_int_type!(i32); // signed int
    test_int_type!(u32); // unsigned int
    test_int_type!(isize); // signed long
    test_int_type!(usize); // unsigned long
    test_int_type!(i64); // signed long long
    test_int_type!(u64); // unsigned long long
    test_float_type!(f32); // float
    test_float_type!(f64); // double
    test_float_type!(f64); // long double

    test_complex_type!(f32);
    test_complex_type!(f64);
    test_complex_type!(f64);

    let i: i32 = 0;
    test_assign(None::<&i32>, None::<&i32>);
    test_assign(None::<&i32>, Some(&i));

    let mut init = S { a: [0i16; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    let mut s1 = init;
    let mut s2 = S { a: [0i16; 1024] };

    let copy1 = { s1 = init; s1 };
    if init != copy1 {
        abort();
    }
    let copy2 = { s2 = s1; s2 };
    if init != copy2 {
        abort();
    }
    let copy3 = s1;
    if init != copy3 {
        abort();
    }
    let copy4 = s2;
    if init != copy4 {
        abort();
    }
}

fn main() {
    test_simple_assign();
    process::exit(0);
}