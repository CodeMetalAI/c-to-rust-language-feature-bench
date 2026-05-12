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
    test_incdec_op(&mut ia[1], 1);
    test_incdec_op(&mut ia[1], -1);
}

fn test_incdec_op(ptr: &mut i32, change: i32) {
    let original = *ptr;
    *ptr += change;
    assert_eq!(*ptr, original + change);
}

fn test_all_incdec_arith<T>(value: T)
where
    T: std::cmp::Eq + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy + From<u8>,
{
    test_incdec_arith(value, 1);
    test_incdec_arith(value, -1);
}

fn test_incdec_arith<T>(mut value: T, change: T)
where
    T: std::cmp::Eq + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy + From<u8>,
{
    let original = value;
    value = value + change;
    assert_eq!(value, original + change);
    
    value = original;
    value = value - change;
    assert_eq!(value, original - change);
}