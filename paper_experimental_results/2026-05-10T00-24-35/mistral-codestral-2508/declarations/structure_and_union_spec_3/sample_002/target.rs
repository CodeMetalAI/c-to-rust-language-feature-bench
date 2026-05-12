use std::mem::size_of;

struct S {
    anonymous: Anonymous,
    a: Vec<i32>,
}

struct Anonymous {
    i: i32,
}

fn main() {
    let mut p: Box<S>;

    if size_of::<Anonymous>() != 0 {
        std::process::exit(1);
    }

    if size_of::<S>() != size_of::<Anonymous>() {
        std::process::exit(2);
    }

    p = Box::new(S {
        anonymous: Anonymous { i: 0 },
        a: vec![0; 4],
    });

    p.anonymous.i = 7;
    p.a[0] = 11;
    p.a[3] = 22;

    if p.anonymous.i != 7 || p.a[0] != 11 || p.a[3] != 22 {
        std::process::exit(4);
    }

    // No need to explicitly free p in Rust, it will be dropped automatically
    std::process::exit(0);
}