#[derive(Debug)]
struct U {
    n: struct_ {
        alltypes: i32,
    },
    ni: struct_ {
        type_: i32,
        intnode: i32,
    },
    nf: struct_ {
        type_: i32,
        doublenode: f64,
    },
}

struct struct_ {
    alltypes: i32,
}

struct struct_2 {
    type_: i32,
    intnode: i32,
}

struct struct_3 {
    type_: i32,
    doublenode: f64,
}

fn main() {
    let mut u = U {
        n: struct_ {
            alltypes: 0,
        },
        ni: struct_2 {
            type_: 0,
            intnode: 0,
        },
        nf: struct_3 {
            type_: 0,
            doublenode: 0.0,
        },
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