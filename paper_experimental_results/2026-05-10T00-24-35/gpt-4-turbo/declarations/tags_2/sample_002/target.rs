struct S1<'a> {
    v1: i32,
    s2p: Option<&'a S2<'a>>,
}

struct S2<'a> {
    v2: i32,
    s1p: Option<&'a S1<'a>>,
}

fn main() {
    let mut a = S1 { v1: 10, s2p: None };
    let mut b = S2 { v2: 20, s1p: None };

    a.s2p = Some(&b);
    b.s1p = Some(&a);

    if a.s2p.unwrap().v2 != 20 {
        std::process::exit(1);
    }

    if b.s1p.unwrap().v1 != 10 {
        std::process::exit(2);
    }

    if !std::ptr::eq(a.s2p.unwrap().s1p.unwrap(), &a) {
        std::process::exit(3);
    }
}