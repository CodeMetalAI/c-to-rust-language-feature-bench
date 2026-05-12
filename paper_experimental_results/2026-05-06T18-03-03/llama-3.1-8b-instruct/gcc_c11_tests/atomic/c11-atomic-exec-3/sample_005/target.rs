use std::sync::atomic::{AtomicBool, AtomicChar, AtomicI8, AtomicU8, AtomicI16, AtomicU16,
                        AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicF32, AtomicF64,
                        Ordering};

fn test_incdec<T: std::sync::atomic::AtomicUsize>(value: T, preop: bool, postop: bool,
                                                 pre_p: bool, change: i32) {
    let a = T::new(value.load(Ordering::Relaxed));
    let expected = if preop && postop {
        value.load(Ordering::Relaxed) + change
    } else if preop {
        value.load(Ordering::Relaxed) + change
    } else if postop {
        value.load(Ordering::Relaxed)
    } else {
        value.load(Ordering::Relaxed)
    };
    if a.load(Ordering::Relaxed) != expected {
        panic!();
    }
    if a.load(Ordering::Relaxed) != value.load(Ordering::Relaxed) + change {
        panic!();
    }
}

fn test_incdec_arith<T: std::sync::atomic::AtomicUsize>(value: T, preop: bool, postop: bool,
                                                     pre_p: bool, change: i32) {
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(AtomicBool::new(value.load(Ordering::Relaxed)), preop, postop, pre_p, change);
    test_incdec(AtomicChar::new(value.load(Ordering::Relaxed)), preop, postop, pre_p, change);
    test_incdec(AtomicI8::new(value.load(Ordering::Relaxed)), preop, postop, pre_p, change);
    test_incdec(AtomicU8::new(value.load(Ordering::Relaxed)), preop, postop, pre_p, change);
    test_incdec(AtomicI16::new(value.load(Ordering::Relaxed)), preop, postop, pre_p, change);
    test_incdec(AtomicU16::new(value.load(Ordering::Relaxed)), preop, postop, pre_p, change);
    test_incdec(AtomicI32::new(value.load(Ordering::Relaxed)), preop, postop, pre_p, change);
    test_incdec(AtomicU32::new(value.load(Ordering::Relaxed)), preop, postop, pre_p, change);
    test_incdec(AtomicI64::new(value.load(Ordering::Relaxed)), preop, postop, pre_p, change);
    test_incdec(AtomicU64::new(value.load(Ordering::Relaxed)), preop, postop, pre_p, change);
    test_incdec(AtomicF32::new(value.load(Ordering::Relaxed)), preop, postop, pre_p, change);
    test_incdec(AtomicF64::new(value.load(Ordering::Relaxed)), preop, postop, pre_p, change);
}

fn test_all_incdec_arith<T: std::sync::atomic::AtomicUsize>(value: T) {
    test_incdec_arith(value, true, false, true, 1);
    test_incdec_arith(value, true, false, true, -1);
    test_incdec_arith(value, false, true, false, 1);
    test_incdec_arith(value, false, true, false, -1);
}

fn test_incdec() {
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith(1u64 << 60);
    test_all_incdec_arith(1.5);
    let mut ia = [0, 0];
    test_incdec(&ia[1], true, false, true, 1);
    test_incdec(&ia[1], true, false, true, -1);
    test_incdec(&ia[1], false, true, false, 1);
    test_incdec(&ia[1], false, true, false, -1);
}

fn main() {
    test_incdec();
    std::process::exit(0);
}