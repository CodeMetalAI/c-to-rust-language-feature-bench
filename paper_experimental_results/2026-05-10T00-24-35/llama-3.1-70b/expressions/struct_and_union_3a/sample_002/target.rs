#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
struct N {
    alltypes: i32,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
struct NI {
    type_: i32,
    intnode: i32,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
struct NF {
    type_: i32,
    doublenode: f64,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
union U {
    n: N,
    ni: NI,
    nf: NF,
}

fn main() {
    let mut u = U::default();

    unsafe {
        u.nf.type_ = 1;
        u.nf.doublenode = 3.14;
    }

    unsafe {
        if u.n.alltypes!= 1 {
            std::process::exit(1);
        }
        if u.ni.type_!= 1 {
            std::process::exit(1);
        }
        if u.nf.type_!= 1 {
            std::process::exit(2);
        }
        if (u.nf.doublenode - 3.14).abs() > f64::EPSILON {
            std::process::exit(3);
        }
    }

    std::process::exit(0);
}