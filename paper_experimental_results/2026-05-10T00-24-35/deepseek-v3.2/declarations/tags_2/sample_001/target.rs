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

    if a.s2p.as_ref().unwrap().v2 != 20 {
        return;
    }

    if b.s1p.as_ref().unwrap().v1 != 10 {
        return;
    }

    // Since we moved a and b into the boxes, we cannot compare addresses directly.
    // Instead, we can compare the values.
    let a_ref = b.s1p.as_ref().unwrap();
    if a_ref.v1 != a.v1 || a_ref.s2p.is_none() {
        return;
    }

    std::process::exit(0);
}