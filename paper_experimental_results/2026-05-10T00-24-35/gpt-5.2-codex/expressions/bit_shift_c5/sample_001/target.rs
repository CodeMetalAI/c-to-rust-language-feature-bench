fn main() {
    let x: u32 = 16u32;

    let y = x >> 2;
    if y != 4u32 {
        std::process::exit(1);
    }

    if (0u32 >> 5) != 0u32 {
        std::process::exit(2);
    }

    if (8u32 >> 1) != 4u32 {
        std::process::exit(3);
    }

    std::process::exit(0);
}