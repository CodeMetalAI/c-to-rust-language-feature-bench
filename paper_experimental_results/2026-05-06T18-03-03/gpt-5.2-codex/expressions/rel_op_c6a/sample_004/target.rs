use std::process::exit;

fn main() {
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    let mut c: i32 = 2;

    let ab = if a < b { 1 } else { 0 };
    if !((ab) < c) {
        exit(1);
    }

    let ab2 = if a < b { 1 } else { 0 };
    if !((ab2) < c) {
        exit(2);
    }

    a = 3;
    b = 2;
    c = 2;

    let ab3 = if a < b { 1 } else { 0 };
    if !((ab3) < c) {
        exit(3);
    }

    exit(0);
}