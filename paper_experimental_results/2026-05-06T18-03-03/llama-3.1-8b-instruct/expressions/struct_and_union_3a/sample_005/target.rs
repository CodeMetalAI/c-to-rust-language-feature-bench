// Define a union with three fields
#[derive(Debug)]
struct U {
    n: struct_n,
    ni: struct_ni,
    nf: struct_nf,
}

#[derive(Debug)]
struct struct_n {
    alltypes: i32,
}

#[derive(Debug)]
struct struct_ni {
    type_: i32,
    intnode: i32,
}

#[derive(Debug)]
struct struct_nf {
    type_: i32,
    doublenode: f64,
}

fn main() {
    let mut u = U {
        n: struct_n { alltypes: 0 },
        ni: struct_ni { type_: 0, intnode: 0 },
        nf: struct_nf { type_: 0, doublenode: 0.0 },
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

    return 0;
}