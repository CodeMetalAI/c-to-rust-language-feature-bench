struct N {
    alltypes: i32,
}

struct Ni {
    type: i32,
    intnode: i32,
}

struct Nf {
    type: i32,
    doublenode: f64,
}

enum U {
    N(N),
    Ni(Ni),
    Nf(Nf),
}

fn main() {
    let u = U::Nf(Nf {
        type: 1,
        doublenode: 3.14,
    });

    match u {
        U::N(n) => {
            if n.alltypes != 1 {
                std::process::exit(1);
            }
        }
        U::Ni(ni) => {
            if ni.type != 1 {
                std::process::exit(1);
            }
        }
        U::Nf(nf) => {
            if nf.type != 1 {
                std::process::exit(2);
            }
            if nf.doublenode != 3.14 {
                std::process::exit(3);
            }
        }
    }
}