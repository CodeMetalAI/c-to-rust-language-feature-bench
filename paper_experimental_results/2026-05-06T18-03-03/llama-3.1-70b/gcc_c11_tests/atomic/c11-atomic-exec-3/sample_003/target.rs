fn main() {
    fn test_incdec<T: std::ops::AddAssign + std::ops::SubAssign + Copy + PartialEq + std::fmt::Debug>(
        value: T,
        preop: bool,
        postop: bool,
        change: T,
    ) {
        let mut a = std::sync::atomic::AtomicCell::new(value);
        let result = if preop {
            a.fetch_add(change, std::sync::atomic::Ordering::SeqCst) + change
        } else {
            a.fetch_add(change, std::sync::atomic::Ordering::SeqCst)
        };
        if result != (if preop { value + change } else { value }) {
            panic!("assertion failed");
        }
        if a.get() != (value + change) {
            panic!("assertion failed");
        }
    }

    fn test_incdec_arith<T: std::ops::AddAssign + std::ops::SubAssign + Copy + PartialEq + std::fmt::Debug>(
        value: T,
    ) {
        test_incdec(value, true, false, T::from(1));
        test_incdec(value, false, false, T::from(-1));
        test_incdec(value, false, true, T::from(1));
        test_incdec(value, false, false, T::from(-1));
    }

    fn test_all_incdec_arith<T: std::ops::AddAssign + std::ops::SubAssign + Copy + PartialEq + std::fmt::Debug>(
        value: T,
    ) {
        test_incdec_arith(value);
    }

    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith(1 << 60);
    test_all_incdec_arith(1.5);

    let mut ia = [0, 0];
    test_incdec(&ia[1], true, false, std::ptr::null::<i32>() as *mut i32);
    test_incdec(&ia[1], false, false, std::ptr::null::<i32>() as *mut i32);
    test_incdec(&ia[1], false, true, std::ptr::null::<i32>() as *mut i32);
    test_incdec(&ia[1], false, false, std::ptr::null::<i32>() as *mut i32);
}