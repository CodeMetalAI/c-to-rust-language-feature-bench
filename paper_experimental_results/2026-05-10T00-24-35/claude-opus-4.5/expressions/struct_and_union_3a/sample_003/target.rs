fn main() {
    // In Rust, we can't have true unions with type punning in safe code.
    // However, we can simulate the behavior by using a struct that holds
    // the overlapping fields. Since the C code relies on the fact that
    // 'alltypes', 'ni.type', and 'nf.type' all occupy the same memory location
    // (the first int in the union), we need to track this.
    
    // The C union has:
    // - n.alltypes (int at offset 0)
    // - ni.type (int at offset 0), ni.intnode (int at offset 4)
    // - nf.type (int at offset 0), nf.doublenode (double at offset 8, due to alignment)
    
    // Since we're setting nf.type = 1 and nf.doublenode = 3.14,
    // and then reading n.alltypes and ni.type (which alias nf.type),
    // we need to ensure they all see the same value.
    
    // In safe Rust, we'll use an enum or a struct to represent this.
    // Since the checks verify that type fields are all 1 (same memory),
    // we can use a single struct with the shared type field.
    
    struct U {
        type_field: i32,      // This is n.alltypes, ni.type, and nf.type
        doublenode: f64,      // This is nf.doublenode
    }
    
    let mut u = U {
        type_field: 0,
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