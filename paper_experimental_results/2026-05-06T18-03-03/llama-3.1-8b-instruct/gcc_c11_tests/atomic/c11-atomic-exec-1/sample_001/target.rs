use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, AtomicF32, AtomicF64, AtomicU128, AtomicComplexF32, AtomicComplexF64, AtomicU128};
use std::cmp::Ordering;

fn test_simple_assign<T: Copy + Default + Eq + std::fmt::Debug>() {
    let a = Atomic::new(T::default());
    let b = Atomic::new(T::default());
    b.store(T::default());
    if a.load(Ordering::SeqCst) != T::default() {
        panic!("a != 0");
    }
    if b.load(Ordering::SeqCst) != T::default() {
        panic!("b != default");
    }
    let c = a.store(b.load(Ordering::SeqCst), Ordering::SeqCst);
    if c != T::default() {
        panic!("a != default");
    }
    if a.load(Ordering::SeqCst) != T::default() {
        panic!("a != default");
    }
}

fn test_simple_assign_arith<T: Copy + Default + Eq + std::fmt::Debug>() {
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
}

fn test_struct_assign() {
    let mut init: [i16; 1024] = [0; 1024];
    for i in 0..1024 {
        init[i] = i as i16;
    }
    let s1 = Atomic::new(init);
    let s2 = Atomic::new(init);
    let copy1 = s1.load(Ordering::SeqCst);
    let copy2 = s2.load(Ordering::SeqCst);
    if copy1 != init {
        panic!("copy1 != init");
    }
    if copy2 != init {
        panic!("copy2 != init");
    }
    let copy = s1.load(Ordering::SeqCst);
    if copy != init {
        panic!("copy != init");
    }
    let copy = s2.load(Ordering::SeqCst);
    if copy != init {
        panic!("copy != init");
    }
}

fn main() {
    test_simple_assign_arith::<i8>();
    test_simple_assign_arith::<i16>();
    test_simple_assign_arith::<i32>();
    test_simple_assign_arith::<i64>();
    test_simple_assign_arith::<isize>();
    test_simple_assign_arith::<u8>();
    test_simple_assign_arith::<u16>();
    test_simple_assign_arith::<u32>();
    test_simple_assign_arith::<u64>();
    test_simple_assign_arith::<usize>();
    test_simple_assign_arith::<f32>();
    test_simple_assign_arith::<f64>();
    test_simple_assign_arith::<u128>();
    test_simple_assign_arith::<complex_f32>();
    test_simple_assign_arith::<complex_f64>();
    test_struct_assign();
    std::process::exit(0);
}

// Complex types
type complex_f32 = (f32, f32);
type complex_f64 = (f64, f64);

// Atomic types
type AtomicBool = Atomic<bool>;
type AtomicI8 = Atomic<i8>;
type AtomicI16 = Atomic<i16>;
type AtomicI32 = Atomic<i32>;
type AtomicI64 = Atomic<i64>;
type AtomicIsize = Atomic<isize>;
type AtomicU8 = Atomic<u8>;
type AtomicU16 = Atomic<u16>;
type AtomicU32 = Atomic<u32>;
type AtomicU64 = Atomic<u64>;
type AtomicUsize = Atomic<usize>;
type AtomicF32 = Atomic<f32>;
type AtomicF64 = Atomic<f64>;
type AtomicU128 = Atomic<u128>;
type AtomicComplexF32 = Atomic<complex_f32>;
type AtomicComplexF64 = Atomic<complex_f64>;

// Atomic implementation
trait Atomic<T> {
    fn new(x: T) -> Self;
    fn load(&self, order: std::sync::atomic::Ordering) -> T;
    fn store(&self, x: T, order: std::sync::atomic::Ordering);
}

impl<T> Atomic<T> for Atomic<T> {
    fn new(x: T) -> Self {
        Atomic { value: x }
    }
    fn load(&self, _order: std::sync::atomic::Ordering) -> T {
        self.value
    }
    fn store(&self, x: T, _order: std::sync::atomic::Ordering) {
        self.value = x;
    }
}

struct Atomic<T> {
    value: T,
}