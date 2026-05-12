fn main() {
    let x: u32 = 1;
    let y = x << 3;
    if y != 8 {
        std::process::exit(1);
    }
    if (0u32 << 5) != 0 {
        std::process::exit(2);
    }
    if (2u32 << 2) != 8 {
        std::process::exit(3);
    }
    std::process::exit(0);
}