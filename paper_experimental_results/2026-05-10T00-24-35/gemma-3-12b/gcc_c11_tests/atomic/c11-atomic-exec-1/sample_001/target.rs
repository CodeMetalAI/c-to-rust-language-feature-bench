use std::sync::atomic::{AtomicBool, AtomicU8, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicF32, AtomicF64, AtomicIsize, AtomicUsize, AtomicPtr};
use std::cmp::min;

#[inline(always)]
fn abort() {
    panic!("abort called");
}

#[inline(always)]
fn exit(code: i32) {
    std::process::exit(code);
}

#[inline(always)]
fn memcmp(a: *const u8, b: *const u8, size: usize) -> usize {
    unsafe {
        let mut i = 0;
        while i < size {
            if *a.add(i) != *b.add(i) {
                return i;
            }
            i += 1;
        }
        0
    }
}

macro_rules! test_simple_assign {
    ($type:ty, $value:expr) => {
        static mut a: $type = 0;
        static mut b: $type = $value;

        unsafe {
            if a != 0 {
                abort();
            }
            if b != $value {
                abort();
            }
            if (a = b) != $value {
                abort();
            }
            if a != $value {
                abort();
            }
        }
    };
}

macro_rules! test_simple_assign_arith {
    ($value:expr) => {
        unsafe {
            test_simple_assign!(AtomicBool, $value);
            test_simple_assign!(AtomicU8, $value);
            test_simple_assign!(AtomicI8, $value);
            test_simple_assign!(AtomicU16, $value);
            test_simple_assign!(AtomicI16, $value);
            test_simple_assign!(AtomicI32, $value);
            test_simple_assign!(AtomicU32, $value);
            test_simple_assign!(AtomicI64, $value);
            test_simple_assign!(AtomicU64, $value);
            test_simple_assign!(AtomicF32, $value);
            test_simple_assign!(AtomicF64, $value);
            test_simple_assign!(AtomicIsize, $value);
            test_simple_assign!(AtomicUsize, $value);
            // Complex numbers are not directly supported, skipping them
        }
    };
}

fn test_simple_assign() {
    unsafe {
        test_simple_assign_arith!(0);
        test_simple_assign_arith!(1);
        test_simple_assign_arith!(2);
        test_simple_assign_arith!(-1);
        test_simple_assign_arith!(1 << 63);
        test_simple_assign_arith!(1.5);
        //test_simple_assign_arith!(std::f64::consts::PI); // Complex number

        static mut i: i32 = 0;
        test_simple_assign!(AtomicPtr<i32>, 0 as *const i32);
        test_simple_assign!(AtomicPtr<i32>, &i);

        struct s {
            a: [i16; 1024],
        }
        static mut init: s = s { a: [0; 1024] };
        static mut copy: s = s { a: [0; 1024] };
        static mut s1: Atomic<s> = Atomic::new(s { a: [0; 1024] });
        static mut s2: Atomic<s> = Atomic::new(s { a: [0; 1024] });

        for j in 0..1024 {
            init.a[j] = j as i16;
        }

        s1.store(init, std::sync::atomic::Ordering::Relaxed);
        copy = s1.load(std::sync::atomic::Ordering::Relaxed);

        if memcmp(&init, &copy, std::mem::size_of::<s>()) != 0 {
            abort();
        }

        s2.store(s1.load(std::sync::atomic::Ordering::Relaxed), std::sync::atomic::Ordering::Relaxed);
        copy = s2.load(std::sync::atomic::Ordering::Relaxed);

        if memcmp(&init, &copy, std::mem::size_of::<s>()) != 0 {
            abort();
        }

        copy = s1.load(std::sync::atomic::Ordering::Relaxed);
        if memcmp(&init, &copy, std::mem::size_of::<s>()) != 0 {
            abort();
        }

        copy = s2.load(std::sync::atomic::Ordering::Relaxed);
        if memcmp(&init, &copy, std::mem::size_of::<s>()) != 0 {
            abort();
        }
    }
}

struct Atomic<T> {
    value: std::sync::atomic::AtomicPtr<T>,
}

impl<T> Atomic<T> {
    fn new(val: T) -> Self {
        Atomic {
            value: std::sync::atomic::AtomicPtr::new(&val as *const T),
        }
    }

    fn store(&self, val: T, ordering: std::sync::atomic::Ordering) {
        self.value.store(&val as *const T, ordering);
    }

    fn load(&self, ordering: std::sync::atomic::Ordering) -> T {
        unsafe { *self.value.load(ordering) }
    }
}

fn main() {
    test_simple_assign();
    exit(0);
}