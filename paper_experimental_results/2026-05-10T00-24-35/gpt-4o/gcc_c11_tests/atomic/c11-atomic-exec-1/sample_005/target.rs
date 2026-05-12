use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, 
                        AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize, Ordering};
use std::mem;

fn test_simple_assign() {
    macro_rules! test_simple_assign {
        ($atomic_type:ty, $value:expr) => {{
            let a = <$atomic_type>::new(Default::default());
            let b = <$atomic_type>::new($value);
            assert_eq!(a.load(Ordering::SeqCst), Default::default());
            assert_eq!(b.load(Ordering::SeqCst), $value);

            a.store(b.load(Ordering::SeqCst), Ordering::SeqCst);
            assert_eq!(a.load(Ordering::SeqCst), $value);
        }};
    }

    macro_rules! test_simple_assign_arith {
        ($value:expr) => {{
            test_simple_assign!(AtomicBool, $value != 0);
            test_simple_assign!(AtomicI8, $value);
            test_simple_assign!(AtomicU8, $value as u8);
            test_simple_assign!(AtomicI16, $value);
            test_simple_assign!(AtomicU16, $value as u16);
            test_simple_assign!(AtomicI32, $value);
            test_simple_assign!(AtomicU32, $value as u32);
            test_simple_assign!(AtomicI64, $value);
            test_simple_assign!(AtomicU64, $value as u64);
            test_simple_assign!(AtomicIsize, $value);
            test_simple_assign!(AtomicUsize, $value as usize);
        }};
    }

    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!(1u64 << 63);
    test_simple_assign_arith!(1.5f32.to_bits() as i32);
    test_simple_assign_arith!(f32::from_bits(0x40200000) as i32); // Equivalent to CMPLX(2.5, 3.5)

    #[derive(Clone, Copy, Default, PartialEq, Eq)]
    struct S {
        a: [i16; 1024],
    }

    let mut init = S::default();
    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    let mut s1 = init.clone();
    let mut s2 = s1.clone();
    let mut copy = s1.clone();

    assert_eq!(init, copy);

    copy = s2.clone();
    assert_eq!(init, copy);

    copy = s1.clone();
    assert_eq!(init, copy);

    copy = s2.clone();
    assert_eq!(init, copy);
}

fn main() {
    test_simple_assign();
    std::process::exit(0);
}