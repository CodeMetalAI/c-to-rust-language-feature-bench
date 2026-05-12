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

fn test_all_incdec_arith<T: Copy + PartialEq + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + From<u8>>(value: T) {
    test_incdec_arith(value, 1);
    test_incdec_arith(value, -1);
}

fn test_incdec_arith<T: Copy + PartialEq + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + From<u8>>(value: T, change: T) {
    test_incdec_type(value, change);
}

fn test_incdec_type<T: Copy + PartialEq + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + From<u8>>(value: T, change: T) {
    let mut a = value;
    let expected = if change == T::from(1u8) { value + change } else { value - change };
    a = a + change;
    assert_eq!(a, expected);
    a = a - change;
    assert_eq!(a, value);
}

fn test_incdec_generic<T: Copy + PartialEq + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + From<i32>>(ptr: &mut T, change: i32) {
    let original = *ptr;
    let expected = if change == 1 { *ptr + T::from(1) } else { *ptr - T::from(1) };
    if change == 1 {
        *ptr = *ptr + T::from(1);
    } else {
        *ptr = *ptr - T::from(1);
    }
    assert_eq!(*ptr, expected);
    // Reset to original
    *ptr = original;
}