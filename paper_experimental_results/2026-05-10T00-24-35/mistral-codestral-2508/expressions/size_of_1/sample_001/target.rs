fn main() {
    let dp: Box<f64> = match Box::new(0.0) {
        dp => dp,
    };

    *dp = 3.14;
    if *dp != 3.14.0 {
        return;
    }
}