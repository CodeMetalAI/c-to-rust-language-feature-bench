fn main() {
    let dp: Box<f64> = Box::new(0.0);

    *dp = 3.14;
    if *dp != 3.14.0 {
        return;
    }

    // The Box will be automatically deallocated when it goes out of scope
}