use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicUsize, Ordering};
use std::sync::atomic::AtomicPtr;
use std::mem::MaybeUninit;

fn main() {
    test_simple_assign();
}

fn test_simple_assign() {
    macro_rules! test_simple_assign {
        ($type:ty, $value:expr) => {{
            let mut a = <$type>::new(0);
            let b = <$type>::new($value);
            assert_eq!(a.load(Ordering::SeqCst), 0);
            assert_eq!(b.load(Ordering::SeqCst), $value);
            a.store(b.load(Ordering::SeqCst), Ordering::SeqCst);
            assert_eq!(a.load(Ordering::SeqCst), $value);
        }};
    }

    macro_rules! test_simple_assign_arith {
        ($value:expr) => {{
            test_simple_assign!(AtomicBool, $value != 0);
            test_simple_assign!(AtomicI8, $value as i8);
            test_simple_assign!(AtomicU8, $value as u8);
            test_simple_assign!(AtomicI16, $value as i16);
            test_simple_assign!(AtomicU16, $value as u16);
            test_simple_assign!(AtomicI32, $value as i32);
            test_simple_assign!(AtomicU32, $value as u32);
            test_simple_assign!(AtomicI64, $value as i64);
            test_simple_assign!(AtomicU64, $value as u64);
            test_simple_assign!(AtomicUsize, $value as usize);
        }};
    }

    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!(1u64 << 63);
    test_simple_assign_arith!(1.5f32);
    test_simple_assign_arith!(2.5f64 + 3.5f64);

    let mut i = 0;
    let ptr: *mut i32 = &mut i;
    let mut a = AtomicPtr::new(ptr);
    let b = AtomicPtr::new(ptr);
    assert_eq!(a.load(Ordering::SeqCst), ptr);
    assert_eq!(b.load(Ordering::SeqCst), ptr);
    a.store(b.load(Ordering::SeqCst), Ordering::SeqCst);
    assert_eq!(a.load(Ordering::SeqCst), ptr);

    #[derive(Clone, Copy)]
    struct S {
        a: [i16; 1024],
    }

    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    let mut s1 = MaybeUninit::<S>::new(init);
    let mut s2 = MaybeUninit::<S>::uninit();
    unsafe {
        s2.as_mut_ptr().write(s1.assume_init());
        assert_eq!((*s1.as_ptr()).a, (*s2.as_ptr()).a);
    }
}