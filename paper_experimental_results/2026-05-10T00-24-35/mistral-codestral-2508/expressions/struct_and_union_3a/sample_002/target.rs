union U {
    n: N,
    ni: Ni,
    nf: Nf,
}

struct N {
    alltypes: i32,
}

struct Ni {
    type_: i32,
    intnode: i32,
}

struct Nf {
    type_: i32,
    doublenode: f64,
}

fn main() {
    let mut u: U;

    unsafe {
        u = U {
            nf: Nf {
                type_: 1,
                doublenode: 3.14,
            },
        };

        if (*u.n).alltypes != 1 {
            std::process::exit(1);
        }
        if (*u.ni).type_ != 1 {
            std::process::exit(1);
        }
        if (*u.nf).type_ != 1 {
            std::process::exit(2);
        }
        if (*u.nf).doublenode != 3.14 {
            std::process::exit(3);
        }
    }

    std::process::exit(0);
}