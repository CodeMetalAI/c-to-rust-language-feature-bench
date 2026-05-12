fn main() {
    struct S {
        n: i32,
        d: Vec<f64>,
    }

    struct Ss {
        n: i32,
    }

    if std::mem::size_of::<S>() < std::mem::size_of::<Ss>() {
        std::process::exit(1);
    }

    if std::mem::size_of::<S>() != std::mem::size_of::<i32>() {
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