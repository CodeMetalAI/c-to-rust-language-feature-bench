#[derive(Debug)]
struct s {
    n: i32,
    d: [f64; 8],
}

#[derive(Debug)]
struct ss {
    n: i32,
}

fn main() {
    if std::mem::size_of::<s>() < std::mem::size_of::<ss>() {
        return 1;
    }

    if std::mem::offset::<s, f64>("d") != std::mem::size_of::<s>() {
        return 1;
    }

    let mut s1 = s {
        n: 0,
        d: [0.0; 8],
    };
    let mut s2 = s {
        n: 0,
        d: [0.0; 8],
    };

    s1.d[0] = 42.0;
    s2.d[0] = 24.0;

    if s1.d[0] != 42.0 || s2.d[0] != 24.0 {
        return 1;
    }
    
    // No need to free in Rust, the memory is automatically managed.
    return 0;
}