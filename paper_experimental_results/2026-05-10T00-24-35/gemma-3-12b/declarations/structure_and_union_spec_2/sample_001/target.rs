fn main() {
    #[repr(C)]
    struct s {
        n: i32,
        d: [f64; 0], // Flexible array member - handled by Vec
    }

    #[repr(C)]
    struct ss {
        n: i32,
    }

    if std::mem::size_of::<s>() < std::mem::size_of::<ss>() {
        return 1;
    }

    if std::mem::offset::<s, f64>() != std::mem::size_of::<s>() {
        return 1;
    }

    let mut s1 = Box::new(s {
        n: 0,
        d: Vec::with_capacity(8),
    });
    let mut s2 = Box::new(s {
        n: 0,
        d: Vec::with_capacity(5),
    });

    if s1.d.len() == 0 || s2.d.len() == 0 {
        return 1;
    }

    s1.d[0] = 42.0;
    s2.d[0] = 24.0;

    if s1.d[0] != 42.0 || s2.d[0] != 24.0 {
        return 1;
    }

    drop(s1);
    drop(s2);

    0
}