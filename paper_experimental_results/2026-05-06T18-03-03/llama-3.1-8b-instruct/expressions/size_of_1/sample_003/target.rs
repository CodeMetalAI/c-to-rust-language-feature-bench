fn alloc<T>(n: usize) -> *mut T {
    Box::into_raw(Box::new(vec![0; n]))
}

fn main() {
    let dp = alloc::<f64>(std::mem::size_of::<f64>()) as *mut f64;

    if dp.is_null() {
        return 1;
    }

    unsafe {
        *dp = 3.14;
        if *dp != 3.14 {
            Box::from_raw(dp);
            return 2;
        }
    }

    unsafe {
        Box::from_raw(dp);
    }
    0
}