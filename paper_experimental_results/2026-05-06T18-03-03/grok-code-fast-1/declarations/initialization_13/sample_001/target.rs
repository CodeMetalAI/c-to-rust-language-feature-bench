use std::process;

fn main() {
    let x_any_member: i32 = 42;
    if x_any_member != 42 {
        process::exit(1);
    }

    {
        let y_u_member: u32 = 7;
        if y_u_member != 7 {
            process::exit(2);
        }
    }

    process::exit(0);
}