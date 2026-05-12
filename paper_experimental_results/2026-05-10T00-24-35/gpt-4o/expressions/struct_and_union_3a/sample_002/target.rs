#[repr(C)]
union U {
    n: N,
    ni: NI,
    nf: NF,
}

#[repr(C)]
struct N {
    alltypes: i32,
}

#[repr(C)]
struct NI {
    type_: i32,
    intnode: i32,
}

#[repr(C)]
struct NF {
    type_: i32,
    doublenode: f64,
}

fn main() {
    let mut u = U { nf: NF { type_: 0, doublenode: 0.0 } };

    // Safe block to mutate union fields
    unsafe {
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
    }

    std::process::exit(0);
}