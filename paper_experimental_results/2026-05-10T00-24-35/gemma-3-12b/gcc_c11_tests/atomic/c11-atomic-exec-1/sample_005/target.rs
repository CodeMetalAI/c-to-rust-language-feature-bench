use std::sync::atomic::{AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicI128, AtomicBool, AtomicF32, AtomicF64, AtomicPtr};
use std::mem;

#[cfg(target_pointer_width = "32")]
use std::ptr::NonNull as Ptr;

#[cfg(target_pointer_width = "64")]
use std::ptr::NonNull as Ptr;

#[derive(Copy, Clone, Debug, PartialEq)]
struct s {
    a: [i16; 1024],
}

fn test_simple_assign() {
    // TEST_SIMPLE_ASSIGN_ARITH(0);
    // TEST_SIMPLE_ASSIGN_ARITH(1);
    // TEST_SIMPLE_ASSIGN_ARITH(2);
    // TEST_SIMPLE_ASSIGN_ARITH(-1);
    // TEST_SIMPLE_ASSIGN_ARITH(1ULL << 63);
    // TEST_SIMPLE_ASSIGN_ARITH(1.5);
    // TEST_SIMPLE_ASSIGN_ARITH(CMPLX(2.5, 3.5));

    // Bool
    let mut a_bool = AtomicBool::new(false);
    let b_bool = AtomicBool::new(true);
    if a_bool.load(std::sync::atomic::Ordering::Relaxed) != false {
        panic!()
    }
    if b_bool.load(std::sync::atomic::Ordering::Relaxed) != true {
        panic!()
    }
    if a_bool.swap(b_bool.load(std::sync::atomic::Ordering::Relaxed), std::sync::atomic::Ordering::Relaxed) != true {
        panic!()
    }
    if a_bool.load(std::sync::atomic::Ordering::Relaxed) != true {
        panic!()
    }

    // Char
    let mut a_char = AtomicI8::new(0);
    let b_char = AtomicI8::new(1);
    if a_char.load(std::sync::atomic::Ordering::Relaxed) != 0 {
        panic!()
    }
    if b_char.load(std::sync::atomic::Ordering::Relaxed) != 1 {
        panic!()
    }
    if a_char.swap(b_char.load(std::sync::atomic::Ordering::Relaxed), std::sync::atomic::Ordering::Relaxed) != 1 {
        panic!()
    }
    if a_char.load(std::sync::atomic::Ordering::Relaxed) != 1 {
        panic!()
    }

     // Signed short
    let mut a_short = AtomicI16::new(0);
    let b_short = AtomicI16::new(1);
    if a_short.load(std::sync::atomic::Ordering::Relaxed) != 0 {
        panic!()
    }
    if b_short.load(std::sync::atomic::Ordering::Relaxed) != 1 {
        panic!()
    }
    if a_short.swap(b_short.load(std::sync::atomic::Ordering::Relaxed), std::sync::atomic::Ordering::Relaxed) != 1 {
        panic!()
    }
    if a_short.load(std::sync::atomic::Ordering::Relaxed) != 1 {
        panic!()
    }

    // Signed int
    let mut a_int = AtomicI32::new(0);
    let b_int = AtomicI32::new(1);
    if a_int.load(std::sync::atomic::Ordering::Relaxed) != 0 {
        panic!()
    }
    if b_int.load(std::sync::atomic::Ordering::Relaxed) != 1 {
        panic!()
    }
    if a_int.swap(b_int.load(std::sync::atomic::Ordering::Relaxed), std::sync::atomic::Ordering::Relaxed) != 1 {
        panic!()
    }
    if a_int.load(std::sync::atomic::Ordering::Relaxed) != 1 {
        panic!()
    }

    // Signed long
    let mut a_long = AtomicI64::new(0);
    let b_long = AtomicI64::new(1);
    if a_long.load(std::sync::atomic::Ordering::Relaxed) != 0 {
        panic!()
    }
    if b_long.load(std::sync::atomic::Ordering::Relaxed) != 1 {
        panic!()
    }
    if a_long.swap(b_long.load(std::sync::atomic::Ordering::Relaxed), std::sync::atomic::Ordering::Relaxed) != 1 {
        panic!()
    }
    if a_long.load(std::sync::atomic::Ordering::Relaxed) != 1 {
        panic!()
    }
    
    // float
    let mut a_f32 = AtomicF32::new(0.0);
    let b_f32 = AtomicF32::new(1.5);

    if a_f32.load(std::sync::atomic::Ordering::Relaxed) != 0.0 {
        panic!()
    }

    if b_f32.load(std::sync::atomic::Ordering::Relaxed) != 1.5 {
        panic!()
    }

    if a_f32.swap(b_f32.load(std::sync::atomic::Ordering::Relaxed), std::sync::atomic::Ordering::Relaxed) != 1.5 {
        panic!()
    }

    if a_f32.load(std::sync::atomic::Ordering::Relaxed) != 1.5 {
        panic!()
    }

    // double
    let mut a_f64 = AtomicF64::new(0.0);
    let b_f64 = AtomicF64::new(1.5);
    if a_f64.load(std::sync::atomic::Ordering::Relaxed) != 0.0 {
        panic!()
    }
    if b_f64.load(std::sync::atomic::Ordering::Relaxed) != 1.5 {
        panic!()
    }
    if a_f64.swap(b_f64.load(std::sync::atomic::Ordering::Relaxed), std::sync::atomic::Ordering::Relaxed) != 1.5 {
        panic!()
    }
    if a_f64.load(std::sync::atomic::Ordering::Relaxed) != 1.5 {
        panic!()
    }

    
    //Memory Comparison Tests

    let mut init = s { a: [0; 1024] };
    for i in 0..1024 {
        init.a[i] = i as i16;
    }
    let mut s1 = s { a: [0; 1024] };
    let mut s2 = s { a: [0; 1024] };

    s1 = init;
    s2 = s1;

    assert_eq!(init, s2);
    s2 = s1;
    assert_eq!(init, s2);

    s1 = init;
    s2 = s1;
    assert_eq!(init, s2);
}


fn main() {
    test_simple_assign();
    println!("Tests passed!");
}