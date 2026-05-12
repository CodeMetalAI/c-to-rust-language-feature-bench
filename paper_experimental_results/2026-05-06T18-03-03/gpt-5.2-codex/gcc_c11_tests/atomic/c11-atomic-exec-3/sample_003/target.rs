use std::process;

#[derive(Clone, Copy)]
enum Value {
    Int(i128),
    Float(f64),
}

trait AtomOps: Copy + PartialEq {
    fn add(self, change: i32) -> Self;
}

trait FromValue: Sized {
    fn from_value(v: &Value) -> Self;
}

impl AtomOps for bool {
    fn add(self, change: i32) -> Self {
        let v = if self { 1 } else { 0 };
        (v + change) != 0
    }
}

impl FromValue for bool {
    fn from_value(v: &Value) -> Self {
        match v {
            Value::Int(x) => *x != 0,
            Value::Float(f) => *f != 0.0,
        }
    }
}

macro_rules! impl_int_traits {
    ($($ty:ty),+) => {
        $(
            impl AtomOps for $ty {
                fn add(self, change: i32) -> Self {
                    self.wrapping_add(change as $ty)
                }
            }
            impl FromValue for $ty {
                fn from_value(v: &Value) -> Self {
                    match v {
                        Value::Int(x) => *x as $ty,
                        Value::Float(f) => *f as $ty,
                    }
                }
            }
        )+
    };
}

macro_rules! impl_float_traits {
    ($($ty:ty),+) => {
        $(
            impl AtomOps for $ty {
                fn add(self, change: i32) -> Self {
                    self + change as $ty
                }
            }
            impl FromValue for $ty {
                fn from_value(v: &Value) -> Self {
                    match v {
                        Value::Int(x) => *x as $ty,
                        Value::Float(f) => *f as $ty,
                    }
                }
            }
        )+
    };
}

impl_int_traits!(i8, u8, i16, u16, i32, u32, i64, u64);
impl_float_traits!(f32, f64);

fn abort() -> ! {
    process::abort()
}

fn pre_op<T: AtomOps>(a: &mut T, change: i32) -> T {
    *a = a.add(change);
    *a
}

fn post_op<T: AtomOps>(a: &mut T, change: i32) -> T {
    let old = *a;
    *a = a.add(change);
    old
}

fn test_incdec<T: AtomOps + FromValue + PartialEq + Copy>(value: &Value, is_pre: bool, change: i32) {
    let initial = T::from_value(value);
    let expected_ret = if is_pre { initial.add(change) } else { initial };
    let mut a = initial;
    let ret = if is_pre {
        pre_op(&mut a, change)
    } else {
        post_op(&mut a, change)
    };
    if ret != expected_ret {
        abort();
    }
    let expected_final = initial.add(change);
    if a != expected_final {
        abort();
    }
}

macro_rules! test_incdec_arith {
    ($value:expr, $is_pre:expr, $change:expr) => {{
        test_incdec::<bool>(&$value, $is_pre, $change);
        test_incdec::<i8>(&$value, $is_pre, $change); // char
        test_incdec::<i8>(&$value, $is_pre, $change); // signed char
        test_incdec::<u8>(&$value, $is_pre, $change);
        test_incdec::<i16>(&$value, $is_pre, $change);
        test_incdec::<u16>(&$value, $is_pre, $change);
        test_incdec::<i32>(&$value, $is_pre, $change);
        test_incdec::<u32>(&$value, $is_pre, $change);
        test_incdec::<i64>(&$value, $is_pre, $change); // long
        test_incdec::<u64>(&$value, $is_pre, $change); // unsigned long
        test_incdec::<i64>(&$value, $is_pre, $change); // long long
        test_incdec::<u64>(&$value, $is_pre, $change); // unsigned long long
        test_incdec::<f32>(&$value, $is_pre, $change);
        test_incdec::<f64>(&$value, $is_pre, $change); // double
        test_incdec::<f64>(&$value, $is_pre, $change); // long double
    }};
}

fn test_all_incdec_arith(value: Value) {
    test_incdec_arith!(value, true, 1);
    test_incdec_arith!(value, true, -1);
    test_incdec_arith!(value, false, 1);
    test_incdec_arith!(value, false, -1);
}

fn test_ptr_incdec() {
    let initial: isize = 1;

    // pre-increment
    {
        let mut a = initial;
        let ret = {
            a += 1;
            a
        };
        if ret != 2 || a != 2 {
            abort();
        }
    }

    // pre-decrement
    {
        let mut a = initial;
        let ret = {
            a -= 1;
            a
        };
        if ret != 0 || a != 0 {
            abort();
        }
    }

    // post-increment
    {
        let mut a = initial;
        let ret = {
            let old = a;
            a += 1;
            old
        };
        if ret != 1 || a != 2 {
            abort();
        }
    }

    // post-decrement
    {
        let mut a = initial;
        let ret = {
            let old = a;
            a -= 1;
            old
        };
        if ret != 1 || a != 0 {
            abort();
        }
    }
}

fn test_incdec() {
    test_all_incdec_arith(Value::Int(0));
    test_all_incdec_arith(Value::Int(1));
    test_all_incdec_arith(Value::Int(2));
    test_all_incdec_arith(Value::Int(-1));
    test_all_incdec_arith(Value::Int(1i128 << 60));
    test_all_incdec_arith(Value::Float(1.5));
    test_ptr_incdec();
}

fn main() {
    test_incdec();
    process::exit(0);
}