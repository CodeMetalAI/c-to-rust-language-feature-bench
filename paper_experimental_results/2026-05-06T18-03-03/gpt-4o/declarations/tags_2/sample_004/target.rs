struct S1<'a> {
    v1: i32,
    s2p: Option<&'a S2<'a>>,
}

struct S2<'a> {
    v2: i32,
    s1p: Option<&'a S1<'a>>,
}

fn main() -> i32 {
    let mut a = S1 { v1: 10, s2p: None };
    let mut b = S2 { v2: 20, s1p: None };

    a.s2p = Some(&b);
    b.s1p = Some(&a);

    if a.s2p.unwrap().v2 != 20 {
        return 1;
    }

    if b.s1p.unwrap().v1 != 10 {
        return 2;
    }

    if a.s2p.unwrap().s1p.unwrap() as *const _ != &a as *const _ {
        return 3;
    }

    0
}