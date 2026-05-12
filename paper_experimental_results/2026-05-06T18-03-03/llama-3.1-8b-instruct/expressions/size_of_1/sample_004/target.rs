fn alloc(n: usize) -> *mut f64 {
    Box::into_raw(Box::new(f64::default()))
}

fn main() {
    let dp = alloc(1);
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