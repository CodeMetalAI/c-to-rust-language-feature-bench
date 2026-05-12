#[repr(C)]
union U {
    n: N,
    ni: Ni,
    nf: Nf,
}

#[repr(C)]
struct N {
    alltypes: i32,
}

#[repr(C)]
struct Ni {
    type_: i32,
    intnode: i32,
}

#[repr(C)]
struct Nf {
    type_: i32,
    doublenode: f64,
}

fn main() {
    let mut u = U { n: N { alltypes: 0 } };

    unsafe {
        u.nf.type_ = 1;
        u.nf.doublenode = 3.14;
    }

    let exit_code = unsafe {
        if u.n.alltypes != 1 {
            1
        } else if u.ni.type_ != 1 {
            1
        } else if u.nf.type_ != 1 {
            2
        } else if u.nf.doublenode != 3.14 {
            3
        } else {
            0
        }
    };

    std::process::exit(exit_code);
}