fn main() {
    let x: i32 = 42;
    if x != 42 {
        std::process::exit(1);
    }
    {
        let y: u32 = 7;
        if y != 7 {
            std::process::exit(2);
        }
    }
    std::process::exit(0);
}