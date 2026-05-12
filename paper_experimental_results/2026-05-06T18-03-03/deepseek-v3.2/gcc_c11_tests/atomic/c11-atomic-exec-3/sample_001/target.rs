use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize, AtomicPtr};
use std::sync::atomic::Ordering::SeqCst;

fn abort() {
    std::process::exit(1);
}

macro_rules! test_incdec {
    ($atomic_type:ty, $value:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {
        {
            let mut a = <$atomic_type>::new($value);
            let result = match ($preop, $postop) {
                ("++", "") => a.fetch_add(1, SeqCst) + 1,
                ("--", "") => a.fetch_sub(1, SeqCst) - 1,
                ("", "++") => {
                    let old = a.load(SeqCst);
                    a.store(old + 1, SeqCst);
                    old
                },
                ("", "--") => {
                    let old = a.load(SeqCst);
                    a.store(old - 1, SeqCst);
                    old
                },
                _ => unreachable!(),
            };
            
            let expected_result = if $pre_p {
                $value + $change
            } else {
                $value
            };
            
            let expected_final = $value + $change;
            
            if result != expected_result {
                abort();
            }
            if a.load(SeqCst) != expected_final {
                abort();
            }
        }
    };
}

macro_rules! test_incdec_arith {
    ($value:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {
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
    test_all_incdec_arith!(1u64 << 60);
    
    // Note: Floating point atomics aren't available in Rust's standard library
    // We'll skip the float/double/long double tests as they require unsafe or external crates
    
    let ia = [0, 0];
    let base_ptr = &ia[1] as *const i32;
    
    // Test pointer arithmetic
    {
        let mut a = AtomicPtr::new(base_ptr as *mut i32);
        let result = a.fetch_add(1, SeqCst);
        if result != base_ptr as *mut i32 {
            abort();
        }
        if a.load(SeqCst) != (base_ptr as *mut i32).wrapping_add(1) {
            abort();
        }
    }
    
    {
        let mut a = AtomicPtr::new(base_ptr as *mut i32);
        let result = a.fetch_sub(1, SeqCst);
        if result != base_ptr as *mut i32 {
            abort();
        }
        if a.load(SeqCst) != (base_ptr as *mut i32).wrapping_sub(1) {
            abort();
        }
    }
    
    {
        let mut a = AtomicPtr::new(base_ptr as *mut i32);
        let old = a.load(SeqCst);
        a.store(old.wrapping_add(1), SeqCst);
        if old != base_ptr as *mut i32 {
            abort();
        }
        if a.load(SeqCst) != (base_ptr as *mut i32).wrapping_add(1) {
            abort();
        }
    }
    
    {
        let mut a = AtomicPtr::new(base_ptr as *mut i32);
        let old = a.load(SeqCst);
        a.store(old.wrapping_sub(1), SeqCst);
        if old != base_ptr as *mut i32 {
            abort();
        }
        if a.load(SeqCst) != (base_ptr as *mut i32).wrapping_sub(1) {
            abort();
        }
    }
}

fn main() {
    test_incdec();
    std::process::exit(0);
}