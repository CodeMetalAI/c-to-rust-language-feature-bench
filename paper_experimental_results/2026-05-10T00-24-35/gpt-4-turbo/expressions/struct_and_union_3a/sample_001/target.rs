fn main() {
    union U {
        n: N,
        ni: NI,
        nf: NF,
    }

    struct N {
        alltypes: i32,
    }

    struct NI {
        type_: i32,
        intnode: i32,
    }

    struct NF {
        type_: i32,
        doublenode: f64,
    }

    let mut u = U { nf: NF { type_: 1, doublenode: 3.14 } };

    unsafe {
        if u.n.alltypes != 1 {
            std::process::exit(1);
        }
        if u.ni.type_ != 1 {
            std::process::exit(1);
        }
        if u.nf.type_ != 1 {
            std::process::exit(2);
        }
        if u.nf.doublenode != 3.14 {
            std::process::exit(3);
        }
    }
}