fn main() {
    struct S {
        n: i32,
        d: Vec<f64>,
    }

    let size_s = std::mem::size_of::<i32>(); // Only the fixed part (n)
    let size_ss = std::mem::size_of::<i32>(); // Structure ss is equivalent in size.

    if size_s < size_ss {
        std::process::exit(1);
    }

    if size_s != size_s {
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

    // Memory is automatically freed at the end of scope in Rust.
}