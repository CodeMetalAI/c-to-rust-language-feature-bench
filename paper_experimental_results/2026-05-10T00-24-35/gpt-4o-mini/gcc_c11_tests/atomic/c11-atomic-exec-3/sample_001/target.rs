use std::panic;

macro_rules! test_incdec {
    ($ty:ty, $value:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {{
        let mut a: std::sync::atomic::AtomicPtr<$ty> = std::sync::atomic::AtomicPtr::new(Box::into_raw(Box::new($value)));
        let expected = if $pre_p { $value + $change } else { $value };
        let result = unsafe { (*a.load(std::sync::atomic::Ordering::SeqCst)).$preop $postop };
        
        if result != expected {
            panic!();
        }
        if unsafe { *a.load(std::sync::atomic::Ordering::SeqCst) } != ($value + $change) {
            panic!();
        }
    }};
}

macro_rules! test_incdec_arith {
    ($value:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {{
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
        test_incdec_arith!($value, ++, , true, 1);
        test_incdec_arith!($value, --, , true, -1);
        test_incdec_arith!($value, , ++, false, 1);
        test_incdec_arith!($value, , --, false, -1);
    }};
}

fn test_incdec() {
    test_all_incdec_arith!(0);
    test_all_incdec_arith!(1);
    test_all_incdec_arith!(2);
    test_all_incdec_arith!(-1);
    test_all_incdec_arith!(1u64 << 60);
    test_all_incdec_arith!(1.5);
    let mut ia = [0; 2];
    test_incdec!(*mut i32, &mut ia[1] as *mut _ as _, ++, , true, 1);
    test_incdec!(*mut i32, &mut ia[1] as *mut _ as _, --, , true, -1);
    test_incdec!(*mut i32, &mut ia[1] as *mut _ as _, , ++, false, 1);
    test_incdec!(*mut i32, &mut ia[1] as *mut _ as _, , --, false, -1);
}

fn main() {
    let _ = panic::catch_unwind(|| {
        test_incdec();
    });
    std::process::exit(0);
}