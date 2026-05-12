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

union U {
    n: N,
    ni: Ni,
    nf: Nf,
}

fn main() -> i32 {
    let mut u = U {
        n: N { alltypes: 0 },
        ..Default::default()
    };

    u.nf.type_ = 1;
    u.nf.doublenode = 3.14;

    if u.n.alltypes != 1 {
        return 1;
    }
    if u.ni.type_ != 1 {
        return 1;
    }
    if u.nf.type_ != 1 {
        return 2;
    }
    if u.nf.doublenode != 3.14 {
        return 3;
    }

    0
}