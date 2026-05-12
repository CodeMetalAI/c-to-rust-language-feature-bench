fn main() {
    struct S {
        n: i32,
        d: Vec<f64>,
    }

    struct SS {
        n: i32,
    }

    // Check the size of S and SS
    if std::mem::size_of::<S>() < std::mem::size_of::<SS>() {
        std::process::exit(1);
    }

    // Create instances of S with dynamic arrays
    let mut s1 = S {
        n: 0,
        d: vec![0.0; 8],
    };
    let mut s2 = S {
        n: 0,
        d: vec![0.0; 5],
    };

    // Assign values to the dynamic arrays
    s1.d[0] = 42.0;
    s2.d[0] = 24.0;

    // Check the assigned values
    if s1.d[0] != 42.0 || s2.d[0] != 24.0 {
        std::process::exit(1);
    }
}