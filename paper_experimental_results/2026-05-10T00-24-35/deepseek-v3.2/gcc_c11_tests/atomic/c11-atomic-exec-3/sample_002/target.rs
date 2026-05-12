use std::process;

fn abort() {
    process::exit(1);
}

fn test_incdec<T: Copy + PartialEq + std::fmt::Debug>(
    value: T,
    preop: bool,
    postop: bool,
    pre_p: bool,
    change: T,
) where
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
{
    let mut a = std::sync::atomic::AtomicPtr::new(&value);
    let original = value;
    let expected_value = if pre_p {
        original + change
    } else {
        original
    };
    
    let result = if preop {
        // Pre-increment or pre-decrement
        let new_value = original + change;
        let ptr = &new_value as *const T as *mut T;
        a.store(ptr, std::sync::atomic::Ordering::SeqCst);
        new_value
    } else if postop {
        // Post-increment or post-decrement
        let new_value = original + change;
        let ptr = &new_value as *const T as *mut T;
        a.store(ptr, std::sync::atomic::Ordering::SeqCst);
        original
    } else {
        original
    };
    
    if result != expected_value {
        abort();
    }
    
    let stored = unsafe { *a.load(std::sync::atomic::Ordering::SeqCst) };
    if stored != original + change {
        abort();
    }
}

fn test_incdec_arith(value: i32, preop: bool, postop: bool, pre_p: bool, change: i32) {
    test_incdec(value != 0, preop, postop, pre_p, change != 0);
    test_incdec(value as i8, preop, postop, pre_p, change as i8);
    test_incdec(value as u8, preop, postop, pre_p, change as u8);
    test_incdec(value as i16, preop, postop, pre_p, change as i16);
    test_incdec(value as u16, preop, postop, pre_p, change as u16);
    test_incdec(value, preop, postop, pre_p, change);
    test_incdec(value as u32, preop, postop, pre_p, change as u32);
    test_incdec(value as i64, preop, postop, pre_p, change as i64);
    test_incdec(value as u64, preop, postop, pre_p, change as u64);
    test_incdec(value as f32, preop, postop, pre_p, change as f32);
    test_incdec(value as f64, preop, postop, pre_p, change as f64);
}

fn test_all_incdec_arith(value: i32) {
    // ++a (pre-increment)
    test_incdec_arith(value, true, false, true, 1);
    // --a (pre-decrement)
    test_incdec_arith(value, true, false, true, -1);
    // a++ (post-increment)
    test_incdec_arith(value, false, true, false, 1);
    // a-- (post-decrement)
    test_incdec_arith(value, false, true, false, -1);
}

fn main() {
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    
    // Test with pointer-like operations
    let mut ia = [0i32; 2];
    let ptr = &mut ia[1] as *mut i32;
    
    // Test pointer increments/decrements
    let mut ptr_val = ptr;
    unsafe {
        // ++ptr
        ptr_val = ptr_val.add(1);
        if ptr_val != ptr.add(1) {
            abort();
        }
        
        // --ptr
        ptr_val = ptr_val.sub(1);
        if ptr_val != ptr {
            abort();
        }
        
        // ptr++
        let old = ptr_val;
        ptr_val = ptr_val.add(1);
        if old != ptr {
            abort();
        }
        
        // ptr--
        let old = ptr_val;
        ptr_val = ptr_val.sub(1);
        if old != ptr.add(1) {
            abort();
        }
    }
    
    process::exit(0);
}