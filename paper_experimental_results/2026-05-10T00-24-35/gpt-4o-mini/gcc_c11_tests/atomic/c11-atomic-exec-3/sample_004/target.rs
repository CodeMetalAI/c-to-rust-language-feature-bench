use std::process;

macro_rules! test_incdec {
    ($t:ty, $value:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {
        let mut a: $t = $value;
        if ($preop a $postop) != if $pre_p { $value + $change } else { $value } {
            process::abort();
        }
        if a != $value + $change {
            process::abort();
        }
        a = $preop a; // apply pre-operation to a
    };
}

macro_rules! test_incdec_arith {
    ($value:expr) => {
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
    };
}

macro_rules! test_all_incdec_arith {
    ($value:expr) => {
        test_incdec_arith!($value);
        test_incdec_arith!($value);
        test_incdec_arith!($value);
        test_incdec_arith!($value);
    };
}

fn test_incdec() {
    test_all_incdec_arith!(0);
    test_all_incdec_arith!(1);
    test_all_incdec_arith!(2);
    test_all_incdec_arith!(-1);
    test_all_incdec_arith!(1 << 60);
    test_all_incdec_arith!(1.5);
}

fn main() {
    test_incdec();
    process::exit(0);
}