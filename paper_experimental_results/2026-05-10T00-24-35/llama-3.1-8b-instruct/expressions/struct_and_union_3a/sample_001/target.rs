// A program to demonstrate the usage of union in Rust
union U {
    struct {
        i32 alltypes: 32;
    } n;
    struct {
        i32 type_: 32;
        i32 intnode: 32;
    } ni;
    struct {
        i32 type_: 32;
        f64 doublenode: 64;
    } nf;
}

fn main() {
    let mut u = U {
        nf: U::nf {
            doublenode: 3.14,
            type_: 1,
        },
    };

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

    return 0;
}