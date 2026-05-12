fn main() {
    let x: u32 = 16;

    let y = x >> 2;
    if y!= 4 {
        std::process::exit(1);
    }

    if 0u32 >> 5!= 0 {
        std::process::exit(2);
    }

    if 8u32 >> 1!= 4 {
        std::process::exit(3);
    }

    std::process::exit(0);
}