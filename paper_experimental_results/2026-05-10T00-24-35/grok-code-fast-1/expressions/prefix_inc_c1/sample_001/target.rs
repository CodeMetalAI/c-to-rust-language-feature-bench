use std::process;

fn main() {
    let mut x = 5;
    let mut y;
    y = {
        x += 1;
        x
    };
    if y != 6 {
        process::exit(1);
    }
    if x != 6 {
        process::exit(2);
    }

    x = 10;
    y = {
        x += 1;
        x
    };
    let mut z = 10;
    z += 1;
    if y != 11 {
        process::exit(3);
    }
    if z != 11 {
        process::exit(4);
    }
    if z != 11 {
        process::exit(5);
    }

    process::exit(0);
}