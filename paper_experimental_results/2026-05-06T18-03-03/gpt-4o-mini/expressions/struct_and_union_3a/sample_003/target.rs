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
    let mut u = U::Nf(Nf { r#type: 1, doublenode: 3.14 });

    match &mut u {
        U::Nf(nf) => {
            if nf.r#type != 1 {
                return 2;
            }
            if nf.doublenode != 3.14 {
                return 3;
            }
        }
        _ => return 1,
    }

    if let U::N(n) = u {
        if n.alltypes != 1 {
            return 1;
        }
    }

    if let U::Ni(ni) = u {
        if ni.r#type != 1 {
            return 1;
        }
    }

    0
}