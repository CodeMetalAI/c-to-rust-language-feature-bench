use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicF32, AtomicF64, AtomicPtr};

fn main() {
    test_incdec();
    std::process::exit(0);
}

fn test_incdec() {
    TEST_ALL_INCDEC_ARITH(0);
    TEST_ALL_INCDEC_ARITH(1);
    TEST_ALL_INCDEC_ARITH(2);
    TEST_ALL_INCDEC_ARITH(-1);
    TEST_ALL_INCDEC_ARITH(1 << 60);
    TEST_ALL_INCDEC_ARITH(1.5);

    let mut ia = [0; 2];
    unsafe {
        TEST_INCDEC(AtomicI32, &ia[1] as *mut i32, ++, , 1, 1);
        TEST_INCDEC(AtomicI32, &ia[1] as *mut i32, --, , 1, -1);
        TEST_INCDEC(AtomicI32, &ia[1] as *mut i32, , ++, 0, 1);
        TEST_INCDEC(AtomicI32, &ia[1] as *mut i32, , --, 0, -1);
    }
}

macro_rules! TEST_INCDEC {
    ($TYPE:ty, $addr:expr, $PREOP:expr, $POSTOP:expr, $PRE_P:expr, $CHANGE:expr) => {
        unsafe {
            let mut a = $TYPE::new($addr);
            if $PREOP a $POSTOP != (if $PRE_P { (*a as i64) + $CHANGE as i64 } else { *a as i64 }) {
                panic!("PREOP/POSTOP check failed");
            }
            if a.load(std::sync::atomic::Ordering::Relaxed) != (if $PRE_P { (*a as i64) + $CHANGE as i64 } else { *a as i64 }) {
                panic!("Load check failed");
            }
        }
    };
}

macro_rules! TEST_INCDEC_ARITH {
    ($VALUE:expr, $PREOP:expr, $POSTOP:expr, $PRE_P:expr, $CHANGE:expr) => {
        TEST_INCDEC(AtomicBool, $VALUE as *mut bool, $PREOP, $POSTOP, $PRE_P, $CHANGE);
        TEST_INCDEC(AtomicI8, $VALUE as *mut i8, $PREOP, $POSTOP, $PRE_P, $CHANGE);
        TEST_INCDEC(AtomicI16, $VALUE as *mut i16, $PREOP, $POSTOP, $PRE_P, $CHANGE);
        TEST_INCDEC(AtomicI32, $VALUE as *mut i32, $PREOP, $POSTOP, $PRE_P, $CHANGE);
        TEST_INCDEC(AtomicI64, $VALUE as *mut i64, $PREOP, $POSTOP, $PRE_P, $CHANGE);
        TEST_INCDEC(AtomicIsize, $VALUE as *mut isize, $PREOP, $POSTOP, $PRE_P, $CHANGE);
        TEST_INCDEC(AtomicF32, $VALUE as *mut f32, $PREOP, $POSTOP, $PRE_P, $CHANGE);
        TEST_INCDEC(AtomicF64, $VALUE as *mut f64, $PREOP, $POSTOP, $PRE_P, $CHANGE);
    };
}

macro_rules! TEST_ALL_INCDEC_ARITH {
    ($VALUE:expr) => {
        TEST_INCDEC_ARITH($VALUE, ++, , 1, 1);
        TEST_INCDEC_ARITH($VALUE, --, , 1, -1);
        TEST_INCDEC_ARITH($VALUE, , ++, 0, 1);
        TEST_INCDEC_ARITH($VALUE, , --, 0, -1);
    };
}