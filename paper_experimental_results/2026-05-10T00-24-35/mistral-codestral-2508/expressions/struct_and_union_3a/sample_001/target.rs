fn main() {
    #[derive(Debug)]
    union U {
        alltypes: i32,
        ni: Ni,
        nf: Nf,
    }

    #[derive(Debug)]
    struct Ni {
        type_field: i32,
        intnode: i32,
    }

    #[derive(Debug)]
    struct Nf {
        type_field: i32,
        doublenode: f64,
    }

    let mut u = U { alltypes: 0 };

    unsafe {
        let nf = Nf {
            type_field: 1,
            doublenode: 3.14,
        };
        u.nf = nf;

        if u.alltypes != 1 {
            std::process::exit(1);
        }

        let ni = std::mem::transmute::<Nf, Ni>(u.nf);
        if ni.type_field != 1 {
            std::process::exit(1);
        }

        if u.nf.type_field != 1 {
            std::process::exit(2);
        }

        if u.nf.doublenode != 3.14 {
            std::process::exit(3);
        }
    }
}