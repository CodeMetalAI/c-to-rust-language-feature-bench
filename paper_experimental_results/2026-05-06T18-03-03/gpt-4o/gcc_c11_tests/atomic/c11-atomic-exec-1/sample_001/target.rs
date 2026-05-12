use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize, Ordering};
use std::mem::MaybeUninit;

fn abort() {
    std::process::abort();
}

fn test_simple_assign() {
    macro_rules! test_simple_assign {
        ($atomic_type:ty, $value:expr) => {
            {
                let a: $atomic_type = <$atomic_type>::new(Default::default());
                let b: $atomic_type = <$atomic_type>::new($value);
                if a.load(Ordering::SeqCst) != Default::default() {
                    abort();
                }
                if b.load(Ordering::SeqCst) != $value {
                    abort();
                }
                a.store(b.load(Ordering::SeqCst), Ordering::SeqCst);
                if a.load(Ordering::SeqCst) != $value {
                    abort();
                }
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

    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!(1u64 << 63);
    test_simple_assign_arith!(1.5_f64 as i64);
    test_simple_assign_arith!(2.5_f64 as i64); // Complex numbers are not directly supported

    static mut I: i32 = 0;
    let i_ptr: *mut i32 = unsafe { &mut I };
    let null_ptr: *mut i32 = std::ptr::null_mut();
    test_simple_assign!(AtomicUsize, null_ptr as usize);
    test_simple_assign!(AtomicUsize, i_ptr as usize);

    #[repr(C)]
    struct S {
        a: [i16; 1024],
    }

    let mut init = S { a: [0; 1024] };
    let mut copy = S { a: [0; 1024] };
    let mut s1 = MaybeUninit::<S>::uninit();
    let mut s2 = MaybeUninit::<S>::uninit();

    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    unsafe {
        s1.as_mut_ptr().write(init);
        copy = s1.assume_init();
        if init.a != copy.a {
            abort();
        }

        s2.as_mut_ptr().write(s1.assume_init());
        copy = s2.assume_init();
        if init.a != copy.a {
            abort();
        }

        copy = s1.assume_init();
        if init.a != copy.a {
            abort();
        }

        copy = s2.assume_init();
        if init.a != copy.a {
            abort();
        }
    }
}

fn main() {
    test_simple_assign();
    std::process::exit(0);
}