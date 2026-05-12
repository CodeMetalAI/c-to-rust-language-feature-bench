struct S2 {
    v2: i32,
    s1p: Option<Box<S1>>,
}

struct S1 {
    v1: i32,
    s2p: Option<Box<S2>>,
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
    if let Some(ref s2p) = a.s2p {
        s2p.s1p = Some(Box::new(a));
    }

    if let Some(ref s2p) = a.s2p {
        if s2p.v2 != 20 {
            std::process::exit(1);
        }
        if let Some(ref s1p) = s2p.s1p {
            if s1p.v1 != 10 {
                std::process::exit(2);
            }
            if s1p.s2p.as_ref().map_or(false, |s2p| s2p.s1p.as_ref().map_or(false, |s1p| s1p.v1 != 10)) {
                std::process::exit(3);
            }
        }
    }
}