#[derive(Debug)]
union U {
    n: Nested1,
    ni: Nested2,
    nf: Nested3,
}

#[derive(Debug)]
struct Nested1 {
    alltypes: i32,
}

#[derive(Debug)]
struct Nested2 {
    type_: i32,
    intnode: i32,
}

#[derive(Debug)]
struct Nested3 {
    type_: i32,
    doublenode: f64,
}

fn main() {
    let mut u = U { n: Nested1 { alltypes: 0 }, ni: Nested2 { type_: 0, intnode: 0 }, nf: Nested3 { type_: 0, doublenode: 0.0 } };

    // Accessing union members safely requires casting
    unsafe {
        (*(&mut u as *mut U)).nf.type_ = 1;
        (*(&mut u as *mut U)).nf.doublenode = 3.14;
    }

    if u.n.alltypes != 1 {
        return 1;
    }
    if u.ni.type_ != 1 {
        return 1;
    }
    if u.nf.type_ != 1 {
        return 2;
    }
    if u.nf.doublenode != 3.14 {
        return 3;
    }

    0
}