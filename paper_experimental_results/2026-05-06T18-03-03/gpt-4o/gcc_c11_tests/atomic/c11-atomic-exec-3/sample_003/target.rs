use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize, Ordering};
use std::process::exit;

macro_rules! test_incdec {
    ($type:ty, $value:expr, $preop:ident, $postop:ident, $pre_p:expr, $change:expr) => {{
        let a = std::sync::Arc::new(std::sync::atomic::AtomicPtr::new(Box::into_raw(Box::new($value as $type))));
        let a_clone = a.clone();
        let mut a_ref = unsafe { &mut *a_clone.load(Ordering::SeqCst) };
        let pre_value = if $pre_p {
            $preop(a_ref, $change)
        } else {
            *a_ref
        };
        if pre_value != if $pre_p { *a_ref } else { $postop(a_ref, $change) } {
            panic!("Test failed");
        }
        if *a_ref != ($value as $type + $change) {
            panic!("Test failed");
        }
    }};
}

macro_rules! test_incdec_arith {
    ($value:expr, $preop:ident, $postop:ident, $pre_p:expr, $change:expr) => {{
        test_incdec!(bool, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(i8, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(u8, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(i16, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(u16, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(i32, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(u32, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(i64, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(u64, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(f32, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(f64, $value, $preop, $postop, $pre_p, $change);
    }};
}

macro_rules! test_all_incdec_arith {
    ($value:expr) => {{
        test_incdec_arith!($value, pre_increment, post_increment, true, 1);
        test_incdec_arith!($value, pre_decrement, post_decrement, true, -1);
        test_incdec_arith!($value, pre_increment, post_increment, false, 1);
        test_incdec_arith!($value, pre_decrement, post_decrement, false, -1);
    }};
}

fn pre_increment<T: std::ops::AddAssign + Copy>(value: &mut T, change: T) -> T {
    *value += change;
    *value
}

fn post_increment<T: std::ops::AddAssign + Copy>(value: &mut T, change: T) -> T {
    let old_value = *value;
    *value += change;
    old_value
}

fn pre_decrement<T: std::ops::SubAssign + Copy>(value: &mut T, change: T) -> T {
    *value -= change;
    *value
}

fn post_decrement<T: std::ops::SubAssign + Copy>(value: &mut T, change: T) -> T {
    let old_value = *value;
    *value -= change;
    old_value
}

fn test_incdec() {
    test_all_incdec_arith!(0);
    test_all_incdec_arith!(1);
    test_all_incdec_arith!(2);
    test_all_incdec_arith!(-1);
    test_all_incdec_arith!(1u64 << 60);
    test_all_incdec_arith!(1.5);

    let mut ia = [0; 2];
    let ia_ptr = ia.as_mut_ptr();
    test_incdec!(usize, ia_ptr.wrapping_add(1), pre_increment, post_increment, true, 1);
    test_incdec!(usize, ia_ptr.wrapping_add(1), pre_decrement, post_decrement, true, -1);
    test_incdec!(usize, ia_ptr.wrapping_add(1), pre_increment, post_increment, false, 1);
    test_incdec!(usize, ia_ptr.wrapping_add(1), pre_decrement, post_decrement, false, -1);
}

fn main() {
    test_incdec();
    exit(0);
}