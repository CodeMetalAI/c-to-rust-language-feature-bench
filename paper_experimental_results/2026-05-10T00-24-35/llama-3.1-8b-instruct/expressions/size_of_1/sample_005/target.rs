fn main() {
    let dp: *mut f64 = Box::into_raw(Box::new(3.14));

    if dp.is_null() {
        return 1;
    }

    unsafe {
        if *dp!= 3.14 {
            Box::from_raw(dp);
            return 2;
        }
    }

    Box::from_raw(dp);
    std::process::exit(0);
}