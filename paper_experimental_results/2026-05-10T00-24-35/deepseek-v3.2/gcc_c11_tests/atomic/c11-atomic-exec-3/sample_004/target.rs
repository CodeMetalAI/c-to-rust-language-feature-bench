use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicIsize, AtomicUsize, AtomicI64, AtomicU64};
use std::sync::atomic::{AtomicPtr, Ordering};
use std::process;

fn abort() {
    process::exit(1);
}

macro_rules! test_incdec {
    ($atomic_type:ty, $value:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {{
        let mut a = <$atomic_type>::new($value);
        let result = if stringify!($preop) == "++" && stringify!($postop) == "" {
            // ++a
            a.fetch_add(1, Ordering::SeqCst);
            a.load(Ordering::SeqCst)
        } else if stringify!($preop) == "--" && stringify!($postop) == "" {
            // --a
            a.fetch_sub(1, Ordering::SeqCst);
            a.load(Ordering::SeqCst)
        } else if stringify!($preop) == "" && stringify!($postop) == "++" {
            // a++
            let old = a.load(Ordering::SeqCst);
            a.fetch_add(1, Ordering::SeqCst);
            old
        } else if stringify!($preop) == "" && stringify!($postop) == "--" {
            // a--
            let old = a.load(Ordering::SeqCst);
            a.fetch_sub(1, Ordering::SeqCst);
            old
        } else {
            unreachable!()
        };
        
        let expected_result = if $pre_p {
            $value.wrapping_add($change)
        } else {
            $value
        };
        
        let expected_final = $value.wrapping_add($change);
        
        if result != expected_result || a.load(Ordering::SeqCst) != expected_final {
            abort();
        }
    }};
}

macro_rules! test_incdec_arith {
    ($value:expr, $preop:tt, $postop:tt, $pre_p:expr, $change:expr) => {
        test_incdec!(AtomicBool, $value != 0, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicI8, $value as i8, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicU8, $value as u8, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicI16, $value as i16, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicU16, $value as u16, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicI32, $value as i32, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicU32, $value as u32, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicIsize, $value as isize, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicUsize, $value as usize, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicI64, $value as i64, $preop, $postop, $pre_p, $change);
        test_incdec!(AtomicU64, $value as u64, $preop, $postop, $pre_p, $change);
    };
}

macro_rules! test_all_incdec_arith {
    ($value:expr) => {
        test_incdec_arith!($value, ++, , true, 1);
        test_incdec_arith!($value, --, , true, -1);
        test_incdec_arith!($value, , ++, false, 1);
        test_incdec_arith!($value, , --, false, -1);
    };
}

fn test_incdec() {
    test_all_incdec_arith!(0);
    test_all_incdec_arith!(1);
    test_all_incdec_arith!(2);
    test_all_incdec_arith!(-1);
    test_all_incdec_arith!(1u64 << 60);
    
    // Test pointer operations
    let mut arr = [0i32, 1i32];
    let base_ptr = &arr[1] as *const i32;
    
    {
        let ptr = AtomicPtr::new(base_ptr as *mut i32);
        let result = ptr.fetch_add(1, Ordering::SeqCst);
        if result != base_ptr as *mut i32 || ptr.load(Ordering::SeqCst) != base_ptr.wrapping_add(1) as *mut i32 {
            abort();
        }
    }
    
    {
        let ptr = AtomicPtr::new(base_ptr as *mut i32);
        let result = ptr.fetch_sub(1, Ordering::SeqCst);
        if result != base_ptr as *mut i32 || ptr.load(Ordering::SeqCst) != base_ptr.wrapping_sub(1) as *mut i32 {
            abort();
        }
    }
    
    {
        let ptr = AtomicPtr::new(base_ptr as *mut i32);
        let old = ptr.load(Ordering::SeqCst);
        ptr.fetch_add(1, Ordering::SeqCst);
        if old != base_ptr as *mut i32 || ptr.load(Ordering::SeqCst) != base_ptr.wrapping_add(1) as *mut i32 {
            abort();
        }
    }
    
    {
        let ptr = AtomicPtr::new(base_ptr as *mut i32);
        let old = ptr.load(Ordering::SeqCst);
        ptr.fetch_sub(1, Ordering::SeqCst);
        if old != base_ptr as *mut i32 || ptr.load(Ordering::SeqCst) != base_ptr.wrapping_sub(1) as *mut i32 {
            abort();
        }
    }
}

fn main() {
    test_incdec();
}