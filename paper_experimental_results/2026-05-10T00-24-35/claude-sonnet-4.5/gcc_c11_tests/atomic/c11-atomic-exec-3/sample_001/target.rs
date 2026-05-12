use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::process;

macro_rules! test_incdec {
    ($atomic_type:ty, $value_type:ty, $value:expr, $pre_op:tt, $post_op:tt, $pre_p:expr, $change:expr) => {{
        static A: $atomic_type = <$atomic_type>::new($value as $value_type);
        
        let result = test_incdec!(@op A, $pre_op, $post_op);
        let expected = if $pre_p {
            ($value as $value_type).wrapping_add($change as $value_type)
        } else {
            $value as $value_type
        };
        
        if result != expected {
            process::abort();
        }
        
        let final_val = A.load(Ordering::SeqCst);
        let expected_final = ($value as $value_type).wrapping_add($change as $value_type);
        if final_val != expected_final {
            process::abort();
        }
    }};
    
    (@op $a:expr, ++, ) => {
        $a.fetch_add(1, Ordering::SeqCst).wrapping_add(1)
    };
    (@op $a:expr, --, ) => {
        $a.fetch_sub(1, Ordering::SeqCst).wrapping_sub(1)
    };
    (@op $a:expr, , ++) => {
        $a.fetch_add(1, Ordering::SeqCst)
    };
    (@op $a:expr, , --) => {
        $a.fetch_sub(1, Ordering::SeqCst)
    };
}

macro_rules! test_incdec_arith {
    ($value:expr, $pre_op:tt, $post_op:tt, $pre_p:expr, $change:expr) => {{
        test_incdec!(AtomicBool, bool, $value, $pre_op, $post_op, $pre_p, $change);
        test_incdec!(AtomicI8, i8, $value, $pre_op, $post_op, $pre_p, $change);
        test_incdec!(AtomicI8, i8, $value, $pre_op, $post_op, $pre_p, $change);
        test_incdec!(AtomicU8, u8, $value, $pre_op, $post_op, $pre_p, $change);
        test_incdec!(AtomicI16, i16, $value, $pre_op, $post_op, $pre_p, $change);
        test_incdec!(AtomicU16, u16, $value, $pre_op, $post_op, $pre_p, $change);
        test_incdec!(AtomicI32, i32, $value, $pre_op, $post_op, $pre_p, $change);
        test_incdec!(AtomicU32, u32, $value, $pre_op, $post_op, $pre_p, $change);
        test_incdec!(AtomicIsize, isize, $value, $pre_op, $post_op, $pre_p, $change);
        test_incdec!(AtomicUsize, usize, $value, $pre_op, $post_op, $pre_p, $change);
        test_incdec!(AtomicI64, i64, $value, $pre_op, $post_op, $pre_p, $change);
        test_incdec!(AtomicU64, u64, $value, $pre_op, $post_op, $pre_p, $change);
    }};
}

macro_rules! test_all_incdec_arith {
    ($value:expr) => {{
        test_incdec_arith!($value, ++, , 1, 1);
        test_incdec_arith!($value, --, , 1, -1);
        test_incdec_arith!($value, , ++, 0, 1);
        test_incdec_arith!($value, , --, 0, -1);
    }};
}

fn test_incdec() {
    test_all_incdec_arith!(0);
    test_all_incdec_arith!(1);
    test_all_incdec_arith!(2);
    test_all_incdec_arith!(-1);
    test_all_incdec_arith!(1u64 << 60);
}

fn main() {
    test_incdec();
    process::exit(0);
}