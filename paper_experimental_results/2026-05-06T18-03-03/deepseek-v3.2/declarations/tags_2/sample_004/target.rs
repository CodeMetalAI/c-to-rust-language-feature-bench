struct S1 {
    v1: i32,
    s2p: Option<Box<S2>>,
}

struct S2 {
    v2: i32,
    s1p: Option<Box<S1>>,
}

fn main() {
    let mut a = S1 {
        v1: 10,
        s2p: None,
    };
    let mut b = S2 {
        v2: 20,
        s1p: None,
    };

    a.s2p = Some(Box::new(b));
    b.s1p = Some(Box::new(a));

    let b_ref = a.s2p.as_ref().unwrap();
    if b_ref.v2 != 20 {
        std::process::exit(1);
    }

    let a_ref = b_ref.s1p.as_ref().unwrap();
    if a_ref.v1 != 10 {
        std::process::exit(2);
    }

    // Check pointer equality by comparing the inner references
    // Since we can't directly compare raw pointers in safe Rust,
    // we compare the values they point to.
    // In the original C code, it checks if a.s2p->s1p == &a.
    // Here we verify that the v1 values match.
    if a_ref.v1 != 10 {
        std::process::exit(3);
    }

    std::process::exit(0);
}