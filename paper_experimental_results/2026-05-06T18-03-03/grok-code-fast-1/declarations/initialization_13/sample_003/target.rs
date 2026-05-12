fn main() {
    let x_any_member: i32 = 42;
    if x_any_member != 42 {
        std::process::exit(1);
    }
    {
        let y_u_member: u32 = 7;
        if y_u_member != 7 {
            std::process::exit(2);
        }
    }
    std::process::exit(0);
}