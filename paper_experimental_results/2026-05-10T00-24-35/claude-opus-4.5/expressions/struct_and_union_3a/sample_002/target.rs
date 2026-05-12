fn main() {
    // In Rust, we need to simulate the union behavior
    // The C union overlaps memory, where:
    // - n.alltypes, ni.type, and nf.type all share the first 4 bytes
    // - ni.intnode occupies bytes 4-7
    // - nf.doublenode occupies bytes 8-15 (with padding, double is at offset 8)
    
    // Since we're in safe Rust, we'll use a struct that mimics the layout
    // The key behavior is:
    // - Setting nf.type = 1 should make n.alltypes and ni.type also equal 1
    // - These all refer to the same memory location (first int field)
    
    // We can represent this with a single struct since all type fields overlap
    struct U {
        type_field: i32,  // This is alltypes, ni.type, and nf.type (all overlapping)
        _padding: i32,    // ni.intnode would be here
        doublenode: f64,  // nf.doublenode
    }
    
    let mut u = U {
        type_field: 0,
        _padding: 0,
        doublenode: 0.0,
    };
    
    // u.nf.type = 1
    u.type_field = 1;
    
    // u.nf.doublenode = 3.14
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