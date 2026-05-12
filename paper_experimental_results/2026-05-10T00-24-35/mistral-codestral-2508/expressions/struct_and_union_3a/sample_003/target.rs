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
    type_field: i32,
    intnode: i32,
}

#[repr(C)]
struct Nf {
    type_field: i32,
    doublenode: f64,
}

fn main() {
    let mut u = U {
        nf: Nf {
            type_field: 1,
            doublenode: 3.14,
        },
    };

    unsafe {
        if u.n.alltypes != 1 {
            std::process::exit(1);
        }
        if u.ni.type_field != 1 {
            std::process::exit(1);
        }
        if u.nf.type_field != 1 {
            std::process::exit(2);
        }
        if u.nf.doublenode != 3.14 {
            std::process::exit(3);
        }
    }

    std::process::exit(0);
}