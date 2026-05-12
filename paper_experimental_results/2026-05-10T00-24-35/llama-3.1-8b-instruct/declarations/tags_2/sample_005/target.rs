struct S2;

struct S1 {
    v1: i32,
    s2p: Option<&'static S2>,
}

struct S2 {
    v2: i32,
    s1p: Option<&'static S1>,
}

fn main() {
    let a = S1 { v1: 10, s2p: None };
    let b = S2 { v2: 20, s1p: None };

    a.s2p = Some(&b);
    b.s1p = Some(&a);

    if a.s2p.as_ref().unwrap().v2!= 20 {
        return;
    }

    if b.s1p.as_ref().unwrap().v1!= 10 {
        return;
    }

    if a.s2p.as_ref().unwrap().s1p.as_ref().unwrap()!= &a {
        return;
    }

    println!("Success!");
}