use std::process::abort;

trait IncDec: Copy + PartialEq {
    fn add_change(self, change: i32) -> Self;
}

impl IncDec for bool {
    fn add_change(self, change: i32) -> Self {
        let v = if self { 1 } else { 0 };
        let res = v + change;
        res != 0
    }
}

impl IncDec for i8 {
    fn add_change(self, change: i32) -> Self {
        self.wrapping_add(change as i8)
    }
}
impl IncDec for u8 {
    fn add_change(self, change: i32) -> Self {
        self.wrapping_add(change as u8)
    }
}
impl IncDec for i16 {
    fn add_change(self, change: i32) -> Self {
        self.wrapping_add(change as i16)
    }
}
impl IncDec for u16 {
    fn add_change(self, change: i32) -> Self {
        self.wrapping_add(change as u16)
    }
}
impl IncDec for i32 {
    fn add_change(self, change: i32) -> Self {
        self.wrapping_add(change as i32)
    }
}
impl IncDec for u32 {
    fn add_change(self, change: i32) -> Self {
        self.wrapping_add(change as u32)
    }
}
impl IncDec for i64 {
    fn add_change(self, change: i32) -> Self {
        self.wrapping_add(change as i64)
    }
}
impl IncDec for u64 {
    fn add_change(self, change: i32) -> Self {
        self.wrapping_add(change as u64)
    }
}
impl IncDec for isize {
    fn add_change(self, change: i32) -> Self {
        self.wrapping_add(change as isize)
    }
}

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

fn test_incdec<T: IncDec + PartialEq + Copy>(value: T, pre: bool, change: i32) {
    let mut a = value;
    let result = if pre {
        a = a.add_change(change);
        a
    } else {
        let old = a;
        a = a.add_change(change);
        old
    };
    let expected = if pre { value.add_change(change) } else { value };
    if result != expected {
        abort();
    }
    let expected_after = value.add_change(change);
    if a != expected_after {
        abort();
    }
}

fn test_all_incdec_arith_for<T: IncDec + PartialEq + Copy>(value: T) {
    test_incdec(value, true, 1);
    test_incdec(value, true, -1);
    test_incdec(value, false, 1);
    test_incdec(value, false, -1);
}

fn test_all_incdec_arith_from_int(value: i128) {
    test_all_incdec_arith_for::<bool>(value != 0);
    test_all_incdec_arith_for::<i8>(value as i8);
    test_all_incdec_arith_for::<i8>(value as i8);
    test_all_incdec_arith_for::<u8>(value as u8);
    test_all_incdec_arith_for::<i16>(value as i16);
    test_all_incdec_arith_for::<u16>(value as u16);
    test_all_incdec_arith_for::<i32>(value as i32);
    test_all_incdec_arith_for::<u32>(value as u32);
    test_all_incdec_arith_for::<i64>(value as i64);
    test_all_incdec_arith_for::<u64>(value as u64);
    test_all_incdec_arith_for::<i64>(value as i64);
    test_all_incdec_arith_for::<u64>(value as u64);
    test_all_incdec_arith_for::<f32>(value as f32);
    test_all_incdec_arith_for::<f64>(value as f64);
    test_all_incdec_arith_for::<f64>(value as f64);
}

fn test_all_incdec_arith_from_u64(value: u64) {
    test_all_incdec_arith_for::<bool>(value != 0);
    test_all_incdec_arith_for::<i8>(value as i8);
    test_all_incdec_arith_for::<i8>(value as i8);
    test_all_incdec_arith_for::<u8>(value as u8);
    test_all_incdec_arith_for::<i16>(value as i16);
    test_all_incdec_arith_for::<u16>(value as u16);
    test_all_incdec_arith_for::<i32>(value as i32);
    test_all_incdec_arith_for::<u32>(value as u32);
    test_all_incdec_arith_for::<i64>(value as i64);
    test_all_incdec_arith_for::<u64>(value as u64);
    test_all_incdec_arith_for::<i64>(value as i64);
    test_all_incdec_arith_for::<u64>(value as u64);
    test_all_incdec_arith_for::<f32>(value as f32);
    test_all_incdec_arith_for::<f64>(value as f64);
    test_all_incdec_arith_for::<f64>(value as f64);
}

fn test_all_incdec_arith_from_f64(value: f64) {
    test_all_incdec_arith_for::<bool>(value != 0.0);
    test_all_incdec_arith_for::<i8>(value as i8);
    test_all_incdec_arith_for::<i8>(value as i8);
    test_all_incdec_arith_for::<u8>(value as u8);
    test_all_incdec_arith_for::<i16>(value as i16);
    test_all_incdec_arith_for::<u16>(value as u16);
    test_all_incdec_arith_for::<i32>(value as i32);
    test_all_incdec_arith_for::<u32>(value as u32);
    test_all_incdec_arith_for::<i64>(value as i64);
    test_all_incdec_arith_for::<u64>(value as u64);
    test_all_incdec_arith_for::<i64>(value as i64);
    test_all_incdec_arith_for::<u64>(value as u64);
    test_all_incdec_arith_for::<f32>(value as f32);
    test_all_incdec_arith_for::<f64>(value as f64);
    test_all_incdec_arith_for::<f64>(value as f64);
}

fn test_incdec_all() {
    test_all_incdec_arith_from_int(0);
    test_all_incdec_arith_from_int(1);
    test_all_incdec_arith_from_int(2);
    test_all_incdec_arith_from_int(-1);
    test_all_incdec_arith_from_u64(1u64 << 60);
    test_all_incdec_arith_from_f64(1.5);

    let _ia = [0i32; 2];
    let ptr_index: isize = 1;
    test_incdec(ptr_index, true, 1);
    test_incdec(ptr_index, true, -1);
    test_incdec(ptr_index, false, 1);
    test_incdec(ptr_index, false, -1);
}

fn main() {
    test_incdec_all();
}