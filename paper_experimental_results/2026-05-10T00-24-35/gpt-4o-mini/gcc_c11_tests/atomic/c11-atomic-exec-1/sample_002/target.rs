use std::sync::atomic::{AtomicUsize, Ordering};
use std::mem;
use std::process;

fn abort() {
    process::abort();
}

fn test_simple_assign() {
    fn test_simple_assign_arith<T: Copy + PartialEq + Default + std::fmt::Debug>(value: T) {
        let a = AtomicUsize::new(0);
        let b = AtomicUsize::new(value as usize);

        if a.load(Ordering::SeqCst) != 0 {
            abort();
        }
        if b.load(Ordering::SeqCst) != (value as usize) {
            abort();
        }
        a.store(b.load(Ordering::SeqCst), Ordering::SeqCst);
        if a.load(Ordering::SeqCst) != (value as usize) {
            abort();
        }
    }

    test_simple_assign_arith(0);
    test_simple_assign_arith(1);
    test_simple_assign_arith(2);
    test_simple_assign_arith(-1);
    test_simple_assign_arith(1u64 << 63);
    test_simple_assign_arith(1.5);
    test_simple_assign_arith((2.5f32, 3.5f32));
    
    let i = AtomicUsize::new(0);
    test_simple_assign_arith(0);
    test_simple_assign_arith(&i as *const _ as usize);
    
    struct S {
        a: [i16; 1024],
    }
    
    let mut init = S { a: [0; 1024] };
    let mut copy = S { a: [0; 1024] };
    let s1 = AtomicUsize::new(0);
    let s2 = AtomicUsize::new(0);

    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    copy = unsafe { std::ptr::read(&s1 as *const _ as *const S) };
    if mem::size_of::<S>() != mem::size_of::<S>() {
        abort();
    }
    
    copy = unsafe { std::ptr::read(&s2 as *const _ as *const S) };
    if mem::size_of::<S>() != mem::size_of::<S>() {
        abort();
    }
    
    copy = unsafe { std::ptr::read(&s1 as *const _ as *const S) };
    if mem::size_of::<S>() != mem::size_of::<S>() {
        abort();
    }
    
    copy = unsafe { std::ptr::read(&s2 as *const _ as *const S) };
    if mem::size_of::<S>() != mem::size_of::<S>() {
        abort();
    }
}

fn main() {
    test_simple_assign();
    process::exit(0);
}