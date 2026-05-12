// C11 6.7.2.3 — Tags (Example 2)

struct S1 {
    v1: i32,
    s2p: Option<Box<S2>>,
}

struct S2 {
    v2: i32,
    s1p: Option<Box<S1>>,
}

fn main() {
    let a = S1 { v1: 10, s2p: None };
    let b = S2 { v2: 20, s1p: None };

    a.s2p = Some(Box::new(b));
    b.s1p = Some(Box::new(a));

    if a.s2p.as_ref().unwrap().v2 != 20 {
        return 1;
    }

    if b.s1p.as_ref().unwrap().v1 != 10 {
        return 2;
    }

    if a.s2p.as_ref().unwrap().s1p.as_ref().unwrap() != &a {
        return 3;
    }

    return 0;
}