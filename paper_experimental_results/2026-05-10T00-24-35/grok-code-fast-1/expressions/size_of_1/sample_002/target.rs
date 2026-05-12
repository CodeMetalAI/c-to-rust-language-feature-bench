fn main() {
    let mut v: Vec<f64> = Vec::new();
    if v.try_reserve(1).is_err() {
        std::process::exit(1);
    }
    v.push(3.14);
    if v[0] != 3.14 {
        std::process::exit(2);
    }
    std::process::exit(0);
}