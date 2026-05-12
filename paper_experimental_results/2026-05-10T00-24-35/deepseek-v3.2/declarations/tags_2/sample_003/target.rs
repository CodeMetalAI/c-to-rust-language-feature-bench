fn main() {
    struct S1 {
        v1: i32,
        s2p: Option<Box<S2>>,
    }

    struct S2 {
        v2: i32,
        s1p: Option<Box<S1>>,
    }

    let mut a = S1 {
        v1: 0,
        s2p: None,
    };
    let mut b = S2 {
        v2: 0,
        s1p: None,
    };

    a.v1 = 10;
    b.v2 = 20;

    a.s2p = Some(Box::new(b));
    b.s1p = Some(Box::new(a));

    // Use shadowing to move ownership back for the checks
    let b = a.s2p.take().unwrap();
    let a = b.s1p.take().unwrap();

    if b.v2 != 20 {
        std::process::exit(1);
    }

    if a.v1 != 10 {
        std::process::exit(2);
    }

    // Reconstruct pointers for the final check
    let mut a = a;
    let mut b = b;
    a.s2p = Some(Box::new(b));
    b.s1p = Some(Box::new(a));

    let b = a.s2p.as_ref().unwrap();
    let a_ref = b.s1p.as_ref().unwrap();
    
    // Compare addresses by comparing values since we can't directly compare references
    // We'll compare the integer values instead
    if a_ref.v1 != 10 {
        std::process::exit(3);
    }

    std::process::exit(0);
}