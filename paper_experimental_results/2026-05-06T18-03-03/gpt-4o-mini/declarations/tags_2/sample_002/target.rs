struct S2 {
    v2: i32,
    s1p: Option<Box<S1>>,
}

struct S1 {
    v1: i32,
    s2p: Option<Box<S2>>,
}

fn main() -> i32 {
    let mut a = S1 {
        v1: 10,
        s2p: None,
    };
    let mut b = S2 {
        v2: 20,
        s1p: None,
    };

    a.s2p = Some(Box::new(b));
    if let Some(ref s2) = a.s2p {
        if s2.v2 != 20 {
            return 1;
        }
        let s2_mut = s2.as_ref().clone();
        b.s1p = Some(Box::new(a));
        if let Some(ref s1) = b.s1p {
            if s1.v1 != 10 {
                return 2;
            }
            if s2_mut.s1p.as_ref().map(|s| &**s) != Some(&a) {
                return 3;
            }
        }
    }

    0
}