use std::process::exit;
use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize, Ordering};

fn test_incdec<T>(value: T, pre_op: fn(&T) -> T, post_op: fn(&T) -> T, pre_p: bool, change: T)
where
    T: Copy + PartialEq + std::fmt::Debug + std::ops::Add<Output = T> + From<i32>,
{
    let a = std::sync::atomic::Atomic::new(value);

    let res = if pre_p {
        let old_value = pre_op(&a.load(Ordering::SeqCst));
        post_op(&old_value)
    } else {
        let old_value = a.load(Ordering::SeqCst);
        post_op(&old_value)
    };

    if res != value + change {
        panic!("Assertion failed: {:?}", (res, value + change));
    }

    let final_value = a.load(Ordering::SeqCst);
    if final_value != value + change {
        panic!("Assertion failed: {:?}", (final_value, value + change));
    }
}

fn test_all_incdec_arith<T>(value: T)
where
    T: Copy + PartialEq + std::fmt::Debug + std::ops::Add<Output = T> + From<i32>,
{
    test_incdec(value, |x| x.fetch_add(1, Ordering::SeqCst) + 1.into(), |_| {}, true, 1.into());
    test_incdec(value, |x| x.fetch_sub(1, Ordering::SeqCst) - 1.into(), |_| {}, true, (-1).into());
    test_incdec(value, |_| {}, |x| x.fetch_add(1, Ordering::SeqCst) + 1.into(), false, 1.into());
    test_incdec(value, |_| {}, |x| x.fetch_sub(1, Ordering::SeqCst) - 1.into(), false, (-1).into());
}

fn test_incdec_arith<T>(value: T)
where
    T: Copy + PartialEq + std::fmt::Debug + std::ops::Add<Output = T> + From<i32>,
{
    test_all_incdec_arith(value);
}

fn main() {
    test_incdec_arith(0);
    test_incdec_arith(1);
    test_incdec_arith(2);
    test_incdec_arith(-1);
    test_incdec_arith(1u64 << 60);
    test_incdec_arith(1.5f64);

    let mut ia = [0; 2];
    let ptr = ia.as_mut_ptr();
    test_incdec(ptr, |x| unsafe { x.add(1) }, |x| unsafe { x.add(1) }, true, 1);
    test_incdec(ptr, |x| unsafe { x.sub(1) }, |x| unsafe { x.sub(1) }, true, -1);
    test_incdec(ptr, |_| {}, |x| unsafe { x.add(1) }, false, 1);
    test_incdec(ptr, |_| {}, |x| unsafe { x.sub(1) }, false, -1);

    exit(0);
}