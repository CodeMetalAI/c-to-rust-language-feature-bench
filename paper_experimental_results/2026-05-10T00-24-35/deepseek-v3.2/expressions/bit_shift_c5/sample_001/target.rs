fn main() {
    let x: u32 = 16;
    let y = x >> 2;
    if y != 4 {
        return 1;
    }
    if (8u32 >> 1) != 4 {
        return 2;
    }
    return 0;
}

fn main() {
    std::process::exit(main());
}