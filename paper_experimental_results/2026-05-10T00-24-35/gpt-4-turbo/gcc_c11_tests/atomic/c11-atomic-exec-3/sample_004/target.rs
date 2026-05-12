fn main() {
    test_incdec();
}

fn test_incdec() {
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith(1u64 << 60);
    test_all_incdec_arith(1.5);
    let mut ia = [0; 2];
    test_incdec_ptr(&mut ia[1], true, true, 1);
    test_incdec_ptr(&mut ia[1], true, false, -1);
    test_incdec_ptr(&mut ia[1], false, true, 1);
    test_incdec_ptr(&mut ia[1], false, false, -1);
}

fn test_all_incdec_arith<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + PartialEq + Copy>(
    value: T,
) {
    test_incdec_arith(value, true, true, 1);
    test_incdec_arith(value, true, false, -1);
    test_incdec_arith(value, false, true, 1);
    test_incdec_arith(value, false, false, -1);
}

fn test_incdec_arith<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + PartialEq + Copy>(
    value: T,
    pre_op: bool,
    increment: bool,
    change: T,
) {
    let mut a = value;
    let original_value = if pre_op { if increment { a = a + change; a } else { a = a - change; a } } else { a };
    assert_eq!(a, value + change);
    if !pre_op {
        if increment { a = a + change; } else { a = a - change; }
    }
    assert_eq!(a, value + change);
}

fn test_incdec_ptr(ptr: &mut i32, pre_op: bool, increment: bool, change: i32) {
    let original_ptr_value = unsafe { *ptr };
    let mut a = ptr;
    let original_a = a;
    if pre_op {
        if increment {
            *a += change;
            a = &mut unsafe { *a.offset(change as isize) };
        } else {
            *a -= change;
            a = &mut unsafe { *a.offset(-change as isize) };
        }
    }
    assert_eq!(unsafe { *a }, original_ptr_value + change);
    if !pre_op {
        if increment {
            *a += change;
            a = &mut unsafe { *a.offset(change as isize) };
        } else {
            *a -= change;
            a = &mut unsafe { *a.offset(-change as isize) };
        }
    }
    assert_eq!(unsafe { *a }, original_ptr_value + change);
}