use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};

fn cmplx(x: f64, y: f64) -> complex64 {
    complex64 { re: x, im: y }
}

fn test_simple_assign<T: Copy + Default + Eq>(value: T) {
    let a = Atomic::new(value);
    let b = Atomic::new(value);
    if a.load(Ordering::SeqCst) != value {
        panic!("a != {}", value);
    }
    if b.load(Ordering::SeqCst) != value {
        panic!("b != {}", value);
    }
    a.store(value, Ordering::SeqCst);
    if a.load(Ordering::SeqCst) != value {
        panic!("a != {}", value);
    }
}

fn test_simple_assign_arith() {
    test_simple_assign(0);
    test_simple_assign(1);
    test_simple_assign(2);
    test_simple_assign(-1);
    test_simple_assign(1isize << 63);
    test_simple_assign(1.5);
    test_simple_assign(cmplx(2.5, 3.5));
    let mut i: isize = 0;
    test_simple_assign(&mut i as *mut _);
    struct S {
        a: [isize; 1024],
    }
    let init = S { a: [0; 1024] };
    let s1 = Atomic::new(init);
    let s2 = Atomic::new(init);
    for j in 0..1024 {
        init.a[j] = j;
    }
    let copy = s1.load(Ordering::SeqCst);
    if init.a.iter().zip(copy.a.iter()).any(|(&a, &b)| a != b) {
        panic!("memcmp failed");
    }
    let copy = s2.load(Ordering::SeqCst);
    if init.a.iter().zip(copy.a.iter()).any(|(&a, &b)| a != b) {
        panic!("memcmp failed");
    }
    let copy = s1.load(Ordering::SeqCst);
    if init.a.iter().zip(copy.a.iter()).any(|(&a, &b)| a != b) {
        panic!("memcmp failed");
    }
    let copy = s2.load(Ordering::SeqCst);
    if init.a.iter().zip(copy.a.iter()).any(|(&a, &b)| a != b) {
        panic!("memcmp failed");
    }
}

fn main() {
    test_simple_assign_arith();
    std::process::exit(0);
}