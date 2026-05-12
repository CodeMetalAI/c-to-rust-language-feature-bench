use std::process::exit;

fn main() {
    let x: u32 = 1;

    let y: u32 = x << 3;
    if y != 8 {
        exit(1);
    }

    if (0u32 << 5) != 0 {
        exit(2);
    }

    if (2u32 << 2) != 8 {
        exit(3);
    }

    exit(0);
}