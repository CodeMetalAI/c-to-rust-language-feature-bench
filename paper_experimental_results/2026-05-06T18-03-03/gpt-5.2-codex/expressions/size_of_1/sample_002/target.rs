use std::process::exit;

fn main() {
    let mut v: Vec<f64> = Vec::new();

    if v.try_reserve_exact(1).is_err() {
        exit(1);
    }

    v.push(0.0);
    v[0] = 3.14;
    if v[0] != 3.14 {
        exit(2);
    }

    exit(0);
}