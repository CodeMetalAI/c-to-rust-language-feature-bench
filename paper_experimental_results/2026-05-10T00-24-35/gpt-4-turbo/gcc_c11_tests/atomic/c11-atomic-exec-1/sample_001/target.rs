use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicUsize, Ordering};
use std::cell::Cell;
use std::mem::size_of;

fn main() {
    test_simple_assign();
}

fn test_simple_assign() {
    macro_rules! test_simple_assign {
        ($type:ty, $value:expr) => {{
            let mut a = <$type>::default();
            let b = $value;
            assert_eq!(a, <$type>::default());
            assert_eq!(b, $value);
            a = b;
            assert_eq!(a, $value);
        }};
    }

    macro_rules! test_simple_assign_arith {
        ($value:expr) => {
            test_simple_assign!(AtomicBool, $value);
            test_simple_assign!(AtomicI8, $value as i8);
            test_simple_assign!(AtomicU8, $value as u8);
            test_simple_assign!(AtomicI16, $value as i16);
            test_simple_assign!(AtomicU16, $value as u16);
            test_simple_assign!(AtomicI32, $value as i32);
            test_simple_assign!(AtomicU32, $value as u32);
            test_simple_assign!(AtomicI64, $value as i64);
            test_simple_assign!(AtomicU64, $value as u64);
            test_simple_assign!(AtomicUsize, $value as usize);
        };
    }

    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!(1u64 << 63);
    test_simple_assign_arith!(1.5f32);
    test_simple_assign_arith!((2.5f32, 3.5f32));

    let i = 0;
    test_simple_assign!(AtomicPtr::<i32>, std::ptr::null_mut());
    test_simple_assign!(AtomicPtr::<i32>, &i as *const i32 as *mut i32);

    #[derive(Clone)]
    struct S {
        a: [i16; 1024],
    }

    let init = S { a: (0..1024).collect::<Vec<_>>().try_into().unwrap() };
    let mut s1 = Cell::new(init.clone());
    let mut s2 = Cell::new(S { a: [0; 1024] });

    let copy = s1.get();
    assert_eq!(copy.a, init.a);
    s2.set(s1.get());
    let copy = s2.get();
    assert_eq!(copy.a, init.a);
    let copy = s1.get();
    assert_eq!(copy.a, init.a);
    let copy = s2.get();
    assert_eq!(copy.a, init.a);
}