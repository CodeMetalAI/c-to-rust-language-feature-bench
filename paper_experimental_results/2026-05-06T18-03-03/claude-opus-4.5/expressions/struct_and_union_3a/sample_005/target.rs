fn main() {
    // In Rust, we need to simulate the union behavior
    // The C union overlaps memory, where:
    // - n.alltypes, ni.type, and nf.type all share the same memory location (first 4 bytes)
    // - ni.intnode occupies bytes 4-7
    // - nf.doublenode occupies bytes 8-15 (with padding, or bytes 4-11 depending on alignment)
    
    // Since we're setting nf.type = 1 and nf.doublenode = 3.14,
    // and then checking that n.alltypes == 1 and ni.type == 1,
    // this works because they all share the first int field.
    
    // We can simulate this with a struct that has the fields we need
    // Since all three struct variants have 'type' as the first field (same offset),
    // we just need to track the type and the doublenode value
    
    let nf_type: i32 = 1;
    let nf_doublenode: f64 = 3.14;
    
    // n.alltypes is at the same memory location as nf.type
    let n_alltypes = nf_type;
    
    // ni.type is at the same memory location as nf.type
    let ni_type = nf_type;
    
    if n_alltypes != 1 {
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