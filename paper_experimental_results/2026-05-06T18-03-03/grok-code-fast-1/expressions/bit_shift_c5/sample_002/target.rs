use std::process;

fn main() {
    let x: u32 = 16;
    let y: u32 = x >> 2;
    if y != 4 {
        process::exit(1);
    }
    if (0u32 >> 5) != 0 {
        process::exit(2);
    }
    if (8u32 >> 1) != 4 {
        process::exit(3);
    }
    process::exit(0);
}