use std::sync::atomic::{AtomicBool, AtomicChar, AtomicI8, AtomicU8, AtomicI16, AtomicU16,
                        AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicF32, AtomicF64,
                        AtomicI128, AtomicU128};

fn test_incdec<T: std::sync::atomic::AtomicUsize + std::sync::atomic::AtomicIsize>(
    value: T,
    preop: fn(T) -> T,
    postop: fn(T) -> T,
    pre_p: bool,
    change: isize,
) {
    let a = Atomic::new(value);
    if preop(a.load()) != postop(value) {
        panic!();
    }
    if a.load() != value + change {
        panic!();
    }
}

fn test_incdec_arith<T: std::sync::atomic::AtomicUsize + std::sync::atomic::AtomicIsize>(
    value: T,
    preop: fn(T) -> T,
    postop: fn(T) -> T,
    pre_p: bool,
    change: isize,
) {
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
    test_incdec(value, preop, postop, pre_p, change);
}

fn test_all_incdec_arith<T: std::sync::atomic::AtomicUsize + std::sync::atomic::AtomicIsize>(
    value: T,
) {
    test_incdec_arith(value, |x| x + 1, |x| x, true, 1);
    test_incdec_arith(value, |x| x - 1, |x| x, true, -1);
    test_incdec_arith(value, |x| x, |x| x + 1, false, 1);
    test_incdec_arith(value, |x| x, |x| x - 1, false, -1);
}

fn test_incdec() {
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith(1u64 << 60);
    test_all_incdec_arith(1.5f32);
    let ia = [0, 0];
    test_incdec(&ia[1], |x| &x + 1, |x| x, true, 1);
    test_incdec(&ia[1], |x| &x - 1, |x| x, true, -1);
    test_incdec(&ia[1], |x| x, |x| &x + 1, false, 1);
    test_incdec(&ia[1], |x| x, |x| &x - 1, false, -1);
}

fn main() {
    test_incdec();
    std::process::exit(0);
}