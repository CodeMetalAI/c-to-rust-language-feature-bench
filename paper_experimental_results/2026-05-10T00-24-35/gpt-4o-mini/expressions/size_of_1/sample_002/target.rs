fn main() {
    let dp: Box<f64> = Box::new(0.0);

    if dp.is_null() {
        std::process::exit(1);
    }

    let dp = Box::into_raw(dp); // Convert Box to raw pointer to mimic behavior
    unsafe { *dp = 3.14 };

    if unsafe { *dp } != 3.14 {
        unsafe { Box::from_raw(dp) }; // Convert back to Box to free memory
        std::process::exit(2);
    }

    unsafe { Box::from_raw(dp) }; // Convert back to Box to free memory
}