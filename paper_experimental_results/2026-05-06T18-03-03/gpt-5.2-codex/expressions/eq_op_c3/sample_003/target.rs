use std::process::exit;

fn main() {
    let (mut a, mut b, mut c, mut d);

    a = 1;
    b = 2;
    c = 3;
    d = 4;
    let v = ((a < b) == (c < d)) as i32;
    if v != 1 {
        exit(1);
    }

    a = 2;
    b = 1;
    c = 4;
    d = 3;
    let v = ((a < b) == (c < d)) as i32;
    if v != 1 {
        exit(2);
    }

    a = 1;
    b = 2;
    c = 4;
    d = 3;
    let v = ((a < b) == (c < d)) as i32;
    if v != 0 {
        exit(3);
    }

    a = 2;
    b = 1;
    c = 3;
    d = 4;
    let v = ((a < b) == (c < d)) as i32;
    if v != 0 {
        exit(4);
    }

    a = 1;
    b = 2;
    c = 3;
    d = 4;
    let v = ((a < b) == (c < d)) as i32;
    if v != 1 {
        exit(5);
    }

    exit(0);
}