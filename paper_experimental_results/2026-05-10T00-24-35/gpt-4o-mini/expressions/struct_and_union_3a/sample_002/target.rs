struct U {
    alltypes: i32,
    ni: Option<Ni>,
    nf: Option<Nf>,
}

struct Ni {
    r#type: i32,
    intnode: i32,
}

struct Nf {
    r#type: i32,
    doublenode: f64,
}

fn main() -> i32 {
    let mut u = U {
        alltypes: 0,
        ni: None,
        nf: Some(Nf {
            r#type: 1,
            doublenode: 3.14,
        }),
    };

    u.alltypes = u.nf.as_ref().unwrap().r#type;

    if u.alltypes != 1 {
        return 1;
    }
    if let Some(ni) = &u.ni {
        if ni.r#type != 1 {
            return 1;
        }
    }
    if let Some(nf) = &u.nf {
        if nf.r#type != 1 {
            return 2;
        }
        if nf.doublenode != 3.14 {
            return 3;
        }
    }

    0
}