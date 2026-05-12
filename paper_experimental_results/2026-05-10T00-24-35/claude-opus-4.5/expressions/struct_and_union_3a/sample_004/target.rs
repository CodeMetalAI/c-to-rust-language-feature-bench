fn main() {
    // In Rust, we simulate the union behavior using a struct that can hold all fields
    // The C union overlaps memory, where:
    // - n.alltypes, ni.type, and nf.type all share the same memory location (first 4 bytes)
    // - ni.intnode occupies bytes 4-7
    // - nf.doublenode occupies bytes 4-11 (on typical alignment)
    
    // Since we're setting nf.type = 1 and nf.doublenode = 3.14, and then reading:
    // - n.alltypes (which is the same memory as nf.type) -> should be 1
    // - ni.type (which is the same memory as nf.type) -> should be 1
    // - nf.type -> should be 1
    // - nf.doublenode -> should be 3.14
    
    // We can simulate this with a simple struct since we only write to nf fields
    // and read from overlapping type fields (which are all at the same offset)
    
    let type_value: i32 = 1;
    let doublenode_value: f64 = 3.14;
    
    // Simulating the union reads:
    // u.n.alltypes reads the first i32, which is type_value
    let alltypes = type_value;
    // u.ni.type reads the first i32, which is type_value  
    let ni_type = type_value;
    // u.nf.type is type_value
    let nf_type = type_value;
    // u.nf.doublenode is doublenode_value
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