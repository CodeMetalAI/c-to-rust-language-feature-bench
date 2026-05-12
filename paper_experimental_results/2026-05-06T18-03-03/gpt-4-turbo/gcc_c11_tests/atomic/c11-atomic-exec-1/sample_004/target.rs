use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicUsize, Ordering};
use std::sync::atomic::AtomicPtr;
use std::mem::MaybeUninit;
use std::ptr;

fn test_simple_assign<T: Copy + PartialEq + std::fmt::Debug>(value: T, zero_value: T) {
    let mut a = MaybeUninit::<T>::uninit();
    let mut b = MaybeUninit::new(value);

    unsafe {
        let a = a.as_mut_ptr();
        let b = b.as_ptr();

        if ptr::read(a) != zero_value {
            panic!("{:?} != {:?}", ptr::read(a), zero_value);
        }
        if ptr::read(b) != value {
            panic!("{:?} != {:?}", ptr::read(b), value);
        }
        ptr::write(a, ptr::read(b));
        if ptr::read(a) != value {
            panic!("{:?} != {:?}", ptr::read(a), value);
        }
        if ptr::read(a) != value {
            panic!("{:?} != {:?}", ptr::read(a), value);
        }
    }
}

fn test_simple_assign_arith<T: Copy + PartialEq + std::fmt::Debug>(value: T, zero_value: T) {
    test_simple_assign(value, zero_value);
}

fn main() {
    let zero_bool = false;
    let zero_i8 = 0i8;
    let zero_u8 = 0u8;
    let zero_i16 = 0i16;
    let zero_u16 = 0u16;
    let zero_i32 = 0i32;
    let zero_u32 = 0u32;
    let zero_i64 = 0i64;
    let zero_u64 = 0u64;
    let zero_f32 = 0.0f32;
    let zero_f64 = 0.0f64;

    let values = [0, 1, 2, -1, 1 << 63, 1.5];

    for &value in &values {
        test_simple_assign_arith(value, zero_bool);
        test_simple_assign_arith(value as i8, zero_i8);
        test_simple_assign_arith(value as u8, zero_u8);
        test_simple_assign_arith(value as i16, zero_i16);
        test_simple_assign_arith(value as u16, zero_u16);
        test_simple_assign_arith(value as i32, zero_i32);
        test_simple_assign_arith(value as u32, zero_u32);
        test_simple_assign_arith(value as i64, zero_i64);
        test_simple_assign_arith(value as u64, zero_u64);
        test_simple_assign_arith(value as f32, zero_f32);
        test_simple_assign_arith(value as f64, zero_f64);
    }

    let mut i = 0;
    test_simple_assign(&mut i as *mut i32, std::ptr::null_mut());

    #[derive(Clone, Debug, PartialEq)]
    struct S {
        a: [i16; 1024],
    }

    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    let mut s1 = init.clone();
    let mut s2 = s1.clone();

    assert_eq!(init, s1);
    assert_eq!(init, s2);
}