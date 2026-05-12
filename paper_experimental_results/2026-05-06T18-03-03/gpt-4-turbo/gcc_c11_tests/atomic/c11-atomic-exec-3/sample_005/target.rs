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
    test_incdec_generic(&mut ia[1], 1);
    test_incdec_generic(&mut ia[1], -1);
}

fn test_incdec_generic<T>(a: &mut T, change: T)
where
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + PartialEq + Copy,
{
    let original = *a;
    *a = *a + change;
    assert_eq!(*a, original + change);
    *a = *a - change;
    assert_eq!(*a, original);
}

fn test_all_incdec_arith<T>(value: T)
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::AddAssign
        + std::ops::SubAssign
        + PartialEq
        + Copy,
{
    test_incdec_arith(value, 1);
    test_incdec_arith(value, -1);
}

fn test_incdec_arith<T>(value: T, change: T)
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::AddAssign
        + std::ops::SubAssign
        + PartialEq
        + Copy,
{
    let mut a = value;
    a += change;
    assert_eq!(a, value + change);
    a -= change;
    assert_eq!(a, value);

    let mut b = value;
    b -= change;
    assert_eq!(b, value - change);
    b += change;
    assert_eq!(b, value);
}