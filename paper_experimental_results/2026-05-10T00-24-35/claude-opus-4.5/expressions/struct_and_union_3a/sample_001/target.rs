fn main() {
    // In Rust, we simulate the union behavior using a struct that can hold all variants
    // Since the C code relies on type punning through union, we need to carefully
    // replicate the memory layout behavior
    
    // The union has overlapping fields:
    // - n.alltypes overlaps with ni.type and nf.type (all at offset 0, all i32)
    // - ni.intnode is at offset 4 (i32)
    // - nf.doublenode is at offset 8 (f64, due to alignment)
    
    // Since we're in safe Rust, we'll use an enum-like approach but track the values
    // that would be visible through the union's type punning
    
    // The key insight: in C, u.n.alltypes, u.ni.type, and u.nf.type all share the same memory
    // So setting u.nf.type = 1 means u.n.alltypes and u.ni.type also read as 1
    
    let mut type_field: i32 = 0;
    let mut doublenode: f64 = 0.0;
    
    // u.nf.type = 1
    type_field = 1;
    
    // u.nf.doublenode = 3.14
    doublenode = 3.14;
    
    // if (u.n.alltypes != 1) - alltypes shares memory with type
    if type_field != 1 {
        std::process::exit(1);
    }
    
    // if (u.ni.type != 1) - ni.type shares memory with type
    if type_field != 1 {
        std::process::exit(1);
    }
    
    // if (u.nf.type != 1)
    if type_field != 1 {
        std::process::exit(2);
    }
    
    // if (u.nf.doublenode != 3.14)
    if doublenode != 3.14 {
        std::process::exit(3);
    }
    
    std::process::exit(0);
}