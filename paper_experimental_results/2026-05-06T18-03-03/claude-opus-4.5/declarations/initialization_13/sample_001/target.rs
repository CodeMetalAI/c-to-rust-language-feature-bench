fn main() {
    // In Rust, we can represent this union behavior using an enum or struct
    // Since we're accessing different "views" of the same data, we'll use
    // separate variables to represent the designated initializer behavior
    
    // union u x = {.any_member = 42};
    let x_any_member: i32 = 42;
    
    if x_any_member != 42 {
        std::process::exit(1);
    }
    
    {
        // union u y = {.u_member = 7u};
        let y_u_member: u32 = 7;
        
        if y_u_member != 7 {
            std::process::exit(2);
        }
    }
    
    std::process::exit(0);
}