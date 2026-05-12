fn main() {
    // In Rust, we need to simulate the union behavior
    // The union has overlapping fields where:
    // - n.alltypes, ni.type, and nf.type all share the same memory location (first 4 bytes)
    // - ni.intnode occupies bytes 4-7
    // - nf.doublenode occupies bytes 8-15 (with padding, or bytes 4-11 depending on alignment)
    
    // Since this is safe Rust, we'll use an enum or struct approach
    // But to preserve exact behavior, we need to track that setting nf.type = 1
    // means n.alltypes and ni.type also read as 1
    
    // The C code sets nf.type = 1 and nf.doublenode = 3.14
    // Then checks that n.alltypes == 1, ni.type == 1, nf.type == 1, nf.doublenode == 3.14
    
    // In safe Rust, we can simulate this by using a struct that represents
    // the common layout. Since type is always at offset 0 and is shared,
    // and doublenode is only accessed through nf:
    
    struct U {
        type_field: i32,      // shared by all: n.alltypes, ni.type, nf.type
        _intnode: i32,        // ni.intnode (not used in this program)
        doublenode: f64,      // nf.doublenode
    }
    
    let mut u = U {
        type_field: 0,
        _intnode: 0,
        doublenode: 0.0,
    };
    
    // u.nf.type = 1;
    u.type_field = 1;
    
    // u.nf.doublenode = 3.14;
    u.doublenode = 3.14;
    
    // if (u.n.alltypes != 1) return 1;
    if u.type_field != 1 {
        std::process::exit(1);
    }
    
    // if (u.ni.type != 1) return 1;
    if u.type_field != 1 {
        std::process::exit(1);
    }
    
    // if (u.nf.type != 1) return 2;
    if u.type_field != 1 {
        std::process::exit(2);
    }
    
    // if (u.nf.doublenode != 3.14) return 3;
    if u.doublenode != 3.14 {
        std::process::exit(3);
    }
    
    // return 0;
    std::process::exit(0);
}