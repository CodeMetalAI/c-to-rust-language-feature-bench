fn alloc<T>(n: usize) -> *mut T {
    Box::into_raw(Box::new([0; n]))
}

fn main() {
    let dp = unsafe { alloc::<f64>(std::mem::size_of::<f64>()) as *mut f64 };
    if dp.is_null() {
        return 1;
    }
    *dp = 3.14;
    if *dp!= 3.14 {
        unsafe { Box::from_raw(dp) };
        return 2;
    }
    unsafe { Box::from_raw(dp) };
    return 0;
}