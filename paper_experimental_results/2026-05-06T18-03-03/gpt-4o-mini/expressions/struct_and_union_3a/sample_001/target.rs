struct N {
    alltypes: i32,
}

struct Ni {
    r#type: i32,
    intnode: i32,
}

struct Nf {
    r#type: i32,
    doublenode: f64,
}

enum U {
    N(N),
    Ni(Ni),
    Nf(Nf),
}

fn main() -> i32 {
    let mut u = U::Nf(Nf {
        r#type: 1,
        doublenode: 3.14,
    });

    if let U::Nf(ref nf) = u {
        if nf.r#type != 1 {
            return 1;
        }
    }

    u = U::N(N { alltypes: 0 });
    
    if let U::N(ref n) = u {
        if n.alltypes != 1 {
            return 1;
        }
    }

    u = U::Ni(Ni { r#type: 1, intnode: 0 });

    if let U::Ni(ref ni) = u {
        if ni.r#type != 1 {
            return 1;
        }
    }

    if let U::Nf(ref nf) = u {
        if nf.doublenode != 3.14 {
            return 3;
        }
    }

    0
}