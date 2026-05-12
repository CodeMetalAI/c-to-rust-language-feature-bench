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

    unsafe {
        s1.d[0] = 42.0;
        s2.d[0] = 24.0;
    }

    if s1.d[0] != 42.0 || s2.d[0] != 24.0 {
        return 1;
    }

    // Rust does not have manual memory management, so no need to free.
    // std::mem::forget(&s1);  //Prevent s1 from being dropped. NOTE: Dropping is happening at the end of main()
    // std::mem::forget(&s2);

    0
}