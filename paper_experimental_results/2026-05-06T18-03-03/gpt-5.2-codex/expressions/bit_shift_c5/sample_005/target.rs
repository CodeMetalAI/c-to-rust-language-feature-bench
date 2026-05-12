use std::process::exit;

fn main() {
    let x: u32 = 16;

    let y = x >> 2;
    if y != 4 {
        exit(1);
    }

    if (0u32 >> 5) != 0 {
        exit(2);
    }

    if (8u32 >> 1) != 4 {
        exit(3);
    }

    exit(0);
}