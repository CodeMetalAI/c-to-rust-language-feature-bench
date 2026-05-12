use std::process::exit;

fn abort() {
    exit(1);
}

macro_rules! test_incdec {
    ($type:ty, $value:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {
        {
            static mut A: std::sync::atomic::Atomic<$type> = std::sync::atomic::Atomic::new($value);
            let a = &mut A;
            let result = match ($preop, $postop) {
                ("++", "") => a.fetch_add($change, std::sync::atomic::Ordering::SeqCst) + $change,
                ("--", "") => a.fetch_sub($change, std::sync::atomic::Ordering::SeqCst) - $change,
                ("", "++") => a.fetch_add($change, std::sync::atomic::Ordering::SeqCst),
                ("", "--") => a.fetch_sub($change, std::sync::atomic::Ordering::SeqCst),
                _ => unreachable!(),
            };
            let expected_pre = if $pre_p {
                $value + $change
            } else {
                $value
            };
            if result != expected_pre {
                abort();
            }
            let current = a.load(std::sync::atomic::Ordering::SeqCst);
            if current != $value + $change {
                abort();
            }
        }
    };
}

macro_rules! test_incdec_arith {
    ($value:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {
        test_incdec!(bool, $value != 0, $preop, $postop, $pre_p, $change != 0);
        test_incdec!(i8, $value as i8, $preop, $postop, $pre_p, $change as i8);
        test_incdec!(u8, $value as u8, $preop, $postop, $pre_p, $change as u8);
        test_incdec!(i16, $value as i16, $preop, $postop, $pre_p, $change as i16);
        test_incdec!(u16, $value as u16, $preop, $postop, $pre_p, $change as u16);
        test_incdec!(i32, $value as i32, $preop, $postop, $pre_p, $change as i32);
        test_incdec!(u32, $value as u32, $preop, $postop, $pre_p, $change as u32);
        test_incdec!(i64, $value as i64, $preop, $postop, $pre_p, $change as i64);
        test_incdec!(u64, $value as u64, $preop, $postop, $pre_p, $change as u64);
        test_incdec!(f32, $value as f32, $preop, $postop, $pre_p, $change as f32);
        test_incdec!(f64, $value as f64, $preop, $postop, $pre_p, $change as f64);
    };
}

macro_rules! test_all_incdec_arith {
    ($value:expr) => {
        test_incdec_arith!($value, "++", "", true, 1);
        test_incdec_arith!($value, "--", "", true, -1);
        test_incdec_arith!($value, "", "++", false, 1);
        test_incdec_arith!($value, "", "--", false, -1);
    };
}

fn test_incdec() {
    test_all_incdec_arith!(0);
    test_all_incdec_arith!(1);
    test_all_incdec_arith!(2);
    test_all_incdec_arith!(-1);
    test_all_incdec_arith!(1 << 60);
    test_all_incdec_arith!(1.5);
    
    static mut IA: [i32; 2] = [0, 0];
    let ia_ptr = &mut IA[1] as *mut i32;
    test_incdec!(*mut i32, ia_ptr, "++", "", true, 1);
    test_incdec!(*mut i32, ia_ptr, "--", "", true, -1);
    test_incdec!(*mut i32, ia_ptr, "", "++", false, 1);
    test_incdec!(*mut i32, ia_ptr, "", "--", false, -1);
}

fn main() {
    test_incdec();
    exit(0);
}