use std::mem;

#[repr(C)]
struct StructAllTypes {
    alltypes: i32,
}

#[repr(C)]
struct StructIntNode {
    type_: i32,
    intnode: i32,
}

#[repr(C)]
struct StructDoubleNode {
    type_: i32,
    doublenode: f64,
}

union U {
    n: StructAllTypes,
    ni: StructIntNode,
    nf: StructDoubleNode,
}

fn main() {
    let mut u = unsafe { mem::zeroed::<U>() };

    unsafe {
        u.nf.type_ = 1;
        u.nf.doublenode = 3.14;

        if u.n.alltypes != 1 {
            std::process::exit(1);
        }
        if u.ni.type_ != 1 {
            std::process::exit(1);
        }
        if u.nf.type_ != 1 {
            std::process::exit(2);
        }
        if u.nf.doublenode != 3.14 {
            std::process::exit(3);
        }
    }

    std::process::exit(0);
}