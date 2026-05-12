use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize, AtomicPtr};
use std::sync::atomic::Ordering::SeqCst;
use std::process::exit;

macro_rules! test_incdec {
    ($atomic_type:ty, $value:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {{
        let mut a = <$atomic_type>::new($value);
        let a_ref = &mut a;
        
        let result = match ($preop, $postop) {
            ("++", "") => {
                let old = a_ref.fetch_add($change, SeqCst);
                old + $change
            },
            ("--", "") => {
                let old = a_ref.fetch_sub(-$change, SeqCst);
                old - $change
            },
            ("", "++") => {
                let old = a_ref.fetch_add($change, SeqCst);
                old
            },
            ("", "--") => {
                let old = a_ref.fetch_sub(-$change, SeqCst);
                old
            },
            _ => unreachable!(),
        };
        
        let expected_result = if $pre_p {
            $value + $change
        } else {
            $value
        };
        
        if result != expected_result {
            exit(1);
        }
        
        let final_value = a_ref.load(SeqCst);
        if final_value != $value + $change {
            exit(1);
        }
    }};
}

macro_rules! test_incdec_arith {
    ($value:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {{
        test_incdec!(AtomicBool, $value != 0, $preop, $postop, $pre_p, $change as i32);
        test_incdec!(AtomicI8, $value as i8, $preop, $postop, $pre_p, $change as i8);
        test_incdec!(AtomicU8, $value as u8, $preop, $postop, $pre_p, $change as u8);
        test_incdec!(AtomicI16, $value as i16, $preop, $postop, $pre_p, $change as i16);
        test_incdec!(AtomicU16, $value as u16, $preop, $postop, $pre_p, $change as u16);
        test_incdec!(AtomicI32, $value as i32, $preop, $postop, $pre_p, $change as i32);
        test_incdec!(AtomicU32, $value as u32, $preop, $postop, $pre_p, $change as u32);
        test_incdec!(AtomicI64, $value as i64, $preop, $postop, $pre_p, $change as i64);
        test_incdec!(AtomicU64, $value as u64, $preop, $postop, $pre_p, $change as u64);
        test_incdec!(AtomicIsize, $value as isize, $preop, $postop, $pre_p, $change as isize);
        test_incdec!(AtomicUsize, $value as usize, $preop, $postop, $pre_p, $change as usize);
    }};
}

macro_rules! test_all_incdec_arith {
    ($value:expr) => {{
        test_incdec_arith!($value, "++", "", true, 1);
        test_incdec_arith!($value, "--", "", true, -1);
        test_incdec_arith!($value, "", "++", false, 1);
        test_incdec_arith!($value, "", "--", false, -1);
    }};
}

fn test_incdec() {
    test_all_incdec_arith!(0);
    test_all_incdec_arith!(1);
    test_all_incdec_arith!(2);
    test_all_incdec_arith!(-1);
    test_all_incdec_arith!(1u64 << 60);
    
    let ia = [0i32, 0i32];
    let ptr_value = &ia[1] as *const i32;
    
    test_incdec!(AtomicPtr<i32>, ptr_value, "++", "", true, 1);
    test_incdec!(AtomicPtr<i32>, ptr_value, "--", "", true, -1);
    test_incdec!(AtomicPtr<i32>, ptr_value, "", "++", false, 1);
    test_incdec!(AtomicPtr<i32>, ptr_value, "", "--", false, -1);
}

fn main() {
    test_incdec();
    exit(0);
}