use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize, Ordering};
use std::mem;
use std::ptr;

fn test_simple_assign() {
    fn test_simple_assign_arith<T: PartialEq + Copy + Default>(value: T) {
        let mut a: T = Default::default();
        let b: T = value;
        if a != Default::default() {
            panic!("abort");
        }
        if b != value {
            panic!("abort");
        }
        a = b;
        if a != value {
            panic!("abort");
        }
    }

    test_simple_assign_arith::<bool>(false);
    test_simple_assign_arith::<bool>(true);
    test_simple_assign_arith::<i8>(0);
    test_simple_assign_arith::<i8>(1);
    test_simple_assign_arith::<i8>(2);
    test_simple_assign_arith::<i8>(-1);
    test_simple_assign_arith::<u8>(0);
    test_simple_assign_arith::<u8>(1);
    test_simple_assign_arith::<u8>(2);
    test_simple_assign_arith::<i16>(0);
    test_simple_assign_arith::<i16>(1);
    test_simple_assign_arith::<i16>(2);
    test_simple_assign_arith::<i16>(-1);
    test_simple_assign_arith::<u16>(0);
    test_simple_assign_arith::<u16>(1);
    test_simple_assign_arith::<u16>(2);
    test_simple_assign_arith::<i32>(0);
    test_simple_assign_arith::<i32>(1);
    test_simple_assign_arith::<i32>(2);
    test_simple_assign_arith::<i32>(-1);
    test_simple_assign_arith::<u32>(0);
    test_simple_assign_arith::<u32>(1);
    test_simple_assign_arith::<u32>(2);
    test_simple_assign_arith::<i64>(0);
    test_simple_assign_arith::<i64>(1);
    test_simple_assign_arith::<i64>(2);
    test_simple_assign_arith::<i64>(-1);
    test_simple_assign_arith::<u64>(0);
    test_simple_assign_arith::<u64>(1);
    test_simple_assign_arith::<u64>(2);
    test_simple_assign_arith::<u64>(1 << 63);
    test_simple_assign_arith::<f32>(0.0);
    test_simple_assign_arith::<f32>(1.0);
    test_simple_assign_arith::<f32>(2.0);
    test_simple_assign_arith::<f32>(-1.0);
    test_simple_assign_arith::<f32>(1.5);
    test_simple_assign_arith::<f64>(0.0);
    test_simple_assign_arith::<f64>(1.0);
    test_simple_assign_arith::<f64>(2.0);
    test_simple_assign_arith::<f64>(-1.0);
    test_simple_assign_arith::<f64>(1.5);

    let mut i: i32 = 0;
    test_simple_assign_arith::<*const i32>(ptr::null());
    test_simple_assign_arith::<*const i32>(&i);

    #[repr(C)]
    struct S {
        a: [i16; 1024],
    }

    let mut init = S { a: [0; 1024] };
    let mut copy = S { a: [0; 1024] };
    let mut s1 = S { a: [0; 1024] };
    let mut s2 = S { a: [0; 1024] };

    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    s1 = init;
    copy = s1;
    if init.a != copy.a {
        panic!("abort");
    }

    s2 = s1;
    copy = s2;
    if init.a != copy.a {
        panic!("abort");
    }

    copy = s1;
    if init.a != copy.a {
        panic!("abort");
    }

    copy = s2;
    if init.a != copy.a {
        panic!("abort");
    }
}

fn main() {
    test_simple_assign();
    std::process::exit(0);
}