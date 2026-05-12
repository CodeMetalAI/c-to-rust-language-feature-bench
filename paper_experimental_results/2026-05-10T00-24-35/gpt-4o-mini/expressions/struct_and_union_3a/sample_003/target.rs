#[derive(Debug, Copy, Clone)]
union U {
    n: N,
    ni: NI,
    nf: NF,
}

#[derive(Debug, Copy, Clone)]
struct N {
    alltypes: i32,
}

#[derive(Debug, Copy, Clone)]
struct NI {
    r#type: i32,
    intnode: i32,
}

#[derive(Debug, Copy, Clone)]
struct NF {
    r#type: i32,
    doublenode: f64,
}

fn main() -> i32 {
    let mut u = U { nf: NF { r#type: 1, doublenode: 3.14 } };

    unsafe {
        if u.n.alltypes != 1 {
            return 1;
        }
        if u.ni.r#type != 1 {
            return 1;
        }
        if u.nf.r#type != 1 {
            return 2;
        }
        if u.nf.doublenode != 3.14 {
            return 3;
        }
    }

    0
}