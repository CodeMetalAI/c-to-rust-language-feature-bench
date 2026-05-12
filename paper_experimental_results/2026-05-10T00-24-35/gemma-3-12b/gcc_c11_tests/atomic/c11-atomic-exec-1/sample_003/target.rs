use std::sync::atomic::{AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicI128, AtomicBool, AtomicF32, AtomicF64, AtomicPtr};
use std::ptr;

#[cfg(target_pointer_width = "32")]
use std::sync::atomic::AtomicU8;
#[cfg(target_pointer_width = "32")]
use std::sync::atomic::AtomicU16;
#[cfg(target_pointer_width = "32")]
use std::sync::atomic::AtomicU32;

#[cfg(target_pointer_width = "64")]
use std::sync::atomic::AtomicU8;
#[cfg(target_pointer_width = "64")]
use std::sync::atomic::AtomicU16;
#[cfg(target_pointer_width = "64")]
use std::sync::atomic::AtomicU32;
#[cfg(target_pointer_width = "64")]
use std::sync::atomic::AtomicU64;
#[cfg(target_pointer_width = "64")]
use std::sync::atomic::AtomicU128;

fn abort() {
    panic!("abort called");
}

fn exit(code: i32) {
    std::process::exit(code);
}

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
        static VOLATILE a: $type = $type(0);
        static mut b: $type = $type($value);

        if a != 0 {
            abort();
        }
        unsafe {
            if *b != $value {
                abort();
            }
            let old = a;
            a = *b;
            if old != a {
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
        #[cfg(target_pointer_width = "32")]
        test_simple_assign!(AtomicU8, $value);
        #[cfg(target_pointer_width = "32")]
        test_simple_assign!(AtomicU16, $value);
        #[cfg(target_pointer_width = "32")]
        test_simple_assign!(AtomicU32, $value);

        #[cfg(target_pointer_width = "64")]
        test_simple_assign!(AtomicU8, $value);
        #[cfg(target_pointer_width = "64")]
        test_simple_assign!(AtomicU16, $value);
        #[cfg(target_pointer_width = "64")]
        test_simple_assign!(AtomicU32, $value);
        #[cfg(target_pointer_width = "64")]
        test_simple_assign!(AtomicI64, $value);
        #[cfg(target_pointer_width = "64")]
        test_simple_assign!(AtomicI128, $value);
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
    // Complex numbers are not directly supported by Atomic types, so skipping them

    static mut i: i32 = 0;
    test_simple_assign!(AtomicPtr<i32>, 0);
    test_simple_assign!(AtomicPtr<i32>, &i);

    struct S {
        a: [i16; 1024],
    }
    let mut init = S { a: [0; 1024] };
    let mut copy = S { a: [0; 1024] };
    static mut s1: AtomicS = AtomicS { a: [0; 1024] };
    static mut s2: AtomicS = AtomicS { a: [0; 1024] };
    unsafe {
        for j in 0..1024 {
            init.a[j] = j as i16;
        }
        s1 = init;
        copy = s1.load();
        if memcmp(&init.a, &copy.a, std::mem::size_of::<[i16; 1024]>()) != 0 {
            abort();
        }
        s2 = s1;
        copy = s2.load();
        if memcmp(&init.a, &copy.a, std::mem::size_of::<[i16; 1024]>()) != 0 {
            abort();
        }
        copy = s1.load();
        if memcmp(&init.a, &copy.a, std::mem::size_of::<[i16; 1024]>()) != 0 {
            abort();
        }
        copy = s2.load();
        if memcmp(&init.a, &copy.a, std::mem::size_of::<[i16; 1024]>()) != 0 {
            abort();
        }
    }
}

struct AtomicS {
    a: [i16; 1024],
}

impl AtomicS {
    fn load(&self) -> S {
        S { a: self.a }
    }
}

fn main() {
    test_simple_assign();
    exit(0);
}