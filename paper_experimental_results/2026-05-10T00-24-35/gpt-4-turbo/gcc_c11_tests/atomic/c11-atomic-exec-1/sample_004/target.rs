use std::sync::atomic::{AtomicBool, AtomicU8, AtomicI8, AtomicU16, AtomicI16, AtomicU32, AtomicI32, AtomicU64, AtomicI64, AtomicUsize, AtomicIsize, Ordering};
use std::mem::MaybeUninit;

fn test_simple_assign() {
    // Test with various types and values
    macro_rules! test_simple_assign {
        ($type:ty, $value:expr) => {{
            let mut a = <$type>::default();
            let mut b = $value;

            assert_eq!(a, <$type>::default());
            assert_eq!(b, $value);

            a = b;
            assert_eq!(a, $value);
        }};
    }

    macro_rules! test_simple_assign_arith {
        ($value:expr) => {{
            test_simple_assign!(AtomicBool, $value != 0);
            test_simple_assign!(AtomicU8, $value as u8);
            test_simple_assign!(AtomicI8, $value as i8);
            test_simple_assign!(AtomicU16, $value as u16);
            test_simple_assign!(AtomicI16, $value as i16);
            test_simple_assign!(AtomicU32, $value as u32);
            test_simple_assign!(AtomicI32, $value as i32);
            test_simple_assign!(AtomicU64, $value as u64);
            test_simple_assign!(AtomicI64, $value as i64);
            test_simple_assign!(AtomicUsize, $value as usize);
            test_simple_assign!(AtomicIsize, $value as isize);
            test_simple_assign!(f32, $value as f32);
            test_simple_assign!(f64, $value as f64);
        }};
    }

    // Test with different numbers
    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!(1u64 << 63);
    test_simple_assign_arith!(1.5);
    test_simple_assign_arith!(2.5 + 3.5_f64);

    let mut i = 0;
    test_simple_assign!(&mut i, &mut i as *mut _);

    #[derive(Clone, Copy, Default)]
    struct S {
        a: [i16; 1024],
    }

    let mut init = S::default();
    let mut copy: S;
    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    let s1 = Atomic::new(init);
    let s2 = Atomic::new(init);

    copy = s1.load(Ordering::SeqCst);
    assert_eq!(init, copy);

    copy = s2.load(Ordering::SeqCst);
    assert_eq!(init, copy);

    copy = s1.load(Ordering::SeqCst);
    assert_eq!(init, copy);

    copy = s2.load(Ordering::SeqCst);
    assert_eq!(init, copy);
}

fn main() {
    test_simple_assign();
}