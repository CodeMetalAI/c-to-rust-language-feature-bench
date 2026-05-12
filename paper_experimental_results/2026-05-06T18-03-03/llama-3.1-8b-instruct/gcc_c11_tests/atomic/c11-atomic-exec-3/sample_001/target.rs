use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, AtomicUsize, AtomicU64, Ordering};

fn test_incdec<T: std::sync::atomic::AtomicUsize + std::sync::atomic::AtomicIsize>(value: T, preop: bool, postop: bool, pre_p: bool, change: i32) {
    let a = T::new(value);
    let expected = if preop { value.get() + change } else { value.get() };
    if a.fetch_add(change, Ordering::SeqCst) != expected {
        panic!();
    }
    if a.load(Ordering::SeqCst) != expected {
        panic!();
    }
}

fn test_incdec_arith<T: std::sync::atomic::AtomicUsize + std::sync::atomic::AtomicIsize>(value: T, preop: bool, postop: bool, pre_p: bool, change: i32) {
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value, preop, postop, pre_p, change);
}

fn test_all_incdec_arith<T: std::sync::atomic::AtomicUsize + std::sync::atomic::AtomicIsize>(value: T) {
    test_incdec_arith(value, true, false, true, 1);
    test_incdec_arith(value, true, false, true, -1);
    test_incdec_arith(value, false, true, false, 1);
    test_incdec_arith(value, false, true, false, -1);
}

fn test_incdec_ptr<T: std::sync::atomic::AtomicUsize + std::sync::atomic::AtomicIsize>(value: *mut T, preop: bool, postop: bool, pre_p: bool, change: i32) {
    let a = AtomicUsize::new(value as usize);
    let expected = if preop { value as usize + change } else { value as usize };
    if a.fetch_add(change, Ordering::SeqCst) != expected {
        panic!();
    }
    if a.load(Ordering::SeqCst) != expected {
        panic!();
    }
}

fn test_incdec_ptr_arith<T: std::sync::atomic::AtomicUsize + std::sync::atomic::AtomicIsize>(value: *mut T, preop: bool, postop: bool, pre_p: bool, change: i32) {
    test_incdec_ptr(value, preop, postop, pre_p, change);
    test_incdec_ptr(value, preop, postop, pre_p, change);
    test_incdec_ptr(value, preop, postop, pre_p, change);
    test_incdec_ptr(value, preop, postop, pre_p, change);
    test_incdec_ptr(value, preop, postop, pre_p, change);
    test_incdec_ptr(value, preop, postop, pre_p, change);
    test_incdec_ptr(value, preop, postop, pre_p, change);
    test_incdec_ptr(value, preop, postop, pre_p, change);
    test_incdec_ptr(value, preop, postop, pre_p, change);
    test_incdec_ptr(value, preop, postop, pre_p, change);
    test_incdec_ptr(value, preop, postop, pre_p, change);
    test_incdec_ptr(value, preop, postop, pre_p, change);
}

fn test_incdec() {
    let ia: [AtomicUsize; 2] = [AtomicUsize::new(0), AtomicUsize::new(0)];
    test_all_incdec_arith(AtomicUsize::new(0));
    test_all_incdec_arith(AtomicUsize::new(1));
    test_all_incdec_arith(AtomicUsize::new(2));
    test_all_incdec_arith(AtomicUsize::new(-1));
    test_all_incdec_arith(AtomicUsize::new(1i64 << 60));
    test_all_incdec_arith(AtomicUsize::new(1.5 as f64));
    test_incdec_ptr(&ia[1], true, false, true, 1);
    test_incdec_ptr(&ia[1], true, false, true, -1);
    test_incdec_ptr(&ia[1], false, true, false, 1);
    test_incdec_ptr(&ia[1], false, true, false, -1);
}

fn main() {
    test_incdec();
    std::process::exit(0);
}