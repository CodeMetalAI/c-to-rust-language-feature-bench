fn abort() {
    std::process::abort();
}

fn test_bool(value: bool) {
    let mut a = value;
    // pre ++
    a = true;
    let result = a;
    let expected_result = true;
    if result != expected_result {
        abort();
    }
    let expected_a = true;
    if a != expected_a {
        abort();
    }
    // pre --
    a = value;
    a = !a;
    let result = a;
    let expected_result = !value;
    if result != expected_result {
        abort();
    }
    let expected_a = !value;
    if a != expected_a {
        abort();
    }
    // post ++
    a = value;
    let result = a;
    a = true;
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = true;
    if a != expected_a {
        abort();
    }
    // post --
    a = value;
    let result = a;
    a = !a;
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = !value;
    if a != expected_a {
        abort();
    }
}

fn test_i8(value: i8) {
    let mut a = value;
    // pre ++
    a = a.wrapping_add(1);
    let result = a;
    let expected_result = value.wrapping_add(1);
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_add(1);
    if a != expected_a {
        abort();
    }
    // pre --
    a = value;
    a = a.wrapping_sub(1);
    let result = a;
    let expected_result = value.wrapping_sub(1);
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_sub(1);
    if a != expected_a {
        abort();
    }
    // post ++
    a = value;
    let result = a;
    a = a.wrapping_add(1);
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_add(1);
    if a != expected_a {
        abort();
    }
    // post --
    a = value;
    let result = a;
    a = a.wrapping_sub(1);
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_sub(1);
    if a != expected_a {
        abort();
    }
}

fn test_u8(value: u8) {
    let mut a = value;
    // pre ++
    a = a.wrapping_add(1);
    let result = a;
    let expected_result = value.wrapping_add(1);
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_add(1);
    if a != expected_a {
        abort();
    }
    // pre --
    a = value;
    a = a.wrapping_sub(1);
    let result = a;
    let expected_result = value.wrapping_sub(1);
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_sub(1);
    if a != expected_a {
        abort();
    }
    // post ++
    a = value;
    let result = a;
    a = a.wrapping_add(1);
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_add(1);
    if a != expected_a {
        abort();
    }
    // post --
    a = value;
    let result = a;
    a = a.wrapping_sub(1);
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_sub(1);
    if a != expected_a {
        abort();
    }
}

fn test_i16(value: i16) {
    let mut a = value;
    // pre ++
    a = a.wrapping_add(1);
    let result = a;
    let expected_result = value.wrapping_add(1);
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_add(1);
    if a != expected_a {
        abort();
    }
    // pre --
    a = value;
    a = a.wrapping_sub(1);
    let result = a;
    let expected_result = value.wrapping_sub(1);
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_sub(1);
    if a != expected_a {
        abort();
    }
    // post ++
    a = value;
    let result = a;
    a = a.wrapping_add(1);
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_add(1);
    if a != expected_a {
        abort();
    }
    // post --
    a = value;
    let result = a;
    a = a.wrapping_sub(1);
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_sub(1);
    if a != expected_a {
        abort();
    }
}

fn test_u16(value: u16) {
    let mut a = value;
    // pre ++
    a = a.wrapping_add(1);
    let result = a;
    let expected_result = value.wrapping_add(1);
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_add(1);
    if a != expected_a {
        abort();
    }
    // pre --
    a = value;
    a = a.wrapping_sub(1);
    let result = a;
    let expected_result = value.wrapping_sub(1);
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_sub(1);
    if a != expected_a {
        abort();
    }
    // post ++
    a = value;
    let result = a;
    a = a.wrapping_add(1);
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_add(1);
    if a != expected_a {
        abort();
    }
    // post --
    a = value;
    let result = a;
    a = a.wrapping_sub(1);
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_sub(1);
    if a != expected_a {
        abort();
    }
}

fn test_i32(value: i32) {
    let mut a = value;
    // pre ++
    a = a.wrapping_add(1);
    let result = a;
    let expected_result = value.wrapping_add(1);
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_add(1);
    if a != expected_a {
        abort();
    }
    // pre --
    a = value;
    a = a.wrapping_sub(1);
    let result = a;
    let expected_result = value.wrapping_sub(1);
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_sub(1);
    if a != expected_a {
        abort();
    }
    // post ++
    a = value;
    let result = a;
    a = a.wrapping_add(1);
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_add(1);
    if a != expected_a {
        abort();
    }
    // post --
    a = value;
    let result = a;
    a = a.wrapping_sub(1);
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_sub(1);
    if a != expected_a {
        abort();
    }
}

fn test_u32(value: u32) {
    let mut a = value;
    // pre ++
    a = a.wrapping_add(1);
    let result = a;
    let expected_result = value.wrapping_add(1);
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_add(1);
    if a != expected_a {
        abort();
    }
    // pre --
    a = value;
    a = a.wrapping_sub(1);
    let result = a;
    let expected_result = value.wrapping_sub(1);
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_sub(1);
    if a != expected_a {
        abort();
    }
    // post ++
    a = value;
    let result = a;
    a = a.wrapping_add(1);
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_add(1);
    if a != expected_a {
        abort();
    }
    // post --
    a = value;
    let result = a;
    a = a.wrapping_sub(1);
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_sub(1);
    if a != expected_a {
        abort();
    }
}

fn test_isize(value: isize) {
    let mut a = value;
    // pre ++
    a = a.wrapping_add(1);
    let result = a;
    let expected_result = value.wrapping_add(1);
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_add(1);
    if a != expected_a {
        abort();
    }
    // pre --
    a = value;
    a = a.wrapping_sub(1);
    let result = a;
    let expected_result = value.wrapping_sub(1);
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_sub(1);
    if a != expected_a {
        abort();
    }
    // post ++
    a = value;
    let result = a;
    a = a.wrapping_add(1);
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_add(1);
    if a != expected_a {
        abort();
    }
    // post --
    a = value;
    let result = a;
    a = a.wrapping_sub(1);
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_sub(1);
    if a != expected_a {
        abort();
    }
}

fn test_usize(value: usize) {
    let mut a = value;
    // pre ++
    a = a.wrapping_add(1);
    let result = a;
    let expected_result = value.wrapping_add(1);
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_add(1);
    if a != expected_a {
        abort();
    }
    // pre --
    a = value;
    a = a.wrapping_sub(1);
    let result = a;
    let expected_result = value.wrapping_sub(1);
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_sub(1);
    if a != expected_a {
        abort();
    }
    // post ++
    a = value;
    let result = a;
    a = a.wrapping_add(1);
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_add(1);
    if a != expected_a {
        abort();
    }
    // post --
    a = value;
    let result = a;
    a = a.wrapping_sub(1);
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_sub(1);
    if a != expected_a {
        abort();
    }
}

fn test_i64(value: i64) {
    let mut a = value;
    // pre ++
    a = a.wrapping_add(1);
    let result = a;
    let expected_result = value.wrapping_add(1);
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_add(1);
    if a != expected_a {
        abort();
    }
    // pre --
    a = value;
    a = a.wrapping_sub(1);
    let result = a;
    let expected_result = value.wrapping_sub(1);
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_sub(1);
    if a != expected_a {
        abort();
    }
    // post ++
    a = value;
    let result = a;
    a = a.wrapping_add(1);
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_add(1);
    if a != expected_a {
        abort();
    }
    // post --
    a = value;
    let result = a;
    a = a.wrapping_sub(1);
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_sub(1);
    if a != expected_a {
        abort();
    }
}

fn test_u64(value: u64) {
    let mut a = value;
    // pre ++
    a = a.wrapping_add(1);
    let result = a;
    let expected_result = value.wrapping_add(1);
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_add(1);
    if a != expected_a {
        abort();
    }
    // pre --
    a = value;
    a = a.wrapping_sub(1);
    let result = a;
    let expected_result = value.wrapping_sub(1);
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_sub(1);
    if a != expected_a {
        abort();
    }
    // post ++
    a = value;
    let result = a;
    a = a.wrapping_add(1);
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_add(1);
    if a != expected_a {
        abort();
    }
    // post --
    a = value;
    let result = a;
    a = a.wrapping_sub(1);
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_sub(1);
    if a != expected_a {
        abort();
    }
}

fn test_f32(value: f32) {
    let mut a = value;
    // pre ++
    a += 1.0;
    let result = a;
    let expected_result = value + 1.0;
    if result != expected_result {
        abort();
    }
    let expected_a = value + 1.0;
    if a != expected_a {
        abort();
    }
    // pre --
    a = value;
    a -= 1.0;
    let result = a;
    let expected_result = value - 1.0;
    if result != expected_result {
        abort();
    }
    let expected_a = value - 1.0;
    if a != expected_a {
        abort();
    }
    // post ++
    a = value;
    let result = a;
    a += 1.0;
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value + 1.0;
    if a != expected_a {
        abort();
    }
    // post --
    a = value;
    let result = a;
    a -= 1.0;
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value - 1.0;
    if a != expected_a {
        abort();
    }
}

fn test_f64(value: f64) {
    let mut a = value;
    // pre ++
    a += 1.0;
    let result = a;
    let expected_result = value + 1.0;
    if result != expected_result {
        abort();
    }
    let expected_a = value + 1.0;
    if a != expected_a {
        abort();
    }
    // pre --
    a = value;
    a -= 1.0;
    let result = a;
    let expected_result = value - 1.0;
    if result != expected_result {
        abort();
    }
    let expected_a = value - 1.0;
    if a != expected_a {
        abort();
    }
    // post ++
    a = value;
    let result = a;
    a += 1.0;
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value + 1.0;
    if a != expected_a {
        abort();
    }
    // post --
    a = value;
    let result = a;
    a -= 1.0;
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value - 1.0;
    if a != expected_a {
        abort();
    }
}

fn test_ptr(value: *mut i32) {
    let mut a = value;
    // pre ++
    a = a.wrapping_offset(1);
    let result = a;
    let expected_result = value.wrapping_offset(1);
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_offset(1);
    if a != expected_a {
        abort();
    }
    // pre --
    a = value;
    a = a.wrapping_offset(-1);
    let result = a;
    let expected_result = value.wrapping_offset(-1);
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_offset(-1);
    if a != expected_a {
        abort();
    }
    // post ++
    a = value;
    let result = a;
    a = a.wrapping_offset(1);
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_offset(1);
    if a != expected_a {
        abort();
    }
    // post --
    a = value;
    let result = a;
    a = a.wrapping_offset(-1);
    let expected_result = value;
    if result != expected_result {
        abort();
    }
    let expected_a = value.wrapping_offset(-1);
    if a != expected_a {
        abort();
    }
}

fn test_incdec() {
    let values: Vec<i64> = vec![0, 1, 2, -1, 1i64 << 60];
    for &v in &values {
        test_bool(v != 0);
        test_i8(v as i8);
        test_u8(v as u8);
        test_i16(v as i16);
        test_u16(v as u16);
        test_i32(v as i32);
        test_u32(v as u32);
        test_isize(v as isize);
        test_usize(v as usize);
        test_i64(v);
        test_u64(v as u64);
        test_f32(v as f32);
        test_f64(v as f64);
    }
    let v = 1.5f64;
    test_bool(v != 0.0);
    test_i8(v as i8);
    test_u8(v as u8);
    test_i16(v as i16);
    test_u16(v as u16);
    test_i32(v as i32);
    test_u32(v as u32);
    test_isize(v as isize);
    test_usize(v as usize);
    test_i64(v as i64);
    test_u64(v as u64);
    test_f32(v as f32);
    test_f64(v);
    let mut ia: [i32; 2] = [0, 0];
    let ptr = &mut ia[1] as *mut i32;
    test_ptr(ptr);
}

fn main() {
    test_incdec();
    std::process::exit(0);
}