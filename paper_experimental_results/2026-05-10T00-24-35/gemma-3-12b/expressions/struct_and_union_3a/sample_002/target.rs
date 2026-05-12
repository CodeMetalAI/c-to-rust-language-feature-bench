#[derive(Debug)]
union U {
    n: Nested,
    ni: NestedInteger,
    nf: NestedFloat,
}

#[derive(Debug)]
struct Nested {
    alltypes: i32,
}

#[derive(Debug)]
struct NestedInteger {
    type_: i32,
    intnode: i32,
}

#[derive(Debug)]
struct NestedFloat {
    type_: i32,
    doublenode: f64,
}

fn main() {
    let mut u = U { n: Nested { alltypes: 0 }, ni: NestedInteger { type_: 0, intnode: 0 }, nf: NestedFloat { type_: 0, doublenode: 0.0 } };

    // Simulate assignment to nf
    unsafe {
        let nf_ptr = &mut u as *mut U as *mut NestedFloat;
        (*nf_ptr).type_ = 1;
        (*nf_ptr).doublenode = 3.14;
    }

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