use std::process;

fn main() {
    let (mut a, mut b, mut c, mut d): (i32, i32, i32, i32);

    a = 1;
    b = 2;
    c = 3;
    d = 4;
    if ((a < b) == (c < d)) != true {
        process::exit(1);
    }

    a = 2;
    b = 1;
    c = 4;
    d = 3;
    if ((a < b) == (c < d)) != true {
        process::exit(2);
    }

    a = 1;
    b = 2;
    c = 4;
    d = 3;
    if ((a < b) == (c < d)) != false {
        process::exit(3);
    }

    a = 2;
    b = 1;
    c = 3;
    d = 4;
    if ((a < b) == (c < d)) != false {
        process::exit(4);
    }

    a = 1;
    b = 2;
    c = 3;
    d = 4;
    if ((a < b) == (c < d)) != true {
        process::exit(5);
    }
}