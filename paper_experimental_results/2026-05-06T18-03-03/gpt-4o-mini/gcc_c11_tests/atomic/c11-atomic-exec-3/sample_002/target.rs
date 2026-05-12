use std::process;

macro_rules! test_incdec {
    ($type:ty, $value:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {{
        static mut A: std::sync::atomic::AtomicPtr<$type> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
        let initial_value: $type = $value;
        let a = unsafe { A.get_mut() }.unwrap_or(&mut std::ptr::null_mut() as *mut _);
        *a = &mut initial_value as *mut $type;

        let expected = if $pre_p {
            initial_value + $change
        } else {
            initial_value
        };

        let result = if $preop == "++" {
            unsafe { (*a).wrapping_add(1) }
        } else if $preop == "--" {
            unsafe { (*a).wrapping_sub(1) }
        } else {
            *a
        };

        if result != expected {
            process::abort();
        }

        if *a != initial_value + $change {
            process::abort();
        }
    }};
}

macro_rules! test_incdec_arith {
    ($value:expr) => {{
        test_incdec!(bool, $value, ++, , true, 1);
        test_incdec!(char, $value, ++, , true, 1);
        test_incdec!(i8, $value, ++, , true, 1);
        test_incdec!(u8, $value, ++, , true, 1);
        test_incdec!(i16, $value, ++, , true, 1);
        test_incdec!(u16, $value, ++, , true, 1);
        test_incdec!(i32, $value, ++, , true, 1);
        test_incdec!(u32, $value, ++, , true, 1);
        test_incdec!(i64, $value, ++, , true, 1);
        test_incdec!(u64, $value, ++, , true, 1);
        test_incdec!(i128, $value, ++, , true, 1);
        test_incdec!(u128, $value, ++, , true, 1);
        test_incdec!(f32, $value, ++, , true, 1);
        test_incdec!(f64, $value, ++, , true, 1);
    }};
}

macro_rules! test_all_incdec_arith {
    ($value:expr) => {{
        test_incdec_arith!($value);
        test_incdec_arith!($value);
        test_incdec_arith!($value);
        test_incdec_arith!($value);
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
    test_incdec!(i32, &mut ia[1], ++, , true, 1);
    test_incdec!(i32, &mut ia[1], --, , true, -1);
    test_incdec!(i32, &mut ia[1], , ++, false, 1);
    test_incdec!(i32, &mut ia[1], , --, false, -1);
}

fn main() {
    test_incdec();
    process::exit(0);
}