// Define a struct s2
struct S2;

// Define a struct s1 with a reference to s2
struct S1 {
    v1: i32,
    s2p: Option<&S2>,
}

// Define a struct s2 with a reference to s1
struct S2 {
    v2: i32,
    s1p: Option<&S1>,
}

fn main() {
    // Create instances of s1 and s2
    let mut a = S1 { v1: 10, s2p: None };
    let mut b = S2 { v2: 20, s1p: None };

    // Set the references
    a.s2p = Some(&b);
    b.s1p = Some(&a);

    // Check the references
    if a.s2p.as_ref().unwrap().v2 != 20 {
        panic!("a.s2p->v2 != 20");
    }

    if b.s1p.as_ref().unwrap().v1 != 10 {
        panic!("b.s1p->v1 != 10");
    }

    if a.s2p.as_ref().unwrap().s1p.as_ref().unwrap() != &a {
        panic!("a.s2p->s1p != &a");
    }
}