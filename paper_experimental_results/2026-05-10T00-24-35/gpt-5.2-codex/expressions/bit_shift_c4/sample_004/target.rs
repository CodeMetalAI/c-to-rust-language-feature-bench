use std::process::exit;

fn main() {
    let x: u32 = 1u32;

    let y = x << 3;
    if y != 8u32 {
        exit(1);
    }

    if (0u32 << 5) != 0u32 {
        exit(2);
    }

    if (2u32 << 2) != 8u32 {
        exit(3);
    }

    exit(0);
}