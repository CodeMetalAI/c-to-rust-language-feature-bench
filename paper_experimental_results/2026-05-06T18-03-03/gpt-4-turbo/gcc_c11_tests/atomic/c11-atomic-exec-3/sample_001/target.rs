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
    test_incdec_generic(&mut ia[1], 1);
    test_incdec_generic(&mut ia[1], -1);
}

fn test_all_incdec_arith<T: Copy + PartialEq + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + From<i8>>(value: T) {
    test_incdec_arith(value, 1);
    test_incdec_arith(value, -1);
}

fn test_incdec_arith<T: Copy + PartialEq + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + From<i8>>(value: T, change: i8) {
    let change_t: T = change.into();
    test_incdec_op(value, change_t, true);
    test_incdec_op(value, change_t, false);
}

fn test_incdec_op<T: Copy + PartialEq + std::ops::Add<Output = T> + std::ops::Sub<Output = T>>(value: T, change: T, pre: bool) {
    let mut a = value;
    let expected = if pre { value + change } else { value };
    let new_value = if pre {
        if change == T::from(1i8) { a += change; a } else { a -= change; a }
    } else {
        let ret = a;
        if change == T::from(1i8) { a += change; } else { a -= change; }
        ret
    };
    assert_eq!(new_value, expected, "Failed on pre/post operation check");
    assert_eq!(a, value + change, "Failed on final value check");
}

fn test_incdec_generic(ptr: &mut i32, change: i32) {
    let original = ptr as *const i32 as usize;
    let expected = original.wrapping_add(change as usize);
    let mut a = ptr;
    if change == 1 {
        a = &mut *(a as *mut i32).wrapping_offset(1);
    } else {
        a = &mut *(a as *mut i32).wrapping_offset(-1);
    }
    assert_eq!(a as *const i32 as usize, expected, "Pointer arithmetic failed");
}