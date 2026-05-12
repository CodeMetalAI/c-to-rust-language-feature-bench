use std::process::exit;

fn main() {
    let x: i32 = 1 - 1;
    let y: i32 = 3;

    if x != 0 {
        exit(1);
    }
    if y != 3 {
        exit(2);
    }

    exit(0);
}