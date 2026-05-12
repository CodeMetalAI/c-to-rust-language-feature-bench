struct S2 {
    v2: i32,
    s1p: Option<*mut S1>,
}

struct S1 {
    v1: i32,
    s2p: Option<*mut S2>,
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

    a.s2p = Some(&mut b);
    b.s1p = Some(&mut a);

    if let Some(s2p) = a.s2p {
        if unsafe { (*s2p).v2 } != 20 {
            std::process::exit(1);
        }
    }

    if let Some(s1p) = b.s1p {
        if unsafe { (*s1p).v1 } != 10 {
            std::process::exit(2);
        }
    }

    if let Some(s2p) = a.s2p {
        if let Some(s1p) = unsafe { (*s2p).s1p } {
            if s1p != &mut a {
                std::process::exit(3);
            }
        }
    }
}