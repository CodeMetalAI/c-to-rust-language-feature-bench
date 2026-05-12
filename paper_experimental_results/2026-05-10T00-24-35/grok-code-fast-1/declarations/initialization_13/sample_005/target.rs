fn main() {
    // Simulate union u with [u8; 4], initialized via any_member which is i32
    let x_bytes: [u8; 4] = 42i32.to_ne_bytes();
    let x_any_member = i32::from_ne_bytes(x_bytes);
    if x_any_member != 42 {
        std::process::exit(1);
    }

    {
        // Simulate union u with [u8; 4], initialized via u_member which is u32
        let y_bytes: [u8; 4] = 7u32.to_ne_bytes();
        let y_u_member = u32::from_ne_bytes(y_bytes);
        if y_u_member != 7 {
            std::process::exit(2);
        }
    }

    std::process::exit(0);
}