/*
 * size_of_1
 */

fn main() {
    let dp = Box::into_raw(Box::new(0.0f64));

    if dp.is_null() {
        std::process::exit(1);
    }

    unsafe {
        *dp = 3.14;
        if *dp != 3.14 {
            Box::from_raw(dp); // Reclaim memory
            std::process::exit(2);
        }

        Box::from_raw(dp); // Reclaim memory
    }
}