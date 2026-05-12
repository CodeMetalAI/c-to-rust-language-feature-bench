use std::sync::atomic::{AtomicUsize, Ordering};

fn test_incdec<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + PartialEq + Copy>(value: T) {
    let a = std::sync::atomic::AtomicUsize::new(value as usize);
    let b = std::sync::atomic::AtomicUsize::new(value as usize);

    assert_eq!(a.fetch_add(1, Ordering::SeqCst) as T, value);
    assert_eq!(a.load(Ordering::SeqCst) as T, value + T::from(1).unwrap());
    assert_eq!(b.fetch_sub(1, Ordering::SeqCst) as T, value);
    assert_eq!(b.load(Ordering::SeqCst) as T, value - T::from(1).unwrap());
    assert_eq!(a.fetch_add(0, Ordering::SeqCst) as T, value + T::from(1).unwrap());
    assert_eq!(b.fetch_sub(0, Ordering::SeqCst) as T, value - T::from(1).unwrap());
}

fn test_incdec_arith(value: usize) {
    test_incdec::<bool>(value!= 0);
    test_incdec::<u8>(value as u8);
    test_incdec::<i8>(value as i8);
    test_incdec::<u16>(value as u16);
    test_incdec::<i16>(value as i16);
    test_incdec::<u32>(value as u32);
    test_incdec::<i32>(value as i32);
    test_incdec::<u64>(value as u64);
    test_incdec::<i64>(value as i64);
    test_incdec::<f32>(value as f32);
    test_incdec::<f64>(value as f64);
}

fn test_all_incdec_arith(value: usize) {
    test_incdec_arith(value);
}

fn test_incdec_ptr() {
    let mut ia = [0; 2];
    let a = &mut ia[1] as *mut _;

    assert_eq!(unsafe { a.offset(1) }, a.offset(1));
    assert_eq!(unsafe { a.offset(-1) }, a.offset(-1));
    assert_eq!(unsafe { a.offset(0) }, a.offset(0));
    assert_eq!(unsafe { a.offset(0) }, a.offset(0));
}

fn main() {
    test_all_incdec_arith(0);
    test_all_incdec_arith(1);
    test_all_incdec_arith(2);
    test_all_incdec_arith(usize::MAX);
    test_all_incdec_arith(usize::MAX / 2);
    test_incdec_ptr();
    std::process::exit(0);
}