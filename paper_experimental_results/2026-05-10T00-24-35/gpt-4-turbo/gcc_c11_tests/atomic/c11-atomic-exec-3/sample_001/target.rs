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

    let mut ia = [0, 0];
    test_incdec_generic(&mut ia[1], 1, 1);
    test_incdec_generic(&mut ia[1], -1, 1);
    test_incdec_generic(&mut ia[1], 1, 0);
    test_incdec_generic(&mut ia[1], -1, 0);
}

fn test_incdec_generic<T>(a: &mut T, change: T, pre_p: bool) 
where
    T: Copy + PartialEq + std::ops::Add<Output = T> + std::ops::Sub<Output = T> {
    let old_value = *a;
    let new_value = if change > old_value {
        *a += change;
    } else {
        *a -= change;
    };
    let expected_value = if pre_p {
        old_value + change
    } else {
        old_value
    };
    assert_eq!(new_value, expected_value);
    assert_eq!(*a, old_value + change);
}

fn test_all_incdec_arith<T>(value: T) 
where
    T: Copy + PartialEq + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + From<u8> {
    test_incdec_arith(value, 1, 1);
    test_incdec_arith(value, -1, 1);
    test_incdec_arith(value, 1, 0);
    test_incdec_arith(value, -1, 0);
}

fn test_incdec_arith<T>(value: T, change: T, pre_p: bool) 
where
    T: Copy + PartialEq + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + From<u8> {
    test_incdec_type(value, change, pre_p);
    test_incdec_type(value as i8, change as i8, pre_p);
    test_incdec_type(value as u8, change as u8, pre_p);
    test_incdec_type(value as i16, change as i16, pre_p);
    test_incdec_type(value as u16, change as u16, pre_p);
    test_incdec_type(value as i32, change as i32, pre_p);
    test_incdec_type(value as u32, change as u32, pre_p);
    test_incdec_type(value as i64, change as i64, pre_p);
    test_incdec_type(value as u64, change as u64, pre_p);
    test_incdec_type(value as f32, change as f32, pre_p);
    test_incdec_type(value as f64, change as f64, pre_p);
}

fn test_incdec_type<T>(value: T, change: T, pre_p: bool) 
where
    T: Copy + PartialEq + std::ops::Add<Output = T> + std::ops::Sub<Output = T> {
    let mut a = value;
    let old_value = a;
    let new_value = if change > old_value {
        a += change;
    } else {
        a -= change;
    };
    let expected_value = if pre_p {
        old_value + change
    } else {
        old_value
    };
    assert_eq!(new_value, expected_value);
    assert_eq!(a, old_value + change);
}