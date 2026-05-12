// Define a union type U with three different structs
union U {
    struct {
        alltypes: i32,
    } n;
    struct {
        type_: i32,
        intnode: i32,
    } ni;
    struct {
        type_: i32,
        doublenode: f64,
    } nf;
}

fn main() {
    // Create a new instance of the union U
    let mut u = U { n: Default::default(), ni: Default::default(), nf: Default::default() };

    // Assign values to the fields of the union
    u.nf.type_ = 1;
    u.nf.doublenode = 3.14;

    // Check the values of the fields
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

    // If all checks pass, return 0
    return 0;
}