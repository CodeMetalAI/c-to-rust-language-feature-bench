use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicI64, AtomicU64, Ordering};
use std::mem::MaybeUninit;
use std::ptr;
use std::process::abort;

macro_rules! test_simple_assign {
    ($type:ty, $value:expr) => {
        {
            static A: Atomic<$type> = Atomic::new(0 as $type);
            static B: Atomic<$type> = Atomic::new($value as $type);
            if A.load(Ordering::SeqCst) != 0 {
                abort();
            }
            if B.load(Ordering::SeqCst) != $value as $type {
                abort();
            }
            A.store(B.load(Ordering::SeqCst), Ordering::SeqCst);
            if A.load(Ordering::SeqCst) != $value as $type {
                abort();
            }
        }
    };
}

macro_rules! test_simple_assign_arith {
    ($value:expr) => {
        {
            test_simple_assign!(bool, $value);
            test_simple_assign!(i8, $value);
            test_simple_assign!(u8, $value);
            test_simple_assign!(i16, $value);
            test_simple_assign!(u16, $value);
            test_simple_assign!(i32, $value);
            test_simple_assign!(u32, $value);
            test_simple_assign!(i64, $value);
            test_simple_assign!(u64, $value);
            test_simple_assign!(f32, $value);
            test_simple_assign!(f64, $value);
            // Complex numbers and long doubles are not natively supported in Rust, skipping
        }
    };
}

struct S {
    a: [i16; 1024],
}

fn test_simple_assign() {
    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!((1_u64 << 63) as i64);
    test_simple_assign_arith!(1.5);
    // Complex numbers are not natively supported in Rust, skipping CMPLX(2.5, 3.5);

    static mut I: MaybeUninit<i32> = MaybeUninit::uninit();
    unsafe {
        let ptr_null: *const i32 = ptr::null();
        let ptr_i: *const i32 = I.as_ptr();
        test_simple_assign!(usize, ptr_null as usize);
        test_simple_assign!(usize, ptr_i as usize);
    }

    let mut init = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    let mut copy = S { a: [0; 1024] };
    let s1 = init;
    copy = s1;
    if init.a != copy.a {
        abort();
    }
    let s2 = s1;
    copy = s2;
    if init.a != copy.a {
        abort();
    }
    copy = s1;
    if init.a != copy.a {
        abort();
    }
    copy = s2;
    if init.a != copy.a {
        abort();
    }
}

fn main() {
    test_simple_assign();
    std::process::exit(0);
}

// Wrapping atomic types for custom types
struct Atomic<T: Copy>(std::sync::atomic::AtomicUsize, std::marker::PhantomData<T>);

impl<T: Copy> Atomic<T> {
    fn new(val: T) -> Self {
        Atomic(std::sync::atomic::AtomicUsize::new(0), std::marker::PhantomData)
    }

    fn load(&self, ordering: Ordering) -> T {
        let ptr = self.0.load(ordering) as *const T;
        unsafe { *ptr }
    }

    fn store(&self, val: T, ordering: Ordering) {
        let ptr = &val as *const T as usize;
        self.0.store(ptr, ordering);
    }
}