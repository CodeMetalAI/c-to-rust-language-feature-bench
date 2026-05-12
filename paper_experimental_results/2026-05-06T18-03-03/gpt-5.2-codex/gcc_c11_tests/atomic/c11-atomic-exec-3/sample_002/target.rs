use std::process;

#[derive(Copy, Clone)]
enum Val {
    Int(i128),
    Float(f64),
}

impl Val {
    fn to_bool(self) -> bool {
        match self {
            Val::Int(i) => i != 0,
            Val::Float(f) => f != 0.0,
        }
    }
    fn to_i8(self) -> i8 {
        match self {
            Val::Int(i) => i as i8,
            Val::Float(f) => f as i8,
        }
    }
    fn to_u8(self) -> u8 {
        match self {
            Val::Int(i) => i as u8,
            Val::Float(f) => f as u8,
        }
    }
    fn to_i16(self) -> i16 {
        match self {
            Val::Int(i) => i as i16,
            Val::Float(f) => f as i16,
        }
    }
    fn to_u16(self) -> u16 {
        match self {
            Val::Int(i) => i as u16,
            Val::Float(f) => f as u16,
        }
    }
    fn to_i32(self) -> i32 {
        match self {
            Val::Int(i) => i as i32,
            Val::Float(f) => f as i32,
        }
    }
    fn to_u32(self) -> u32 {
        match self {
            Val::Int(i) => i as u32,
            Val::Float(f) => f as u32,
        }
    }
    fn to_i64(self) -> i64 {
        match self {
            Val::Int(i) => i as i64,
            Val::Float(f) => f as i64,
        }
    }
    fn to_u64(self) -> u64 {
        match self {
            Val::Int(i) => i as u64,
            Val::Float(f) => f as u64,
        }
    }
    fn to_f32(self) -> f32 {
        match self {
            Val::Int(i) => i as f32,
            Val::Float(f) => f as f32,
        }
    }
    fn to_f64(self) -> f64 {
        match self {
            Val::Int(i) => i as f64,
            Val::Float(f) => f,
        }
    }
}

trait IncDec: Copy + PartialEq {
    fn add_change(self, change: i32) -> Self;
}

impl IncDec for bool {
    fn add_change(self, change: i32) -> Self {
        let v = if self { 1i32 } else { 0i32 };
        let res = v.wrapping_add(change);
        res != 0
    }
}

macro_rules! impl_incdec_signed {
    ($t:ty) => {
        impl IncDec for $t {
            fn add_change(self, change: i32) -> Self {
                if change >= 0 {
                    self.wrapping_add(change as $t)
                } else {
                    self.wrapping_sub((-change) as $t)
                }
            }
        }
    };
}

macro_rules! impl_incdec_unsigned {
    ($t:ty) => {
        impl IncDec for $t {
            fn add_change(self, change: i32) -> Self {
                if change >= 0 {
                    self.wrapping_add(change as $t)
                } else {
                    self.wrapping_sub((-change) as $t)
                }
            }
        }
    };
}

impl_incdec_signed!(i8);
impl_incdec_signed!(i16);
impl_incdec_signed!(i32);
impl_incdec_signed!(i64);
impl_incdec_signed!(isize);

impl_incdec_unsigned!(u8);
impl_incdec_unsigned!(u16);
impl_incdec_unsigned!(u32);
impl_incdec_unsigned!(u64);
impl_incdec_unsigned!(usize);

impl IncDec for f32 {
    fn add_change(self, change: i32) -> Self {
        self + change as f32
    }
}
impl IncDec for f64 {
    fn add_change(self, change: i32) -> Self {
        self + change as f64
    }
}

fn test_incdec_generic<T: IncDec + PartialEq + Copy>(value: T, pre: bool, change: i32) {
    let mut a = value;
    let ret;
    if pre {
        let new = a.add_change(change);
        a = new;
        ret = new;
    } else {
        let old = a;
        let new = a.add_change(change);
        a = new;
        ret = old;
    }
    let expected_ret = if pre {
        value.add_change(change)
    } else {
        value
    };
    let expected_a = value.add_change(change);
    if ret != expected_ret || a != expected_a {
        process::abort();
    }
}

fn test_incdec_arith(value: Val, pre: bool, change: i32) {
    test_incdec_generic::<bool>(value.to_bool(), pre, change);
    test_incdec_generic::<i8>(value.to_i8(), pre, change); // char
    test_incdec_generic::<i8>(value.to_i8(), pre, change); // signed char
    test_incdec_generic::<u8>(value.to_u8(), pre, change);
    test_incdec_generic::<i16>(value.to_i16(), pre, change);
    test_incdec_generic::<u16>(value.to_u16(), pre, change);
    test_incdec_generic::<i32>(value.to_i32(), pre, change);
    test_incdec_generic::<u32>(value.to_u32(), pre, change);
    test_incdec_generic::<i64>(value.to_i64(), pre, change);
    test_incdec_generic::<u64>(value.to_u64(), pre, change);
    test_incdec_generic::<i64>(value.to_i64(), pre, change); // long long
    test_incdec_generic::<u64>(value.to_u64(), pre, change); // unsigned long long
    test_incdec_generic::<f32>(value.to_f32(), pre, change);
    test_incdec_generic::<f64>(value.to_f64(), pre, change);
    test_incdec_generic::<f64>(value.to_f64(), pre, change); // long double
}

fn test_all_incdec_arith(value: Val) {
    test_incdec_arith(value, true, 1);
    test_incdec_arith(value, true, -1);
    test_incdec_arith(value, false, 1);
    test_incdec_arith(value, false, -1);
}

fn test_incdec() {
    test_all_incdec_arith(Val::Int(0));
    test_all_incdec_arith(Val::Int(1));
    test_all_incdec_arith(Val::Int(2));
    test_all_incdec_arith(Val::Int(-1));
    test_all_incdec_arith(Val::Int(1i128 << 60));
    test_all_incdec_arith(Val::Float(1.5));

    let _ia = [0i32; 2];
    let base = 1isize;
    test_incdec_generic::<isize>(base, true, 1);
    test_incdec_generic::<isize>(base, true, -1);
    test_incdec_generic::<isize>(base, false, 1);
    test_incdec_generic::<isize>(base, false, -1);
}

fn main() {
    test_incdec();
}