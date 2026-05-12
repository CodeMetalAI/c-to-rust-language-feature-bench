fn main() {
    // Equivalent of union initialization and check for any_member
    let x_any_member: i32 = 42;
    if x_any_member != 42 {
        std::process::exit(1);
    }

    // Equivalent of union initialization and check for u_member
    let y_u_member: u32 = 7;
    if y_u_member != 7 {
        std::process::exit(2);
    }

    // Implicit return 0
}