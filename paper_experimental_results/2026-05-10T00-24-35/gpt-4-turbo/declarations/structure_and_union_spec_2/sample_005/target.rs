fn main() {
    struct S {
        n: i32,
        d: Vec<f64>,
    }

    let size_of_s = std::mem::size_of::<i32>() + std::mem::size_of::<Vec<f64>>();
    let size_of_ss = std::mem::size_of::<i32>();

    if size_of_s < size_of_ss || std::mem::size_of::<i32>() != size_of_s {
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