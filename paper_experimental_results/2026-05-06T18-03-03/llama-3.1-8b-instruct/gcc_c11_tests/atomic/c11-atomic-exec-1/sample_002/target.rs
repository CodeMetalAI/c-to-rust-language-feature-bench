use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::cmp::Ordering;

fn test_simple_assign_arith<T: Copy + Default + Eq + std::fmt::Debug>(value: T) {
    let a = Atomic::new(value);
    let b = Atomic::new(value);
    if a.load(Ordering::SeqCst)!= value {
        panic!();
    }
    if b.load(Ordering::SeqCst)!= value {
        panic!();
    }
    a.store(value, Ordering::SeqCst);
    if a.load(Ordering::SeqCst)!= value {
        panic!();
    }
    b.store(value, Ordering::SeqCst);
    if b.load(Ordering::SeqCst)!= value {
        panic!();
    }
}

fn test_simple_assign() {
    test_simple_assign_arith(0);
    test_simple_assign_arith(1);
    test_simple_assign_arith(2);
    test_simple_assign_arith(-1);
    test_simple_assign_arith(1u64 << 63);
    test_simple_assign_arith(1.5);
    test_simple_assign_arith(CMPLX(2.5, 3.5));
    let i: i32 = 0;
    test_simple_assign_arith(&i as *const i32);
    let s = [0; 1024];
    let init = s.clone();
    let s1 = Atomic::new(s);
    let s2 = Atomic::new(s);
    for j in 0..1024 {
        s[j] = j as i16;
    }
    let copy = s1.load(Ordering::SeqCst);
    if init!= copy {
        panic!();
    }
    let copy = s2.load(Ordering::SeqCst);
    if init!= copy {
        panic!();
    }
    let copy = s1.load(Ordering::SeqCst);
    if init!= copy {
        panic!();
    }
    let copy = s2.load(Ordering::SeqCst);
    if init!= copy {
        panic!();
    }
}

fn CMPLX(x: f64, y: f64) -> std::complex::Complex<f64> {
    std::complex::Complex::new(x, y)
}

fn main() {
    test_simple_assign();
    std::process::exit(0);
}