use std::process::exit;

fn main() {
    let mut x: i32 = 5;

    x += 1;
    let y = x;
    if y != 6 {
        exit(1);
    }
    if x != 6 {
        exit(2);
    }

    x = 10;
    x += 1;
    let y = x;
    let mut z: i32 = 10;
    z += 1;
    if y != 11 {
        exit(3);
    }
    if z != 11 {
        exit(4);
    }
    if z != 11 {
        exit(5);
    }

    exit(0);
}