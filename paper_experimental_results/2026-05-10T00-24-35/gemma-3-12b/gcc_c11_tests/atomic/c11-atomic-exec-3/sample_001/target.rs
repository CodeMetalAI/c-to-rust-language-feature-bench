use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicF32, AtomicF64, AtomicPtr};

fn main() {
    test_incdec();
    std::process::exit(0);
}

macro_rules! test_incdec {
    ($a:ty, $value:expr, $preop:ident, $postop:ident, $pre_p:expr, $change:expr) => {
        {
            static a: std::sync::atomic::Atomic<$a> = std::sync::atomic::Atomic::new($value);
            let expected = if $pre_p {
                ($value as $a + $change) as $a
            } else {
                $value as $a
            };

            if a.compare_exchange($value, expected, std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst).is_err() {
                panic!("compare_exchange failed");
            }

            if a.load(std::sync::atomic::Ordering::SeqCst) != expected {
                panic!("load failed");
            }
        }
    };
}

macro_rules! test_incdec_arith {
    ($value:expr, $preop:ident, $postop:ident, $pre_p:expr, $change:expr) => {
        test_incdec(bool, $value, $preop, $postop, $pre_p, $change);
        test_incdec(i8, $value, $preop, $postop, $pre_p, $change);
        test_incdec(i16, $value, $preop, $postop, $pre_p, $change);
        test_incdec(i32, $value, $preop, $postop, $pre_p, $change);
        test_incdec(i64, $value, $preop, $postop, $pre_p, $change);
        test_incdec(isize, $value, $preop, $postop, $pre_p, $change);
        test_incdec(u8, $value, $preop, $postop, $pre_p, $change);
        test_incdec(u16, $value, $preop, $postop, $pre_p, $change);
        test_incdec(u32, $value, $preop, $postop, $pre_p, $change);
        test_incdec(u64, $value, $preop, $postop, $pre_p, $change);
        test_incdec(usize, $value, $preop, $postop, $pre_p, $change);
        test_incdec(f32, $value, $preop, $postop, $pre_p, $change);
        test_incdec(f64, $value, $preop, $postop, $pre_p, $change);
    };
}

macro_rules! test_all_incdec_arith {
    ($value:expr) => {
        test_incdec_arith($value, ++, , 1, 1);
        test_incdec_arith($value, --, , 1, -1);
        test_incdec_arith($value, , ++, 0, 1);
        test_incdec_arith($value, , --, 0, -1);
    };
}

fn test_incdec() {
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith!(1 << 60);
    test_all_incdec_arith(1.5);

    let mut ia[2];
    test_incdec( *const i32, &ia[1] as *const i32, ++, , 1, 1);
    test_incdec( *const i32, &ia[1] as *const i32, --, , 1, -1);
    test_incdec( *const i32, &ia[1] as *const i32, , ++, 0, 1);
    test_incdec( *const i32, &ia[1] as *const i32, , --, 0, -1);
}