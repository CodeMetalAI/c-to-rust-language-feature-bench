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

    match &mut u {
        U::Nf(ref mut nf) => {
            if nf.r#type != 1 {
                return 2;
            }
            if nf.doublenode != 3.14 {
                return 3;
            }
        }
        _ => return 1,
    }

    match &u {
        U::N(ref n) => {
            if n.alltypes != 1 {
                return 1;
            }
        }
        U::Ni(ref ni) => {
            if ni.r#type != 1 {
                return 1;
            }
        }
        _ => {}
    }

    0
}