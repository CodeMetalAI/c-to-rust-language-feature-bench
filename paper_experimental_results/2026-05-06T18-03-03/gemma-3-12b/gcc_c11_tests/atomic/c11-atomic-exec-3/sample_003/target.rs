use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicF32, AtomicF64, AtomicPtr};

fn main() {
    test_incdec();
    std::process::exit(0);
}

fn test_incdec() {
    macro_rules! test_incdec {
        ($type:ty, $value:expr, $preop:ident, $postop:ident, $pre_p:expr, $change:expr) => {
            static a: $type = $value;
            unsafe {
                let a_atomic = std::sync::atomic::Atomic::$type::new(a);
                if a_atomic.compare_exchange($pre_p, $preop(a), $postop(a), std::sync::atomic::Ordering::SeqCst).is_err() {
                    panic!();
                }
                if a_atomic.load(std::sync::atomic::Ordering::SeqCst) != $change {
                    panic!();
                }
            }
        };
    }

    macro_rules! test_incdec_arith {
        ($value:expr, $preop:ident, $postop:ident, $pre_p:expr, $change:expr) => {
            test_incdec(AtomicBool, $value, $preop, $postop, $pre_p, $change);
            test_incdec(AtomicI8, $value, $preop, $postop, $pre_p, $change);
            test_incdec(AtomicI16, $value, $preop, $postop, $pre_p, $change);
            test_incdec(AtomicI32, $value, $preop, $postop, $pre_p, $change);
            test_incdec(AtomicI64, $value, $preop, $postop, $pre_p, $change);
            test_incdec(AtomicIsize, $value, $preop, $postop, $pre_p, $change);
            test_incdec(AtomicF32, $value, $preop, $postop, $pre_p, $change);
            test_incdec(AtomicF64, $value, $preop, $postop, $pre_p, $change);
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
    test_all_incdec_arith(1 << 60);
    test_all_incdec_arith(1.5);

    let mut ia: [u8; 2] = [0; 2];
    let ptr_ia = &ia[1] as *const u8 as *mut u8;

    test_incdec(AtomicPtr<u8>, ptr_ia, ++, , 1, 1);
    test_incdec(AtomicPtr<u8>, ptr_ia, --, , 1, -1);
    test_incdec(AtomicPtr<u8>, ptr_ia, , ++, 0, 1);
    test_incdec(AtomicPtr<u8>, ptr_ia, , --, 0, -1);
}