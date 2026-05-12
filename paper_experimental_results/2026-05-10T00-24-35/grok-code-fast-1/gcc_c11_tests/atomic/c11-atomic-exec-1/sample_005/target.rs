use std::process::{abort, exit};
use std::cmp::Ordering;
use std::mem;

macro_rules! test_simple_assign {
    ($type:ty, $value:expr) => {{
        static mut A: $type = unsafe { mem::zeroed() };
        static mut B: $type = $value;
        unsafe {
            if A != mem::zeroed::<$type>() { abort(); }
            if B != $value { abort(); }
            if { A = B; A } != $value { abort(); }
            if A != $value { abort(); }
        }
    }};
}

macro_rules! test_simple_assign_arith {
    ($value:expr) => {{
        test_simple_assign!(bool, $value != 0);
        test_simple_assign!(i8, $value as i8);
        test_simple_assign!(u8, $value as u8);
        test_simple_assign!(i16, $value as i16);
        test_simple_assign!(u16, $value as u16);
        test_simple_assign!(i32, $value as i32);
        test_simple_assign!(u32, $value as u32);
        test_simple_assign!(i64, $value as i64);
        test_simple_assign!(u64, $value as u64);
        test_simple_assign!(isize, $value as isize);
        test_simple_assign!(usize, $value as usize);
        test_simple_assign!(f32, $value as f32);
        test_simple_assign!(f64, $value as f64);
        // For long double, approximate with f64
        test_simple_assign!(f64, $value as f64);
        // For complex, use arrays
        test_simple_assign!([f32; 2], [($value as f32).re as f32, ($value as f32).im as f32]);
        test_simple_assign!([f64; 2], [($value as f64).re as f64, ($value as f64).im as f64]);
        test_simple_assign!([f64; 2], [($value as f64).re as f64, ($value as f64).im as f64]);
    }};
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct S {
    a: [i16; 1024],
}

impl Default for S {
    fn default() -> Self {
        S { a: [0; 1024] }
    }
}

fn memcmp(a: &[u8], b: &[u8]) -> Ordering {
    a.cmp(b)
}

fn test_simple_assign() {
    test_simple_assign_arith!(0);
    test_simple_assign_arith!(1);
    test_simple_assign_arith!(2);
    test_simple_assign_arith!(-1);
    test_simple_assign_arith!(1u64 << 63);
    test_simple_assign_arith!(1.5);
    // For complex, rust doesn't have built-in complex, so use array
    test_simple_assign_arith!([2.5f32, 3.5f32]); // approximate

    static mut i: i32 = 0;
    test_simple_assign!(*const i32, std::ptr::null());
    test_simple_assign!(*const i32, unsafe { &i });

    let mut init = S::default();
    let mut copy = S::default();
    static mut s1: S = S { a: [0; 1024] };
    static mut s2: S = S { a: [0; 1024] };
    for j in 0..1024 {
        init.a[j] = j as i16;
    }
    unsafe {
        s1 = init;
        copy = s1;
        if memcmp(mem::transmute(&init), mem::transmute(&copy), mem::size_of::<S>()) != Ordering::Equal {
            abort();
        }
        s2 = s1;
        copy = s2;
        if memcmp(mem::transmute(&init), mem::transmute(&copy), mem::size_of::<S>()) != Ordering::Equal {
            abort();
        }
        copy = s1;
        if memcmp(mem::transmute(&init), mem::transmute(&copy), mem::size_of::<S>()) != Ordering::Equal {
            abort();
        }
        copy = s2;
        if memcmp(mem::transmute(&init), mem::transmute(&copy), mem::size_of::<S>()) != Ordering::Equal {
            abort();
        }
    }
}

fn main() {
    test_simple_assign();
    exit(0);
}