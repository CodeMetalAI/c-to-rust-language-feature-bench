use std::process;

fn test_incdec<T: Copy + PartialEq + std::ops::AddAssign + std::ops::SubAssign>(
    value: T,
    pre_p: bool,
    change: T,
) {
    let mut a = value;
    let result = if pre_p {
        a += change;
        a
    } else {
        let temp = a;
        a += change;
        temp
    };

    if result != value + change {
        process::exit(1);
    }

    if a != value + change {
        process::exit(1);
    }
}

fn test_incdec_arith<T: Copy + PartialEq + std::ops::AddAssign + std::ops::SubAssign>(
    value: T,
    pre_p: bool,
    change: T,
) {
    test_incdec(value, pre_p, change);
}

fn test_all_incdec_arith<T: Copy + PartialEq + std::ops::AddAssign + std::ops::SubAssign>(
    value: T,
) {
    test_incdec_arith(value, true, value + value);
    test_incdec_arith(value, true, value - value);
    test_incdec_arith(value, false, value + value);
    test_incdec_arith(value, false, value - value);
}

fn main() {
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith(1 << 60);
    test_all_incdec_arith(1.5);
    let mut ia = [0; 2];
    test_incdec(&ia[1], true, 1);
    test_incdec(&ia[1], true, -1);
    test_incdec(&ia[1], false, 1);
    test_incdec(&ia[1], false, -1);
    process::exit(0);
}