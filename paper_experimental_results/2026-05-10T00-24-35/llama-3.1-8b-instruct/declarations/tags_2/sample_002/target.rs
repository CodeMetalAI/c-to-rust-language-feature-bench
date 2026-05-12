/*
 * Tagged pointer example
 */

struct S2 {}

struct S1 {
    v1: i32,
    s2p: *const S2,
}

struct S2 {
    v2: i32,
    s1p: *const S1,
}

fn main() {
    let mut a: S1 = S1 {
        v1: 10,
        s2p: &S2 {},
    };
    let mut b: S2 = S2 {
        v2: 20,
        s1p: &a,
    };

    a.s2p = &b;
    b.s1p = &a;

    if a.s2p.v2 != 20 {
        return 1;
    }

    if b.s1p.v1 != 10 {
        return 2;
    }

    if a.s2p.s1p != &a {
        return 3;
    }

    return 0;
}