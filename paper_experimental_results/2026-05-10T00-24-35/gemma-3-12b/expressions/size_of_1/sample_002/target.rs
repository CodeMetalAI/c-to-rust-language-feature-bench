fn main() {
    let mut dp: Box<f64> = Box::new(0.0);

    if dp.is_null() {
        return 1;
    }

    *dp = 3.14;
    if *dp != 3.14 {
        return 2;
    }

    return 0;
}