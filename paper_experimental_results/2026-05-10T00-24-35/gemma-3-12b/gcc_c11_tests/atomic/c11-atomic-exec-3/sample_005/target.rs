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
        TEST_INCDEC(AtomicI32, &ia[1], ++, , 1, 1);
        TEST_INCDEC(AtomicI32, &ia[1], --, , 1, -1);
        TEST_INCDEC(AtomicI32, &ia[1], , ++, 0, 1);
        TEST_INCDEC(AtomicI32, &ia[1], , --, 0, -1);
    }
}

macro_rules! TEST_INCDEC {
    ($TYPE:ty, $VALUE:expr, $PREOP:ident, $POSTOP:ident, $PRE_P:tt, $CHANGE:expr) => {
        let mut a = $TYPE::new($VALUE);
        if $PREOP a $POSTOP != ($PRE_P == 1 ? $TYPE::new($VALUE as i64 + $CHANGE as i64) : $TYPE::new($VALUE as i64)) {
            panic!()
        }
        if a != $TYPE::new(($VALUE as i64 + $CHANGE as i64) as i64) {
            panic!()
        }
    };
}

macro_rules! TEST_INCDEC_ARITH {
    ($VALUE:expr, $PREOP:ident, $POSTOP:ident, $PRE_P:tt, $CHANGE:expr) => {
        TEST_INCDEC(AtomicBool, $VALUE, $PREOP, $POSTOP, $PRE_P, $CHANGE);
        TEST_INCDEC(AtomicI8, $VALUE, $PREOP, $POSTOP, $PRE_P, $CHANGE);
        TEST_INCDEC(AtomicI16, $VALUE, $PREOP, $POSTOP, $PRE_P, $CHANGE);
        TEST_INCDEC(AtomicI32, $VALUE, $PREOP, $POSTOP, $PRE_P, $CHANGE);
        TEST_INCDEC(AtomicI64, $VALUE, $PREOP, $POSTOP, $PRE_P, $CHANGE);
        TEST_INCDEC(AtomicIsize, $VALUE, $PREOP, $POSTOP, $PRE_P, $CHANGE);
        TEST_INCDEC(AtomicF32, $VALUE, $PREOP, $POSTOP, $PRE_P, $CHANGE);
        TEST_INCDEC(AtomicF64, $VALUE, $PREOP, $POSTOP, $PRE_P, $CHANGE);
        TEST_INCDEC(AtomicPtr<u8>, $VALUE, $PREOP, $POSTOP, $PRE_P, $CHANGE);
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