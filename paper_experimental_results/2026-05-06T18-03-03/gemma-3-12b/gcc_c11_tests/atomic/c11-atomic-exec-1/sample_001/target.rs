use std::sync::atomic::{AtomicBool, AtomicU8, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicF32, AtomicF64, AtomicIsize, AtomicUsize, AtomicPtr};
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
        static VOLATILE a: $type = $type(0);
        static mut b: $type = $type($value);
        unsafe {
            if a != 0 {
                abort();
            }
            if *b != $value {
                abort();
            }
            if (a = *b) != $value {
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
    };
}

fn test_simple_assign() {
    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!(1 << 63);
    test_simple_assign_arith!(1.5);

    // Complex numbers are not directly supported by atomic types.
    // Skipping complex number tests.

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

    unsafe {
        for j in 0..1024 {
            init.a[j] = j as i16;
        }
    }

    unsafe {
        s1 = init;
        copy = s1;
        if memcmp(&init as *const s as *const u8, &copy as *const s as *const u8, std::mem::size_of::<s>()) != 0 {
            abort();
        }

        s2 = s1;
        copy = s2;
        if memcmp(&init as *const s as *const u8, &copy as *const s as *const u8, std::mem::size_of::<s>()) != 0 {
            abort();
        }

        copy = s1;
        if memcmp(&init as *const s as *const u8, &copy as *const s as *const u8, std::mem::size_of::<s>()) != 0 {
            abort();
        }

        copy = s2;
        if memcmp(&init as *const s as *const u8, &copy as *const s as *const u8, std::mem::size_of::<s>()) != 0 {
            abort();
        }
    }
}

fn main() {
    test_simple_assign();
    exit(0);
}