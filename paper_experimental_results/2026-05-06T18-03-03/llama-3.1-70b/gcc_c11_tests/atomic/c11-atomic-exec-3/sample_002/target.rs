fn test_incdec<T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + PartialEq + std::fmt::Debug>(
    value: T,
    change: T,
    pre_op: bool,
    post_op: bool,
) {
    let mut a = std::sync::atomic::AtomicPtr::new(&value);
    let expected = if pre_op {
        &value + &change
    } else {
        &value
    };
    let result = if post_op {
        a.fetch_add(&change, std::sync::atomic::Ordering::SeqCst)
    } else {
        a.fetch_add(&change, std::sync::atomic::Ordering::SeqCst)
    };
    if result!= expected {
        panic!("assertion failed: result!= expected");
    }
    if a.load(std::sync::atomic::Ordering::SeqCst)!= &value + &change {
        panic!("assertion failed: a.load()!= value + change");
    }
}

fn test_incdec_arith<T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + PartialEq + std::fmt::Debug>(
    value: T,
) {
    test_incdec(value, T::from(1).unwrap(), true, false);
    test_incdec(value, T::from(-1).unwrap(), true, false);
    test_incdec(value, T::from(1).unwrap(), false, true);
    test_incdec(value, T::from(-1).unwrap(), false, true);
}

fn test_all_incdec_arith<T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + PartialEq + std::fmt::Debug>(
    value: T,
) {
    test_incdec_arith(value);
}

fn test_incdec() {
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith(1 << 60);
    test_all_incdec_arith(1.5);
    let ia = [0, 0];
    test_incdec(&ia[1], 1, true, false);
    test_incdec(&ia[1], -1, true, false);
    test_incdec(&ia[1], 1, false, true);
    test_incdec(&ia[1], -1, false, true);
}

fn main() {
    test_incdec();
}