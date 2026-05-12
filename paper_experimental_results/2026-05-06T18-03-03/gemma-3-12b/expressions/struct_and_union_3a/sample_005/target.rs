#[derive(Debug)]
union U {
    n: NestedStruct,
    ni: NestedStruct2,
    nf: NestedStruct3,
}

#[derive(Debug)]
struct NestedStruct {
    alltypes: i32,
}

#[derive(Debug)]
struct NestedStruct2 {
    type_: i32,
    intnode: i32,
}

#[derive(Debug)]
struct NestedStruct3 {
    type_: i32,
    doublenode: f64,
}

fn main() {
    let mut u = U { n: NestedStruct { alltypes: 0 }, ni: NestedStruct2 { type_: 0, intnode: 0 }, nf: NestedStruct3 { type_: 0, doublenode: 0.0 } };

    // Accessing union members requires casting.  This is a bit awkward, but necessary
    // to mimic the C++ behavior.  We'll use a match statement to handle the different cases.
    match &mut u {
        U { nf: _ } => {
            u.nf.type_ = 1;
            u.nf.doublenode = 3.14;
        }
        _ => {}
    }

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