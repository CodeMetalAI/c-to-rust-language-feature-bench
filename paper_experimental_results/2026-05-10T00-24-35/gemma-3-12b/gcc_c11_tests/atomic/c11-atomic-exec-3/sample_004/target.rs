use std::sync::atomic::{AtomicBool, AtomicChar, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicF32, AtomicF64, AtomicIsize, AtomicUsize, AtomicPtr};

fn main() {
    test_incdec();
    std::process::exit(0);
}

fn test_incdec() {
    macro_rules! test_incdec {
        ($type:ty, $value:expr, $preop:ident, $postop:ident, $pre_p:expr, $change:expr) => {
            static a: $type = $type($value);
            unsafe {
                let a_atomic = $type::new(a);
                if $preop(a_atomic) $postop() != ($pre_p as bool as $type)((($value as $type) + $change) as $type) {
                    panic!();
                }
                if a_atomic.load(std::sync::atomic::Ordering::Relaxed) != (($value as $type) + $change) as $type {
                    panic!();
                }
            }
        };
    }

    macro_rules! test_incdec_arith {
        ($value:expr, $preop:ident, $postop:ident, $pre_p:expr, $change:expr) => {
            test_incdec(AtomicBool, $value, $preop, $postop, $pre_p, $change);
            test_incdec(AtomicChar, $value, $preop, $postop, $pre_p, $change);
            test_incdec(AtomicI8, $value, $preop, $postop, $pre_p, $change);
            test_incdec(AtomicU8, $value, $preop, $postop, $pre_p, $change);
            test_incdec(AtomicI16, $value, $preop, $postop, $pre_p, $change);
            test_incdec(AtomicU16, $value, $preop, $postop, $pre_p, $change);
            test_incdec(AtomicI32, $value, $preop, $postop, $pre_p, $change);
            test_incdec(AtomicU32, $value, $preop, $postop, $pre_p, $change);
            test_incdec(AtomicI64, $value, $preop, $postop, $pre_p, $change);
            test_incdec(AtomicU64, $value, $preop, $postop, $pre_p, $change);
            test_incdec(AtomicF32, $value, $preop, $postop, $pre_p, $change);
            test_incdec(AtomicF64, $value, $preop, $postop, $pre_p, $change);
            test_incdec(AtomicIsize, $value, $preop, $postop, $pre_p, $change);
            test_incdec(AtomicUsize, $value, $preop, $postop, $pre_p, $change);
            test_incdec(AtomicPtr<u8>, $value, $preop, $postop, $pre_p, $change);
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

    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith!(1 << 60);
    test_all_incdec_arith(1.5);

    let mut ia: [i32; 2] = [0; 2];
    let ptr: *mut i32 = &mut ia[1];

    test_incdec(AtomicPtr<u8>, ptr as *mut u8, ++, , 1, 1);
    test_incdec(AtomicPtr<u8>, ptr as *mut u8, --, , 1, -1);
    test_incdec(AtomicPtr<u8>, ptr as *mut u8, , ++, 0, 1);
    test_incdec(AtomicPtr<u8>, ptr as *mut u8, , --, 0, -1);
}