use std::process::{abort, exit};
use std::mem;

static mut IA: [i32; 2] = [0; 2];

fn test_bool(value: i32, preop: bool, postop: bool, pre_p: bool, change: i32) {
    let initial = value;
    let mut current = initial;
    let tmp: bool;
    if preop {
        current += change;
        tmp = current != 0;
    } else if postop {
        tmp = current != 0;
        current += change;
    } else {
        tmp = current != 0;
    }
    let expected_tmp = if pre_p { (value + change) != 0 } else { value != 0 };
    if tmp != expected_tmp {
        abort();
    }
    if (current != 0) != ((value + change) != 0) {
        abort();
    }
}

fn test_i8(value: i32, preop: bool, postop: bool, pre_p: bool, change: i32) {
    let mut a: i8 = value as i8;
    let tmp: i8;
    if preop {
        a = a.wrapping_add(change as i8);
        tmp = a;
    } else if postop {
        tmp = a;
        a = a.wrapping_add(change as i8);
    } else {
        tmp = a;
    }
    let expected_tmp = if pre_p { (value as i8).wrapping_add(change as i8) } else { value as i8 };
    if tmp != expected_tmp {
        abort();
    }
    if a != (value as i8).wrapping_add(change as i8) {
        abort();
    }
}

fn test_u8(value: i32, preop: bool, postop: bool, pre_p: bool, change: i32) {
    let mut a: u8 = value as u8;
    let tmp: u8;
    if preop {
        a = a.wrapping_add(change as u8);
        tmp = a;
    } else if postop {
        tmp = a;
        a = a.wrapping_add(change as u8);
    } else {
        tmp = a;
    }
    let expected_tmp = if pre_p { (value as u8).wrapping_add(change as u8) } else { value as u8 };
    if tmp != expected_tmp {
        abort();
    }
    if a != (value as u8).wrapping_add(change as u8) {
        abort();
    }
}

fn test_i16(value: i32, preop: bool, postop: bool, pre_p: bool, change: i32) {
    let mut a: i16 = value as i16;
    let tmp: i16;
    if preop {
        a = a.wrapping_add(change as i16);
        tmp = a;
    } else if postop {
        tmp = a;
        a = a.wrapping_add(change as i16);
    } else {
        tmp = a;
    }
    let expected_tmp = if pre_p { (value as i16).wrapping_add(change as i16) } else { value as i16 };
    if tmp != expected_tmp {
        abort();
    }
    if a != (value as i16).wrapping_add(change as i16) {
        abort();
    }
}

fn test_u16(value: i32, preop: bool, postop: bool, pre_p: bool, change: i32) {
    let mut a: u16 = value as u16;
    let tmp: u16;
    if preop {
        a = a.wrapping_add(change as u16);
        tmp = a;
    } else if postop {
        tmp = a;
        a = a.wrapping_add(change as u16);
    } else {
        tmp = a;
    }
    let expected_tmp = if pre_p { (value as u16).wrapping_add(change as u16) } else { value as u16 };
    if tmp != expected_tmp {
        abort();
    }
    if a != (value as u16).wrapping_add(change as u16) {
        abort();
    }
}

fn test_i32(value: i32, preop: bool, postop: bool, pre_p: bool, change: i32) {
    let mut a: i32 = value;
    let tmp: i32;
    if preop {
        a = a.wrapping_add(change);
        tmp = a;
    } else if postop {
        tmp = a;
        a = a.wrapping_add(change);
    } else {
        tmp = a;
    }
    let expected_tmp = if pre_p { value.wrapping_add(change) } else { value };
    if tmp != expected_tmp {
        abort();
    }
    if a != value.wrapping_add(change) {
        abort();
    }
}

fn test_u32(value: i32, preop: bool, postop: bool, pre_p: bool, change: i32) {
    let mut a: u32 = value as u32;
    let tmp: u32;
    if preop {
        a = a.wrapping_add(change as u32);
        tmp = a;
    } else if postop {
        tmp = a;
        a = a.wrapping_add(change as u32);
    } else {
        tmp = a;
    }
    let expected_tmp = if pre_p { (value as u32).wrapping_add(change as u32) } else { value as u32 };
    if tmp != expected_tmp {
        abort();
    }
    if a != (value as u32).wrapping_add(change as u32) {
        abort();
    }
}

fn test_i64(value: i64, preop: bool, postop: bool, pre_p: bool, change: i32) {
    let mut a: i64 = value;
    let tmp: i64;
    if preop {
        a = a.wrapping_add(change as i64);
        tmp = a;
    } else if postop {
        tmp = a;
        a = a.wrapping_add(change as i64);
    } else {
        tmp = a;
    }
    let expected_tmp = if pre_p { value.wrapping_add(change as i64) } else { value };
    if tmp != expected_tmp {
        abort();
    }
    if a != value.wrapping_add(change as i64) {
        abort();
    }
}

fn test_u64(value: u64, preop: bool, postop: bool, pre_p: bool, change: i32) {
    let mut a: u64 = value;
    let tmp: u64;
    if preop {
        a = a.wrapping_add(change as u64);
        tmp = a;
    } else if postop {
        tmp = a;
        a = a.wrapping_add(change as u64);
    } else {
        tmp = a;
    }
    let expected_tmp = if pre_p { value.wrapping_add(change as u64) } else { value };
    if tmp != expected_tmp {
        abort();
    }
    if a != value.wrapping_add(change as u64) {
        abort();
    }
}

fn test_f32(value: f32, preop: bool, postop: bool, pre_p: bool, change: i32) {
    let mut a: f32 = value;
    let tmp: f32;
    if preop {
        a += change as f32;
        tmp = a;
    } else if postop {
        tmp = a;
        a += change as f32;
    } else {
        tmp = a;
    }
    let expected_tmp = if pre_p { value + change as f32 } else { value };
    if tmp != expected_tmp {
        abort();
    }
    if a != value + change as f32 {
        abort();
    }
}

fn test_f64(value: f64, preop: bool, postop: bool, pre_p: bool, change: i32) {
    let mut a: f64 = value;
    let tmp: f64;
    if preop {
        a += change as f64;
        tmp = a;
    } else if postop {
        tmp = a;
        a += change as f64;
    } else {
        tmp = a;
    }
    let expected_tmp = if pre_p { value + change as f64 } else { value };
    if tmp != expected_tmp {
        abort();
    }
    if a != value + change as f64 {
        abort();
    }
}

fn test_ptr(value: usize, preop: bool, postop: bool, pre_p: bool, change: i32) {
    let ptr_size = mem::size_of::<i32>() as usize;
    let change_usize = (change as isize as usize).wrapping_mul(ptr_size);
    let mut a: usize = value;
    let tmp: usize;
    if preop {
        a = a.wrapping_add(change_usize);
        tmp = a;
    } else if postop {
        tmp = a;
        a = a.wrapping_add(change_usize);
    } else {
        tmp = a;
    }
    let expected_tmp = if pre_p { value.wrapping_add(change_usize) } else { value };
    if tmp != expected_tmp {
        abort();
    }
    if a != value.wrapping_add(change_usize) {
        abort();
    }
}

fn test_incdec_arith(value: i64, preop: bool, postop: bool, pre_p: bool, change: i32) {
    test_bool(value as i32, preop, postop, pre_p, change);
    test_i8(value as i32, preop, postop, pre_p, change);
    test_u8(value as i32, preop, postop, pre_p, change);
    test_i16(value as i32, preop, postop, pre_p, change);
    test_u16(value as i32, preop, postop, pre_p, change);
    test_i32(value as i32, preop, postop, pre_p, change);
    test_u32(value as i32, preop, postop, pre_p, change);
    test_i64(value, preop, postop, pre_p, change);
    test_u64(value as u64, preop, postop, pre_p, change);
    test_f32(value as f32, preop, postop, pre_p, change);
    test_f64(value as f64, preop, postop, pre_p, change);
    test_f64(value as f64, preop, postop, pre_p, change); // long double as f64
}

fn test_all_incdec_arith(value: i64) {
    test_incdec_arith(value, true, false, true, 1); // ++
    test_incdec_arith(value, true, false, true, -1); // --
    test_incdec_arith(value, false, true, false, 1); // post ++
    test_incdec_arith(value, false, true, false, -1); // post --
}

fn test_incdec() {
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith(1u64.wrapping_shl(60) as i64);
    test_all_incdec_arith(1.5f64 as i64); // approximate for float test
    unsafe {
        let base = &IA[1] as *const i32 as usize;
        test_ptr(base, true, false, true, 1);
        test_ptr(base, true, false, true, -1);
        test_ptr(base, false, true, false, 1);
        test_ptr(base, false, true, false, -1);
    }
}

fn main() {
    test_incdec();
    exit(0);
}