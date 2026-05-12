use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicUsize, Ordering};
use std::sync::atomic::AtomicPtr;
use std::mem::size_of;
use std::process::exit;
use std::ptr;

fn main() {
    test_simple_assign();
    exit(0);
}

fn test_simple_assign() {
    macro_rules! test_simple_assign {
        ($type:ty, $value:expr) => {{
            let mut a = <$type>::default();
            let b = $value;
            assert_eq!(a.load(Ordering::SeqCst), <$type>::default().load(Ordering::SeqCst));
            assert_eq!(b.load(Ordering::SeqCst), $value);
            a.store(b.load(Ordering::SeqCst), Ordering::SeqCst);
            assert_eq!(a.load(Ordering::SeqCst), $value);
        }};
    }

    macro_rules! test_simple_assign_arith {
        ($value:expr) => {{
            test_simple_assign!(AtomicBool, AtomicBool::new($value));
            test_simple_assign!(AtomicI8, AtomicI8::new($value as i8));
            test_simple_assign!(AtomicU8, AtomicU8::new($value as u8));
            test_simple_assign!(AtomicI16, AtomicI16::new($value as i16));
            test_simple_assign!(AtomicU16, AtomicU16::new($value as u16));
            test_simple_assign!(AtomicI32, AtomicI32::new($value as i32));
            test_simple_assign!(AtomicU32, AtomicU32::new($value as u32));
            test_simple_assign!(AtomicI64, AtomicI64::new($value as i64));
            test_simple_assign!(AtomicU64, AtomicU64::new($value as u64));
            test_simple_assign!(AtomicUsize, AtomicUsize::new($value as usize));
            test_simple_assign!(AtomicPtr::<i32>, AtomicPtr::new(ptr::null_mut()));
            let mut i = 0;
            test_simple_assign!(AtomicPtr::<i32>, AtomicPtr::new(&mut i as *mut i32));
        }};
    }

    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!(1u64 << 63);
    test_simple_assign_arith!(1.5f64);

    struct S {
        a: [i16; 1024],
    }

    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    let mut copy = init;
    assert_eq!(init, copy);
    copy = init;
    assert_eq!(init, copy);
    copy = init;
    assert_eq!(init, copy);
    copy = init;
    assert_eq!(init, copy);
}