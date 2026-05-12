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

fn main() -> i32 {
    let mut u = U { nf: Nf { r#type: 0, doublenode: 0.0 } };

    unsafe {
        u.nf.r#type = 1;
        u.nf.doublenode = 3.14;

        if (*u.n).alltypes != 1 {
            return 1;
        }
        if (*u.ni).r#type != 1 {
            return 1;
        }
        if (*u.nf).r#type != 1 {
            return 2;
        }
        if (*u.nf).doublenode != 3.14 {
            return 3;
        }
    }

    return 0;
}