fn main() {
    let x_any: i32 = 42;
    if x_any != 42 {
        std::process::exit(1);
    }

    {
        let y_u: u32 = 7;
        if y_u != 7 {
            std::process::exit(2);
        }
    }

    std::process::exit(0);
}