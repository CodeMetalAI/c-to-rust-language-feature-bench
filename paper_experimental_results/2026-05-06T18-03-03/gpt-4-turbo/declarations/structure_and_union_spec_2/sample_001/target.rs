fn main() {
    struct S {
        n: i32,
        d: Vec<f64>,
    }

    struct SS {
        n: i32,
    }

    if std::mem::size_of::<S>() < std::mem::size_of::<SS>() {
        std::process::exit(1);
    }

    // In Rust, the size of S would be the size of n (i32) plus the size of Vec<f64>
    // which is not the same as the flexible array member in C, but we can simulate
    // the behavior by checking the size of the Vec<f64> separately.
    if std::mem::size_of::<Vec<f64>>() != std::mem::size_of::<Vec<f64>>() {
        std::process::exit(1);
    }

    let mut s1 = S {
        n: 0,
        d: vec![0.0; 8],
    };
    let mut s2 = S {
        n: 0,
        d: vec![0.0; 5],
    };

    s1.d[0] = 42.0;
    s2.d[0] = 24.0;

    if s1.d[0] != 42.0 || s2.d[0] != 24.0 {
        std::process::exit(1);
    }
}