use std::process::exit;

fn main() {
    let x_any = 42;
    if x_any != 42 {
        exit(1);
    }

    {
        let y_u: u32 = 7;
        if y_u != 7 {
            exit(2);
        }
    }
}