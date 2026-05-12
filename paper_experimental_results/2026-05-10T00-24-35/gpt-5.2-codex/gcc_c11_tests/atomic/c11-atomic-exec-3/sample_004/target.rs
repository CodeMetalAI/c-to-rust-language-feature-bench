use std::process;

fn abort() -> ! {
    process::abort();
}

macro_rules! test_incdec_bool {
    ($value:expr, $pre:expr, $change:expr) => {{
        let mut a: i32 = if ($value as f64) != 0.0 { 1 } else { 0 };
        let old = a;
        let sum = a + $change;
        let expected = if sum != 0 { 1 } else { 0 };
        let ret = if $pre { expected } else { old };
        a = expected;
        if ret != (if $pre { expected } else { old }) {
            abort();
        }
        if a != expected {
            abort();
        }
    }};
}

macro_rules! test_incdec_signed {
    ($ty:ty, $value:expr, $pre:expr, $change:expr) => {{
        let mut a: $ty = ($value as i128) as $ty;
        let old: $ty = a;
        let expected: $ty = a.wrapping_add(($change) as $ty);
        let ret: $ty = if $pre { expected } else { old };
        a = expected;
        if ret != (if $pre { expected } else { old }) {
            abort();
        }
        if a != expected {
            abort();
        }
    }};
}

macro_rules! test_incdec_unsigned {
    ($ty:ty, $value:expr, $pre:expr, $change:expr) => {{
        let mut a: $ty = ($value as i128) as $ty;
        let old: $ty = a;
        let expected: $ty = a.wrapping_add(($change) as $ty);
        let ret: $ty = if $pre { expected } else { old };
        a = expected;
        if ret != (if $pre { expected } else { old }) {
            abort();
        }
        if a != expected {
            abort();
        }
    }};
}

macro_rules! test_incdec_float {
    ($ty:ty, $value:expr, $pre:expr, $change:expr) => {{
        let mut a: $ty = ($value) as $ty;
        let old: $ty = a;
        let expected: $ty = a + ($change as $ty);
        let ret: $ty = if $pre { expected } else { old };
        a = expected;
        if ret != (if $pre { expected } else { old }) {
            abort();
        }
        if a != expected {
            abort();
        }
    }};
}

macro_rules! test_incdec_arith {
    ($value:expr, $pre:expr, $change:expr) => {{
        test_incdec_bool!($value, $pre, $change);
        test_incdec_signed!(i8, $value, $pre, $change); // char
        test_incdec_signed!(i8, $value, $pre, $change); // signed char
        test_incdec_unsigned!(u8, $value, $pre, $change);
        test_incdec_signed!(i16, $value, $pre, $change);
        test_incdec_unsigned!(u16, $value, $pre, $change);
        test_incdec_signed!(i32, $value, $pre, $change);
        test_incdec_unsigned!(u32, $value, $pre, $change);
        test_incdec_signed!(isize, $value, $pre, $change);
        test_incdec_unsigned!(usize, $value, $pre, $change);
        test_incdec_signed!(i64, $value, $pre, $change);
        test_incdec_unsigned!(u64, $value, $pre, $change);
        test_incdec_float!(f32, $value, $pre, $change);
        test_incdec_float!(f64, $value, $pre, $change);
        test_incdec_float!(f64, $value, $pre, $change); // long double
    }};
}

macro_rules! test_all_incdec_arith {
    ($value:expr) => {{
        test_incdec_arith!($value, true, 1);
        test_incdec_arith!($value, true, -1);
        test_incdec_arith!($value, false, 1);
        test_incdec_arith!($value, false, -1);
    }};
}

fn test_incdec_ptr(value: isize, pre: bool, change: isize) {
    let mut a = value;
    let old = a;
    let expected = a + change;
    let ret = if pre { expected } else { old };
    a = expected;
    if ret != (if pre { expected } else { old }) {
        abort();
    }
    if a != expected {
        abort();
    }
}

fn test_incdec() {
    test_all_incdec_arith!(0);
    test_all_incdec_arith!(1);
    test_all_incdec_arith!(2);
    test_all_incdec_arith!(-1);
    test_all_incdec_arith!(1u64 << 60);
    test_all_incdec_arith!(1.5f64);
    let _ia = [0i32; 2];
    let base_index = 1isize;
    test_incdec_ptr(base_index, true, 1);
    test_incdec_ptr(base_index, true, -1);
    test_incdec_ptr(base_index, false, 1);
    test_incdec_ptr(base_index, false, -1);
}

fn main() {
    test_incdec();
    process::exit(0);
}