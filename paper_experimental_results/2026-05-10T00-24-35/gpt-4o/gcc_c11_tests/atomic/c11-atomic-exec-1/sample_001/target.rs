use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize, Ordering};
use std::ptr;
use std::process::abort;

fn test_simple_assign_arith<T, F>(value: T, atomic_init: F)
where
    T: PartialEq + Copy,
    F: Fn(T) -> T,
{
    let a = atomic_init(T::default());
    let b = atomic_init(value);
    
    if a != T::default() {
        abort();
    }
    
    if b != value {
        abort();
    }
    
    let a = value;
    
    if a != value {
        abort();
    }
}

fn test_simple_assign() {
    test_simple_assign_arith(false, AtomicBool::new);
    test_simple_assign_arith(true, AtomicBool::new);
    test_simple_assign_arith(0i8, AtomicI8::new);
    test_simple_assign_arith(1i8, AtomicI8::new);
    test_simple_assign_arith(0u8, AtomicU8::new);
    test_simple_assign_arith(1u8, AtomicU8::new);
    test_simple_assign_arith(0i16, AtomicI16::new);
    test_simple_assign_arith(1i16, AtomicI16::new);
    test_simple_assign_arith(0u16, AtomicU16::new);
    test_simple_assign_arith(1u16, AtomicU16::new);
    test_simple_assign_arith(0i32, AtomicI32::new);
    test_simple_assign_arith(1i32, AtomicI32::new);
    test_simple_assign_arith(0u32, AtomicU32::new);
    test_simple_assign_arith(1u32, AtomicU32::new);
    test_simple_assign_arith(0i64, AtomicI64::new);
    test_simple_assign_arith(1i64, AtomicI64::new);
    test_simple_assign_arith(0u64, AtomicU64::new);
    test_simple_assign_arith(1u64, AtomicU64::new);
    
    let mut i = 0;
    let a = ptr::null_mut::<i32>();
    let b = &mut i as *mut i32;
    
    if a != ptr::null_mut() {
        abort();
    }
    
    if b != &mut i as *mut i32 {
        abort();
    }
    
    #[derive(Clone, Copy)]
    struct S {
        a: [i16; 1024],
    }
    
    let init = S { a: [0; 1024] };
    let mut s1: S = S { a: [0; 1024] };
    let mut s2: S = S { a: [0; 1024] };
    let mut copy: S;
    
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    
    s1 = init;
    copy = s1;
    
    if init.a != copy.a {
        abort();
    }
    
    s2 = s1;
    copy = s2;
    
    if init.a != copy.a {
        abort();
    }
}

fn main() {
    test_simple_assign();
    std::process::exit(0);
}