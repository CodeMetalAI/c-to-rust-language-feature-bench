use std::process::exit;

fn abort() {
    exit(1);
}

fn test_incdec<T: Copy + PartialEq + std::ops::Add<Output = T> + std::ops::Sub<Output = T>>(
    value: T,
    change: T,
    pre_p: bool,
) {
    let mut a = std::sync::atomic::AtomicPtr::<T>::new(std::ptr::null_mut());
    let boxed = Box::new(value);
    a.store(Box::into_raw(boxed), std::sync::atomic::Ordering::SeqCst);

    let ptr = a.load(std::sync::atomic::Ordering::SeqCst);
    let deref = unsafe { *ptr };
    let expected_result = if pre_p {
        value + change
    } else {
        value
    };
    if deref != expected_result {
        abort();
    }

    let new_ptr = a.load(std::sync::atomic::Ordering::SeqCst);
    let new_deref = unsafe { *new_ptr };
    if new_deref != value + change {
        abort();
    }

    // Clean up the heap allocation
    unsafe {
        drop(Box::from_raw(ptr));
    }
}

fn test_incdec_arith(value: i32, change: i32, pre_p: bool) {
    test_incdec(value as bool, change as bool, pre_p);
    test_incdec(value as i8, change as i8, pre_p);
    test_incdec(value as u8, change as u8, pre_p);
    test_incdec(value as i16, change as i16, pre_p);
    test_incdec(value as u16, change as u16, pre_p);
    test_incdec(value as i32, change as i32, pre_p);
    test_incdec(value as u32, change as u32, pre_p);
    test_incdec(value as i64, change as i64, pre_p);
    test_incdec(value as u64, change as u64, pre_p);
    test_incdec(value as f32, change as f32, pre_p);
    test_incdec(value as f64, change as f64, pre_p);
}

fn test_all_incdec_arith(value: i32) {
    test_incdec_arith(value, 1, true);
    test_incdec_arith(value, -1, true);
    test_incdec_arith(value, 1, false);
    test_incdec_arith(value, -1, false);
}

fn main() {
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(-1);
    test_all_incdec_arith(1 << 30);
    test_all_incdec_arith(1);

    // Pointer tests
    let mut ia = [0, 0];
    test_incdec(&ia[1] as *const i32, 1, true);
    test_incdec(&ia[1] as *const i32, -1, true);
    test_incdec(&ia[1] as *const i32, 1, false);
    test_incdec(&ia[1] as *const i32, -1, false);

    exit(0);
}