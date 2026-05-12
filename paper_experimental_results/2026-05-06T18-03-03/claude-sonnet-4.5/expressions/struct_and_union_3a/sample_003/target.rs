use std::mem;

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

#[repr(C)]
union U {
    n: N,
    ni: Ni,
    nf: Nf,
}

fn main() {
    let mut u = U { n: N { alltypes: 0 } };

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