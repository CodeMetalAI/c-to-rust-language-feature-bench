struct S1 {
    v1: i32,
    s2p: Option<Box<S2>>,
}

struct S2 {
    v2: i32,
    s1p: Option<Box<S1>>,
}

fn main() {
    let a = S1 {
        v1: 10,
        s2p: None,
    };
    let b = S2 {
        v2: 20,
        s1p: None,
    };

    let a_box = Box::new(a);
    let b_box = Box::new(b);

    let mut a_ref = a_box;
    let mut b_ref = b_box;

    a_ref.s2p = Some(b_ref);
    b_ref.s1p = Some(a_ref);

    if a_ref.s2p.as_ref().unwrap().v2 != 20 {
        return std::process::exit(1);
    }

    if b_ref.s1p.as_ref().unwrap().v1 != 10 {
        return std::process::exit(2);
    }

    // Compare pointers by comparing the Box values
    if !std::ptr::eq(
        b_ref.s1p.as_ref().unwrap().as_ref(),
        a_ref.as_ref(),
    ) {
        return std::process::exit(3);
    }

    std::process::exit(0);
}