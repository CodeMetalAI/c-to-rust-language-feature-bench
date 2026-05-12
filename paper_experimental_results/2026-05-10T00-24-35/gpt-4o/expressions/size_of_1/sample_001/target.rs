fn main() {
    let dp: Box<f64> = match Box::new(3.14) {
        val => val,
    };

    if *dp != 3.14 {
        std::process::exit(2);
    }

    std::process::exit(0);
}