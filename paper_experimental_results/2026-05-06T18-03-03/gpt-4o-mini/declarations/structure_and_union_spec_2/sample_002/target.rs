use std::mem;

struct S {
    n: i32,
    d: Vec<f64>,
}

struct SS {
    n: i32,
}

fn main() {
    if mem::size_of::<S>() < mem::size_of::<SS>() {
        std::process::exit(1);
    }

    if mem::offset_of!(S, d) != mem::size_of::<S>() {
        std::process::exit(1);
    }

    let mut s1 = S { n: 0, d: vec![0.0; 8] };
    let mut s2 = S { n: 0, d: vec![0.0; 5] };

    s1.d[0] = 42.0;
    s2.d[0] = 24.0;

    if s1.d[0] != 42.0 || s2.d[0] != 24.0 {
        std::process::exit(1);
    }

    // No need to explicitly free memory in Rust; it will be done automatically.
}