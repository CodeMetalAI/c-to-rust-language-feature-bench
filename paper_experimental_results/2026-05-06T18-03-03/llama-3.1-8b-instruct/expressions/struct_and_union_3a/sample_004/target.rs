#[derive(Debug)]
struct U {
    n: struct_1,
    ni: struct_2,
    nf: struct_3,
}

#[derive(Debug)]
struct struct_1 {
    alltypes: i32,
}

#[derive(Debug)]
struct struct_2 {
    type_: i32,
    intnode: i32,
}

#[derive(Debug)]
struct struct_3 {
    type_: i32,
    doublenode: f64,
}

fn main() {
    let mut u = U {
        n: struct_1 { alltypes: 0 },
        ni: struct_2 { type_: 0, intnode: 0 },
        nf: struct_3 { type_: 0, doublenode: 0.0 },
    };

    u.nf.type_ = 1;
    u.nf.doublenode = 3.14;

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

    std::process::exit(0);
}