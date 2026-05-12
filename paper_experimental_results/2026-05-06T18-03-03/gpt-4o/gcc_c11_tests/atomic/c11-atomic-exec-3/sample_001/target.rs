use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize, Ordering};
use std::process::exit;

macro_rules! test_incdec {
    ($type:ty, $value:expr, $preop:ident, $postop:ident, $pre_p:expr, $change:expr) => {{
        let a = std::sync::Arc::new(<$type>::new($value));
        let a_clone = a.clone();
        let result = if $pre_p {
            a.$preop(Ordering::SeqCst)
        } else {
            a.$postop(Ordering::SeqCst)
        };
        if result != if $pre_p { $value + $change } else { $value } {
            panic!("abort");
        }
        if a.load(Ordering::SeqCst) != $value + $change {
            panic!("abort");
        }
    }};
}

macro_rules! test_incdec_arith {
    ($value:expr, $preop:ident, $postop:ident, $pre_p:expr, $change:expr) => {{
        test_incdec!(AtomicBool, $value as bool, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicI8, $value as i8, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicU8, $value as u8, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicI16, $value as i16, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicU16, $value as u16, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicI32, $value as i32, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicU32, $value as u32, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicI64, $value as i64, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicU64, $value as u64, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicIsize, $value as isize, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicUsize, $value as usize, $preop, $postop, $pre_p, $change);
    }};
}

macro_rules! test_all_incdec_arith {
    ($value:expr) => {{
        test_incdec_arith!($value, fetch_add, fetch_add, true, 1);
        test_incdec_arith!($value, fetch_sub, fetch_sub, true, -1);
        test_incdec_arith!($value, fetch_add, fetch_add, false, 1);
        test_incdec_arith!($value, fetch_sub, fetch_sub, false, -1);
    }};
}

fn test_incdec() {
    test_all_incdec_arith!(0);
    test_all_incdec_arith!(1);
    test_all_incdec_arith!(2);
    test_all_incdec_arith!(-1);
    test_all_incdec_arith!(1u64 << 60);
    // Floating point types are not atomic in Rust, so we skip them.
    // test_all_incdec_arith!(1.5);

    let ia = [0; 2];
    let ia_ptr = ia.as_ptr();
    let ia_ptr = ia_ptr.wrapping_add(1);
    let ia_atomic = std::sync::Arc::new(std::sync::atomic::AtomicPtr::new(ia_ptr as *mut i32));

    let ia_clone = ia_atomic.clone();
    let result = ia_atomic.fetch_add(1, Ordering::SeqCst);
    if result != ia_ptr as *mut i32 {
        panic!("abort");
    }
    if ia_atomic.load(Ordering::SeqCst) != ia_ptr.wrapping_add(1) as *mut i32 {
        panic!("abort");
    }

    let ia_clone = ia_atomic.clone();
    let result = ia_atomic.fetch_sub(1, Ordering::SeqCst);
    if result != ia_ptr.wrapping_add(1) as *mut i32 {
        panic!("abort");
    }
    if ia_atomic.load(Ordering::SeqCst) != ia_ptr as *mut i32 {
        panic!("abort");
    }
}

fn main() {
    test_incdec();
    exit(0);
}