fn main() {
    let mut v: Vec<f64> = Vec::new();

    if v.try_reserve_exact(1).is_err() {
        std::process::exit(1);
    }

    v.push(3.14_f64);

    if v[0] != 3.14_f64 {
        std::process::exit(2);
    }
}