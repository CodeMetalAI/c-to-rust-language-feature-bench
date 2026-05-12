fn alloc(n: usize) -> *mut f64 {
    Box::into_raw(Box::new([0.0; n]))
}

fn main() {
    let dp = unsafe { alloc(1) };

    if dp.is_null() {
        return 1;
    }

    *dp = 3.14;
    if unsafe { *dp }!= 3.14 {
        unsafe { Box::from_raw(dp) };
        return 2;
    }

    unsafe { Box::from_raw(dp) };
    0
}