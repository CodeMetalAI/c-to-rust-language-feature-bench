use std::sync::atomic::{AtomicUsize, Ordering};

fn test_incdec<T>(value: T, preop: bool, postop: bool, change: T)
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + PartialEq,
{
    let a = std::sync::atomic::AtomicUsize::new(value as usize);
    let result = if preop {
        a.fetch_add(change as usize, Ordering::SeqCst) as T
    } else if postop {
        a.fetch_add(change as usize, Ordering::SeqCst) as T + change
    } else {
        a.fetch_sub(change as usize, Ordering::SeqCst) as T
    };
    if result!= value + change {
        panic!("assertion failed");
    }
    if a.load(Ordering::SeqCst) as T!= value + change {
        panic!("assertion failed");
    }
}

fn test_incdec_arith<T>(value: T)
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + PartialEq,
{
    test_incdec(value, true, false, 1);
    test_incdec(value, false, true, 1);
    test_incdec(value, false, false, -1);
    test_incdec(value, true, false, -1);
}

fn test_all_incdec_arith<T>(value: T)
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + PartialEq,
{
    test_incdec_arith(value);
}

fn test_incdec_ptr() {
    let mut ia = [0, 0];
    let a = &ia[1] as *const i32 as usize;
    test_incdec(a, true, false, 1);
    test_incdec(a, false, true, 1);
    test_incdec(a, false, false, -1);
    test_incdec(a, true, false, -1);
}

fn main() {
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith(1 << 60);
    test_all_incdec_arith(1.5);
    test_incdec_ptr();
}