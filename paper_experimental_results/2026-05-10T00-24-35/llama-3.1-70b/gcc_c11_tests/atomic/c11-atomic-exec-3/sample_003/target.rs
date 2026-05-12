fn test_incdec<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy + PartialEq>(value: T, change: T, pre_op: bool) {
    let mut a = std::sync::atomic::Atomic::new(value);
    let pre_value = if pre_op {
        let prev = a.fetch_add(change, std::sync::atomic::Ordering::SeqCst);
        prev + change
    } else {
        a.fetch_add(change, std::sync::atomic::Ordering::SeqCst)
    };
    if pre_value!= (value + change) {
        std::process::abort();
    }
    if a.load(std::sync::atomic::Ordering::SeqCst)!= (value + change) {
        std::process::abort();
    }
}

fn test_incdec_arith(value: i64) {
    test_incdec::<i8>(value as i8, 1, true);
    test_incdec::<i8>(value as i8, -1, true);
    test_incdec::<i8>(value as i8, 1, false);
    test_incdec::<i8>(value as i8, -1, false);
    test_incdec::<u8>(value as u8, 1, true);
    test_incdec::<u8>(value as u8, -1, true);
    test_incdec::<u8>(value as u8, 1, false);
    test_incdec::<u8>(value as u8, -1, false);
    test_incdec::<i16>(value as i16, 1, true);
    test_incdec::<i16>(value as i16, -1, true);
    test_incdec::<i16>(value as i16, 1, false);
    test_incdec::<i16>(value as i16, -1, false);
    test_incdec::<u16>(value as u16, 1, true);
    test_incdec::<u16>(value as u16, -1, true);
    test_incdec::<u16>(value as u16, 1, false);
    test_incdec::<u16>(value as u16, -1, false);
    test_incdec::<i32>(value as i32, 1, true);
    test_incdec::<i32>(value as i32, -1, true);
    test_incdec::<i32>(value as i32, 1, false);
    test_incdec::<i32>(value as i32, -1, false);
    test_incdec::<u32>(value as u32, 1, true);
    test_incdec::<u32>(value as u32, -1, true);
    test_incdec::<u32>(value as u32, 1, false);
    test_incdec::<u32>(value as u32, -1, false);
    test_incdec::<i64>(value, 1, true);
    test_incdec::<i64>(value, -1, true);
    test_incdec::<i64>(value, 1, false);
    test_incdec::<i64>(value, -1, false);
    test_incdec::<u64>(value as u64, 1, true);
    test_incdec::<u64>(value as u64, -1, true);
    test_incdec::<u64>(value as u64, 1, false);
    test_incdec::<u64>(value as u64, -1, false);
    test_incdec::<f64>(value as f64, 1.0, true);
    test_incdec::<f64>(value as f64, -1.0, true);
    test_incdec::<f64>(value as f64, 1.0, false);
    test_incdec::<f64>(value as f64, -1.0, false);
}

fn test_incdec_ptr() {
    let mut ia = [0, 0];
    let mut a = std::sync::atomic::Atomic::new(&ia[1]);
    let pre_value = a.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    if pre_value!= &ia[1] {
        std::process::abort();
    }
    if a.load(std::sync::atomic::Ordering::SeqCst)!= &ia[2] {
        std::process::abort();
    }
    a = std::sync::atomic::Atomic::new(&ia[1]);
    let pre_value = a.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
    if pre_value!= &ia[1] {
        std::process::abort();
    }
    if a.load(std::sync::atomic::Ordering::SeqCst)!= &ia[0] {
        std::process::abort();
    }
    a = std::sync::atomic::Atomic::new(&ia[1]);
    let pre_value = a.load(std::sync::atomic::Ordering::SeqCst);
    a.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    if pre_value!= &ia[1] {
        std::process::abort();
    }
    if a.load(std::sync::atomic::Ordering::SeqCst)!= &ia[2] {
        std::process::abort();
    }
    a = std::sync::atomic::Atomic::new(&ia[1]);
    let pre_value = a.load(std::sync::atomic::Ordering::SeqCst);
    a.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
    if pre_value!= &ia[1] {
        std::process::abort();
    }
    if a.load(std::sync::atomic::Ordering::SeqCst)!= &ia[0] {
        std::process::abort();
    }
}

fn main() {
    test_incdec_arith(0);
    test_incdec_arith(1);
    test_incdec_arith(2);
    test_incdec_arith(-1);
    test_incdec_arith(1 << 60);
    test_incdec_arith(1.5 as i64);
    test_incdec_ptr();
    std::process::exit(0);
}