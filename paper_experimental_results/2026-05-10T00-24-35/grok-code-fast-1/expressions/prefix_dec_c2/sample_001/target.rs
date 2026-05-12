use std::process;

fn main() {
    let mut x = 5;
    x -= 1;
    let y = x;
    if y != 4 {
        process::exit(1);
    }
    if x != 4 {
        process::exit(2);
    }

    x = 10;
    x -= 1;
    let y = x;
    let mut z = 10;
    z -= 1;
    if y != 9 {
        process::exit(3);
    }
    if x != 9 {
        process::exit(4);
    }
    if z != 9 {
        process::exit(5);
    }
}