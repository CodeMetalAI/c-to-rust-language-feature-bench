use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize, Ordering};
use std::mem::MaybeUninit;

fn abort() {
    std::process::abort();
}

fn test_simple_assign() {
    macro_rules! test_simple_assign {
        ($atomic_type:ty, $value:expr) => {
            {
                static mut A: $atomic_type = <$atomic_type>::new(0);
                static B: $atomic_type = <$atomic_type>::new($value);
                unsafe {
                    if A.load(Ordering::SeqCst) != 0 {
                        abort();
                    }
                    if B.load(Ordering::SeqCst) != $value {
                        abort();
                    }
                    A.store(B.load(Ordering::SeqCst), Ordering::SeqCst);
                    if A.load(Ordering::SeqCst) != $value {
                        abort();
                    }
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
    // Floating point and complex numbers are not directly supported in atomic operations in Rust.

    static mut I: MaybeUninit<i32> = MaybeUninit::uninit();
    unsafe {
        test_simple_assign!(AtomicUsize, 0);
        test_simple_assign!(AtomicUsize, I.as_ptr() as usize);
    }

    #[repr(C)]
    struct S {
        a: [i16; 1024],
    }

    static mut INIT: S = S { a: [0; 1024] };
    static mut COPY: S = S { a: [0; 1024] };
    static mut S1: S = S { a: [0; 1024] };
    static mut S2: S = S { a: [0; 1024] };

    unsafe {
        for j in 0..1024 {
            INIT.a[j] = j as i16;
        }
        COPY = INIT;
        S1 = INIT;
        if INIT.a != COPY.a {
            abort();
        }
        S2 = S1;
        if INIT.a != S2.a {
            abort();
        }
        COPY = S1;
        if INIT.a != COPY.a {
            abort();
        }
        COPY = S2;
        if INIT.a != COPY.a {
            abort();
        }
    }
}

fn main() {
    test_simple_assign();
    std::process::exit(0);
}