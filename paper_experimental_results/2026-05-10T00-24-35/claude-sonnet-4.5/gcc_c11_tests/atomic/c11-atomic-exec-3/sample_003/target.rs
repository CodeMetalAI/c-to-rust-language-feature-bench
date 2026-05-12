use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::process;

macro_rules! test_incdec {
    ($atomic_type:ty, $rust_type:ty, $value:expr, $pre_op:tt, $post_op:tt, $pre_p:expr, $change:expr) => {{
        static A: $atomic_type = <$atomic_type>::new($value as $rust_type);
        
        let result = test_incdec!(@operation A, $pre_op, $post_op, $change);
        let expected = if $pre_p {
            ($value as $rust_type).wrapping_add($change as $rust_type)
        } else {
            $value as $rust_type
        };
        
        if result != expected {
            process::abort();
        }
        
        if A.load(Ordering::SeqCst) != ($value as $rust_type).wrapping_add($change as $rust_type) {
            process::abort();
        }
    }};
    
    (@operation $a:ident, ++, , $change:expr) => {
        $a.fetch_add($change, Ordering::SeqCst).wrapping_add($change)
    };
    (@operation $a:ident, --, , $change:expr) => {
        $a.fetch_sub(-$change, Ordering::SeqCst).wrapping_sub(-$change)
    };
    (@operation $a:ident, , ++, $change:expr) => {
        $a.fetch_add($change, Ordering::SeqCst)
    };
    (@operation $a:ident, , --, $change:expr) => {
        $a.fetch_sub(-$change, Ordering::SeqCst)
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