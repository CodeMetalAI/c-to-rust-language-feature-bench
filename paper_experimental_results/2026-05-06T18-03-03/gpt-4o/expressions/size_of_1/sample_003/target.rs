fn main() {
    let dp: Box<f64> = match Box::new(3.14) {
        Ok(boxed) => boxed,
        Err(_) => std::process::exit(1),
    };

    if *dp != 3.14 {
        std::process::exit(2);
    }
}