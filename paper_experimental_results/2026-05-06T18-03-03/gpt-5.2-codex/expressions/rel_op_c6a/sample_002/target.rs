use std::process::exit;

fn main() {
    let mut a = 1;
    let mut b = 2;
    let mut c = 2;

    if !(((a < b) as i32) < c) {
        exit(1);
    }

    if !((((a < b) as i32) < c)) {
        exit(2);
    }

    a = 3;
    b = 2;
    c = 2;

    if !(((a < b) as i32) < c) {
        exit(3);
    }

    exit(0);
}