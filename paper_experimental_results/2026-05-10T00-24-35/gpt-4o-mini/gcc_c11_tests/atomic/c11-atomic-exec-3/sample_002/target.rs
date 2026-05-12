use std::process;

macro_rules! test_incdec {
    ($type:ty, $value:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {{
        static mut A: std::sync::atomic::AtomicPtr<$type> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
        let initial = $value as $type;
        let expected = if $pre_p {
            initial.wrapping_add($change as $type)
        } else {
            initial
        };
        let result = unsafe { A.load(std::sync::atomic::Ordering::SeqCst) };
        if $preop result $postop != expected {
            process::abort();
        }
        if result != (initial.wrapping_add($change as $type)) {
            process::abort();
        }
    }};
}

macro_rules! test_incdec_arith {
    ($value:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {{
        test_incdec!(bool, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(char, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(i8, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(u8, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(i16, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(u16, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(i32, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(u32, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(i64, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(u64, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(i128, $value, $preop, $postop, $pre_p, $change);
        test_incdec!(u128, $value, $preop, $postop, $pre_p, $change);
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
    static IA: [i32; 2] = [0; 2];
    test_incdec!(i32, &IA[1], ++, , true, 1);
    test_incdec!(i32, &IA[1], --, , true, -1);
    test_incdec!(i32, &IA[1], , ++, false, 1);
    test_incdec!(i32, &IA[1], , --, false, -1);
}

fn main() {
    test_incdec();
    process::exit(0);
}