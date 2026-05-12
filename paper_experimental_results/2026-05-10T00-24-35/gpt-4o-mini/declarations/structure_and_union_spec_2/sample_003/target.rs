struct S {
    n: i32,
    d: Vec<f64>,
}

struct SS {
    n: i32,
}

fn main() -> i32 {
    if std::mem::size_of::<S>() < std::mem::size_of::<SS>() {
        return 1;
    }

    if std::mem::offset_of!(S, d) != std::mem::size_of::<S>() {
        return 1;
    }

    let mut s1 = S { n: 0, d: vec![0.0; 8] };
    let mut s2 = S { n: 0, d: vec![0.0; 5] };

    s1.d[0] = 42.0;
    s2.d[0] = 24.0;

    if s1.d[0] != 42.0 || s2.d[0] != 24.0 {
        return 1;
    }

    0
}