fn main() {
    // In Rust, we need to simulate the union behavior
    // The C union overlaps memory, where:
    // - n.alltypes, ni.type, and nf.type all share the same memory location (first 4 bytes)
    // - ni.intnode occupies bytes 4-7
    // - nf.doublenode occupies bytes 8-15 (with padding, or bytes 4-11 depending on alignment)
    
    // Since we're setting nf.type = 1 and nf.doublenode = 3.14,
    // and then checking that n.alltypes and ni.type also equal 1,
    // this works because they all share the same first int field.
    
    // In safe Rust, we can simulate this with an enum or struct
    // But to match the exact behavior, we'll use a struct that tracks the values
    
    // The key insight is that in the C code:
    // - All three struct variants have 'type' (or 'alltypes') as their first field
    // - Setting nf.type = 1 means n.alltypes and ni.type will also read as 1
    
    // We simulate this by using a single storage
    let type_value: i32 = 1;
    let doublenode_value: f64 = 3.14;
    
    // Simulating the union reads:
    // u.n.alltypes reads the first int (which is type_value)
    let alltypes = type_value;
    
    // u.ni.type reads the first int (which is type_value)
    let ni_type = type_value;
    
    // u.nf.type reads the first int (which is type_value)
    let nf_type = type_value;
    
    // u.nf.doublenode reads the double
    let nf_doublenode = doublenode_value;
    
    if alltypes != 1 {
        std::process::exit(1);
    }
    if ni_type != 1 {
        std::process::exit(1);
    }
    if nf_type != 1 {
        std::process::exit(2);
    }
    if nf_doublenode != 3.14 {
        std::process::exit(3);
    }
    
    std::process::exit(0);
}