use std::sync::atomic::{AtomicUsize, Ordering};

macro_rules! test_incdec {
    ($value:expr, $preop:expr, $postop:expr, $pre_p:expr, $change:expr) => {{
        let a = std::sync::atomic::AtomicUsize::new($value);
        let result = if $preop {
            a.fetch_add($change, Ordering::SeqCst)
        } else {
            a.fetch_sub($change, Ordering::SeqCst)
        };
        assert_eq!(result, if $pre_p { $value } else { $value - $change });
        assert_eq!(a.load(Ordering::SeqCst), $value + $change);
    }};
}

macro_rules! test_incdec_arith {
    ($value:expr) => {{
        test_incdec!($value, true, false, true, 1);
        test_incdec!($value, true, false, true, -1);
        test_incdec!($value, false, true, false, 1);
        test_incdec!($value, false, true, false, -1);
    }};
}

fn test_incdec() {
    test_incdec_arith!(0);
    test_incdec_arith!(1);
    test_incdec_arith!(2);
    test_incdec_arith!(-1);
    test_incdec_arith!(1 << 60);
    test_incdec_arith!(1.5 as usize);
    let ia = [0, 0];
    let ia_ptr = &ia[1] as *const _ as usize;
    test_incdec!(ia_ptr, true, false, true, 1);
    test_incdec!(ia_ptr, true, false, true, -1);
    test_incdec!(ia_ptr, false, true, false, 1);
    test_incdec!(ia_ptr, false, true, false, -1);
}

fn main() {
    test_incdec();
}