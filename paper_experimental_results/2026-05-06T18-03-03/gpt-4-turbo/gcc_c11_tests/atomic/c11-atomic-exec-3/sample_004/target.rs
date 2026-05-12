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

fn test_incdec_generic<T>(value: &mut T, change: T)
where
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + PartialEq + Copy,
{
    let original = *value;
    *value = *value + change;
    assert_eq!(*value, original + change);
    *value = *value - change;
    assert_eq!(*value, original);
}

fn test_all_incdec_arith<T>(value: T)
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + PartialEq
        + Copy
        + From<u8>
        + std::fmt::Debug,
{
    test_incdec_arith(value, 1);
    test_incdec_arith(value, -1);
}

fn test_incdec_arith<T>(value: T, change: T)
where
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + PartialEq + Copy,
{
    let mut x = value;
    x = x + change;
    assert_eq!(x, value + change);
    x = x - change;
    assert_eq!(x, value);
}