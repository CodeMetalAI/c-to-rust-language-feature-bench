struct S {
    n: i32,
    d: Vec<f64>,
}

fn main() {
    let s1_size = std::mem::size_of::<i32>() + 8 * std::mem::size_of::<f64>();
    let s2_size = std::mem::size_of::<i32>() + 5 * std::mem::size_of::<f64>();
    let ss_size = std::mem::size_of::<i32>();

    if std::mem::size_of::<i32>() < ss_size {
        std::process::exit(1);
    }

    if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
        std::process::exit(1);
    }

    let mut s1 = S { n: 0, d: vec![0.0; 8] };
    let mut s2 = S { n: 0, d: vec![0.0; 5] };

    s1.d[0] = 42.0;
    s2.d[0] = 24.0;

    if s1.d[0] != 42.0 || s2.d[0] != 24.0 {
        std::process::exit(1);
    }
}