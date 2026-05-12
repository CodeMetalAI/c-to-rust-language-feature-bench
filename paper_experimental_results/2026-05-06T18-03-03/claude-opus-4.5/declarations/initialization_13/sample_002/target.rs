fn main() {
    // In Rust, we can represent this union behavior using an enum or struct
    // Since we're only reading the same field we write to, we can use a simple approach
    
    // First case: any_member = 42
    let x_any_member: i32 = 42;
    
    if x_any_member != 42 {
        std::process::exit(1);
    }
    
    // Second case: u_member = 7
    {
        let y_u_member: u32 = 7;
        if y_u_member != 7 {
            std::process::exit(2);
        }
    }
    
    std::process::exit(0);
}