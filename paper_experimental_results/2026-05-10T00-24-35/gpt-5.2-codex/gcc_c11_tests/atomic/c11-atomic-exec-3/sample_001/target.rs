use std::process;

fn abort() -> ! {
    process::abort();
}

fn check(cond: bool) {
    if !cond {
        abort();
    }
}

macro_rules! test_incdec_int {
    ($t:ty, $value:expr, $pre:expr, $change:expr) => {{
        let mut a: $t = $value;
        let orig: $t = a;
        let result: $t = if $pre {
            if $change == 1 {
                a = a.wrapping_add(1 as $t);
            } else {
                a = a.wrapping_sub(1 as $t);
            }
            a
        } else {
            let old = a;
            if $change == 1 {
                a = a.wrapping_add(1 as $t);
            } else {
                a = a.wrapping_sub(1 as $t);
            }
            old
        };
        let expected: $t = if $pre {
            if $change == 1 {
                orig.wrapping_add(1 as $t)
            } else {
                orig.wrapping_sub(1 as $t)
            }
        } else {
            orig
        };
        let expected_after: $t = if $change == 1 {
            orig.wrapping_add(1 as $t)
        } else {
            orig.wrapping_sub(1 as $t)
        };
        check(result == expected);
        check(a == expected_after);
    }};
}

macro_rules! test_incdec_float {
    ($t:ty, $value:expr, $pre:expr, $change:expr) => {{
        let mut a: $t = $value;
        let orig: $t = a;
        let result: $t = if $pre {
            a = a + ($change as $t);
            a
        } else {
            let old = a;
            a = a + ($change as $t);
            old
        };
        let expected: $t = if $pre {
            orig + ($change as $t)
        } else {
            orig
        };
        let expected_after: $t = orig + ($change as $t);
        check(result == expected);
        check(a == expected_after);
    }};
}

fn test_incdec_bool(value: f64, pre: bool, change: i32) {
    let mut a: bool = value != 0.0;
    let orig = a;
    let op = |b: bool| -> bool {
        let mut i = if b { 1i32 } else { 0i32 };
        if change == 1 {
            i += 1;
        } else {
            i -= 1;
        }
        i != 0
    };
    let result = if pre {
        a = op(a);
        a
    } else {
        let old = a;
        a = op(a);
        old
    };
    let expected = if pre { op(orig) } else { orig };
    let expected_after = op(orig);
    check(result == expected);
    check(a == expected_after);
}

macro_rules! test_incdec_arith {
    ($value:expr, $pre:expr, $change:expr) => {{
        let value = $value;
        test_incdec_bool(value as f64, $pre, $change);
        test_incdec_int!(i8, value as i8, $pre, $change); // char
        test_incdec_int!(i8, value as i8, $pre, $change); // signed char
        test_incdec_int!(u8, value as u8, $pre, $change); // unsigned char
        test_incdec_int!(i16, value as i16, $pre, $change); // signed short
        test_incdec_int!(u16, value as u16, $pre, $change); // unsigned short
        test_incdec_int!(i32, value as i32, $pre, $change); // signed int
        test_incdec_int!(u32, value as u32, $pre, $change); // unsigned int
        test_incdec_int!(i64, value as i64, $pre, $change); // signed long
        test_incdec_int!(u64, value as u64, $pre, $change); // unsigned long
        test_incdec_int!(i64, value as i64, $pre, $change); // signed long long
        test_incdec_int!(u64, value as u64, $pre, $change); // unsigned long long
        test_incdec_float!(f32, value as f32, $pre, $change); // float
        test_incdec_float!(f64, value as f64, $pre, $change); // double
        test_incdec_float!(f64, value as f64, $pre, $change); // long double
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

fn test_incdec_ptr(pre: bool, change: i32) {
    let mut a: isize = 1;
    let orig = a;
    let result = if pre {
        if change == 1 {
            a = a.wrapping_add(1);
        } else {
            a = a.wrapping_sub(1);
        }
        a
    } else {
        let old = a;
        if change == 1 {
            a = a.wrapping_add(1);
        } else {
            a = a.wrapping_sub(1);
        }
        old
    };
    let expected = if pre {
        if change == 1 {
            orig.wrapping_add(1)
        } else {
            orig.wrapping_sub(1)
        }
    } else {
        orig
    };
    let expected_after = if change == 1 {
        orig.wrapping_add(1)
    } else {
        orig.wrapping_sub(1)
    };
    check(result == expected);
    check(a == expected_after);
}

fn test_incdec() {
    test_all_incdec_arith!(0);
    test_all_incdec_arith!(1);
    test_all_incdec_arith!(2);
    test_all_incdec_arith!(-1);
    test_all_incdec_arith!(1u64 << 60);
    test_all_incdec_arith!(1.5);
    let _ia = [0i32; 2];
    test_incdec_ptr(true, 1);
    test_incdec_ptr(true, -1);
    test_incdec_ptr(false, 1);
    test_incdec_ptr(false, -1);
}

fn main() {
    test_incdec();
    process::exit(0);
}