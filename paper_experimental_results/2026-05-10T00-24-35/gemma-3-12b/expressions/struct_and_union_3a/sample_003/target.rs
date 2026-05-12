#[derive(Debug)]
union U {
    n: NestedAlltypes,
    ni: NestedIntInt,
    nf: NestedDouble,
}

#[derive(Debug)]
struct NestedAlltypes {
    alltypes: i32,
}

#[derive(Debug)]
struct NestedIntInt {
    type_: i32,
    intnode: i32,
}

#[derive(Debug)]
struct NestedDouble {
    type_: i32,
    doublenode: f64,
}

fn main() {
    let mut u = U { n: NestedAlltypes { alltypes: 0 }, ni: NestedIntInt { type_: 0, intnode: 0 }, nf: NestedDouble { type_: 0, doublenode: 0.0 } };

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