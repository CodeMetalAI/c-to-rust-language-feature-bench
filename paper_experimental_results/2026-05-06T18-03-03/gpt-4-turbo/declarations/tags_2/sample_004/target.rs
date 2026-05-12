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

    if let Some(s2) = a.s2p {
        if s2.v2 != 20 {
            std::process::exit(1);
        }
        if let Some(s1) = s2.s1p {
            if s1.v1 != 10 {
                std::process::exit(2);
            }
            if !std::ptr::eq(s1, &a) {
                std::process::exit(3);
            }
        }
    }
}