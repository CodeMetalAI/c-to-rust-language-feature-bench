fn main() {
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
        r#type: i32,
        intnode: i32,
    }

    #[repr(C)]
    struct Nf {
        r#type: i32,
        doublenode: f64,
    }

    let mut u = U {
        nf: Nf {
            r#type: 1,
            doublenode: 3.14,
        },
    };

    unsafe {
        if u.n.alltypes != 1 {
            std::process::exit(1);
        }
        if u.ni.r#type != 1 {
            std::process::exit(1);
        }
        if u.nf.r#type != 1 {
            std::process::exit(2);
        }
        if (u.nf.doublenode - 3.14).abs() > std::f64::EPSILON {
            std::process::exit(3);
        }
    }
}