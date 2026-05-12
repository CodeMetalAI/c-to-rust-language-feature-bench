use std::sync::atomic::{AtomicBool, AtomicU8, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicF32, AtomicF64, AtomicIsize, AtomicUsize, AtomicPtr};
use std::ptr;

#[inline(always)]
fn abort() {
    panic!("Assertion failed");
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
        static VOLATILE a: $type = 0;
        static mut b: $type = $value;
        unsafe {
            if a != 0 {
                abort();
            }
            if *b != $value {
                abort();
            }
            let old = a;
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
        //test_simple_assign!(AtomicPtr, $value); //Avoid raw pointer use
    };
}

fn test_simple_assign() {
    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!(1 << 63);
    test_simple_assign_arith!(1.5);
    test_simple_assign_arith!(0.0); // Complex numbers are not directly supported
    
    static mut i: i32 = 0;
    unsafe {
        test_simple_assign!(AtomicPtr, ptr::null());
        test_simple_assign!(AtomicPtr, &i);
    }
    
    #[derive(Debug, Clone, Copy)]
    #[repr(C)]
    pub struct s {
        a: [i16; 1024],
    }

    let mut init = s {
        a: [0; 1024],
    };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    
    static mut s1: AtomicS = AtomicS { a: [0; 1024] };
    static mut s2: AtomicS = AtomicS { a: [0; 1024] };
    
    unsafe {
        s1 = init;
        s2 = s1;
    }

    unsafe {
        let copy_init = s1.load();
        let copy_s2 = s2.load();
        if memcmp(&copy_init.a as *const _, &copy_s2.a as *const _, 1024 * std::mem::size_of::<i16>()) != 0 {
            abort();
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AtomicS {
    a: [i16; 1024],
}

impl AtomicS {
    fn load(&self) -> AtomicS {
        *self
    }
}

fn main() {
    test_simple_assign();
    exit(0);
}