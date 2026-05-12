use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize, Ordering};
use std::mem;

macro_rules! test_simple_assign {
    ($type:ty, $value:expr) => {
        {
            let a = <$type>::default();
            let b: $type = $value;
            assert_eq!(a.load(Ordering::SeqCst), 0 as $type);
            assert_eq!(b, $value);
            a.store(b, Ordering::SeqCst);
            assert_eq!(a.load(Ordering::SeqCst), $value);
        }
    };
}

macro_rules! test_simple_assign_arith {
    ($value:expr) => {
        {
            test_simple_assign!(AtomicBool, $value != 0);
            test_simple_assign!(AtomicI8, $value as i8);
            test_simple_assign!(AtomicU8, $value as u8);
            test_simple_assign!(AtomicI16, $value as i16);
            test_simple_assign!(AtomicU16, $value as u16);
            test_simple_assign!(AtomicI32, $value as i32);
            test_simple_assign!(AtomicU32, $value as u32);
            test_simple_assign!(AtomicI64, $value as i64);
            test_simple_assign!(AtomicU64, $value as u64);
            test_simple_assign!(AtomicIsize, $value as isize);
            test_simple_assign!(AtomicUsize, $value as usize);
        }
    };
}

fn test_simple_assign() {
    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!(1u64 << 63);
    test_simple_assign_arith!(1.5_f32 as i32);

    #[repr(C)]
    struct S {
        a: [u16; 1024],
    }

    let mut init = S { a: [0; 1024] };
    let mut copy = S { a: [0; 1024] };

    for j in 0..1024 {
        init.a[j] = j as u16;
    }

    let s1 = init.clone();
    copy = s1.clone();
    assert_eq!(memcmp(&init, &copy), 0);

    let s2 = s1.clone();
    copy = s2.clone();
    assert_eq!(memcmp(&init, &copy), 0);

    copy = s1.clone();
    assert_eq!(memcmp(&init, &copy), 0);

    copy = s2.clone();
    assert_eq!(memcmp(&init, &copy), 0);
}

fn memcmp<T: PartialEq>(a: &T, b: &T) -> isize {
    if a == b { 0 } else { 1 }
}

fn main() {
    test_simple_assign();
}