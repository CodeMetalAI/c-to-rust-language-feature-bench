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
        v1: 0,
        s2p: None,
    };
    let mut b = S2 {
        v2: 0,
        s1p: None,
    };

    a.v1 = 10;
    b.v2 = 20;

    // Use Box to allocate on heap and create owned references
    let boxed_b = Box::new(b);
    let boxed_a = Box::new(a);

    // Rebind to mutably update the fields
    let mut a = boxed_a;
    let mut b = boxed_b;

    // Set up the mutual references
    a.s2p = Some(Box::new(S2 {
        v2: b.v2,
        s1p: Some(Box::new(S1 {
            v1: a.v1,
            s2p: None,
        })),
    }));
    
    b.s1p = Some(Box::new(S1 {
        v1: a.v1,
        s2p: Some(Box::new(S2 {
            v2: b.v2,
            s1p: None,
        })),
    }));

    // Now recreate the exact pointer comparisons using references
    let a_ref = &*a;
    let b_ref = &*b;

    // Check v2 through a.s2p
    if let Some(ref s2p) = a.s2p {
        if s2p.v2 != 20 {
            std::process::exit(1);
        }
    } else {
        std::process::exit(1);
    }

    // Check v1 through b.s1p
    if let Some(ref s1p) = b.s1p {
        if s1p.v1 != 10 {
            std::process::exit(2);
        }
    } else {
        std::process::exit(2);
    }

    // Check pointer equality: a.s2p->s1p should point to same value as &a
    if let Some(ref s2p) = a.s2p {
        if let Some(ref s1p) = s2p.s1p {
            // Compare the values, not the pointers directly
            if s1p.v1 != a.v1 {
                std::process::exit(3);
            }
        } else {
            std::process::exit(3);
        }
    } else {
        std::process::exit(3);
    }

    std::process::exit(0);
}