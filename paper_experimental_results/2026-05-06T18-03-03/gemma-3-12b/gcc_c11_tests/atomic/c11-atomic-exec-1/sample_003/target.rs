use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, AtomicF32, AtomicF64, AtomicPtr};
use std::ptr;

#[inline(always)]
fn abort() {
    panic!("abort called");
}

#[inline(always)]
fn exit(code: i32) {
    std::process::exit(code);
}

#[inline(always)]
fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    unsafe {
        let mut i = 0;
        while i < n {
            if *s1.add(i) != *s2.add(i) {
                return i as i32;
            }
            i += 1;
        }
        0
    }
}

macro_rules! test_simple_assign {
    ($type:ty, $value:expr) => {
        static VOLATILE a: $type = 0;
        static mut b: $type = $value;

        if a != 0 {
            abort();
        }
        unsafe {
            if *b != $value {
                abort();
            }
            let old = a.load(std::sync::atomic::Ordering::Relaxed);
            if (a.store($value, std::sync::atomic::Ordering::Relaxed)) != old {
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
        test_simple_assign!(AtomicBool, $value);
        test_simple_assign!(AtomicI8, $value);
        test_simple_assign!(AtomicI16, $value);
        test_simple_assign!(AtomicI32, $value);
        test_simple_assign!(AtomicI64, $value);
        test_simple_assign!(AtomicIsize, $value);
        test_simple_assign!(AtomicU8, $value);
        test_simple_assign!(AtomicU16, $value);
        test_simple_assign!(AtomicU32, $value);
        test_simple_assign!(AtomicU64, $value);
        test_simple_assign!(AtomicUsize, $value);
        test_simple_assign!(AtomicF32, $value);
        test_simple_assign!(AtomicF64, $value);
        test_simple_assign!(AtomicPtr<u8>, $value);
    };
}

fn test_simple_assign() {
    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!(1 << 63);
    test_simple_assign_arith!(1.5);
    // Complex numbers are not directly supported by Atomic types in Rust.
    // We skip this test.
    
    static mut i: i32 = 0;
    unsafe {
        test_simple_assign!(AtomicPtr<i32>, ptr::null());
        test_simple_assign!(AtomicPtr<i32>, &i);
    }

    #[derive(Debug, Clone, Copy)]
    struct s {
        a: [i16; 1024],
    }

    let mut init = s { a: [0; 1024] };
    let mut copy = s { a: [0; 1024] };
    let mut s1 = s { a: [0; 1024] };
    let mut s2 = s { a: [0; 1024] };

    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    unsafe {
        s1 = s { a: [0; 1024] };
        s1.a = init.a;
        copy = s1;
        if memcmp(&init.a as *const _, &copy.a as *const _, 1024 * std::mem::size_of::<i16>()) != 0 {
            abort();
        }

        s2 = s { a: [0; 1024] };
        s2.a = s1.a;
        copy = s2;
        if memcmp(&init.a as *const _, &copy.a as *const _, 1024 * std::mem::size_of::<i16>()) != 0 {
            abort();
        }

        copy = s1;
        if memcmp(&init.a as *const _, &copy.a as *const _, 1024 * std::mem::size_of::<i16>()) != 0 {
            abort();
        }

        copy = s2;
        if memcmp(&init.a as *const _, &copy.a as *const _, 1024 * std::mem::size_of::<i16>()) != 0 {
            abort();
        }
    }
}

fn main() {
    test_simple_assign();
    exit(0);
}