use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64, Ordering};
use std::mem::MaybeUninit;

macro_rules! test_simple_assign {
    ($type:ty, $value:expr) => {
        {
            static mut A: $type = <$type>::default();
            static B: $type = $value;
            unsafe {
                if A != <$type>::default() {
                    panic!("Test failed");
                }
                if B != $value {
                    panic!("Test failed");
                }
                A = B;
                if A != $value {
                    panic!("Test failed");
                }
            }
        }
    };
}

macro_rules! test_simple_assign_arith {
    ($value:expr) => {
        {
            test_simple_assign!(AtomicBool, AtomicBool::new($value != 0));
            test_simple_assign!(AtomicI8, AtomicI8::new($value as i8));
            test_simple_assign!(AtomicU8, AtomicU8::new($value as u8));
            test_simple_assign!(AtomicI16, AtomicI16::new($value as i16));
            test_simple_assign!(AtomicU16, AtomicU16::new($value as u16));
            test_simple_assign!(AtomicI32, AtomicI32::new($value as i32));
            test_simple_assign!(AtomicU32, AtomicU32::new($value as u32));
            test_simple_assign!(AtomicI64, AtomicI64::new($value as i64));
            test_simple_assign!(AtomicU64, AtomicU64::new($value as u64));
        }
    };
}

fn test_simple_assign() {
    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!(1u64 << 63);
    test_simple_assign_arith!(1.5_f64.to_bits());
    test_simple_assign_arith!(f64::from_bits(2.5_f64.to_bits() + 3.5_f64.to_bits()));

    static mut I: i32 = 0;
    test_simple_assign!(Option<*const i32>, None);
    test_simple_assign!(Option<*const i32>, Some(unsafe { &I }));

    #[repr(C)]
    struct S {
        a: [i16; 1024],
    }

    static mut INIT: S = S { a: [0; 1024] };
    static mut COPY: S = S { a: [0; 1024] };
    static mut S1: MaybeUninit<S> = MaybeUninit::uninit();
    static mut S2: MaybeUninit<S> = MaybeUninit::uninit();

    unsafe {
        for j in 0..1024 {
            INIT.a[j] = j as i16;
        }
        COPY = INIT;
        S1.write(INIT);
        if &INIT.a != &COPY.a {
            panic!("Test failed");
        }
        S2.write(S1.assume_init());
        if &INIT.a != &COPY.a {
            panic!("Test failed");
        }
        COPY = S1.assume_init();
        if &INIT.a != &COPY.a {
            panic!("Test failed");
        }
        COPY = S2.assume_init();
        if &INIT.a != &COPY.a {
            panic!("Test failed");
        }
    }
}

fn main() {
    test_simple_assign();
    std::process::exit(0);
}