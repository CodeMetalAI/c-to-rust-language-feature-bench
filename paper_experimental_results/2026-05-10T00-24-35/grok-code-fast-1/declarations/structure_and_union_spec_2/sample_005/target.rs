use std::mem;
use std::process;

struct S {
    n: i32,
    d: [f64; 0],
}

struct Ss {
    n: i32,
}

fn main() {
    if mem::size_of::<S>() < mem::size_of::<Ss>() {
        process::exit(1);
    }

    if mem::offset_of!(S, d) != mem::size_of::<S>() {
        process::exit(1);
    }

    let mut d1 = vec![0.0f64; 8];
    let mut d2 = vec![0.0f64; 5];

    d1[0] = 42.0;
    d2[0] = 24.0;

    if d1[0] != 42.0 || d2[0] != 24.0 {
        process::exit(1);
    }

    process::exit(0);
}