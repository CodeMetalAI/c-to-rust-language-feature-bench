fn bool_to_int(b: bool) -> i32 {
    if b { 1 } else { 0 }
}

fn main() {
    let (mut a, mut b, mut c, mut d): (i32, i32, i32, i32);

    a = 1;
    b = 2;
    c = 3;
    d = 4;
    if bool_to_int((a < b) == (c < d)) != 1 {
        std::process::exit(1);
    }

    a = 2;
    b = 1;
    c = 4;
    d = 3;
    if bool_to_int((a < b) == (c < d)) != 1 {
        std::process::exit(2);
    }

    a = 1;
    b = 2;
    c = 4;
    d = 3;
    if bool_to_int((a < b) == (c < d)) != 0 {
        std::process::exit(3);
    }

    a = 2;
    b = 1;
    c = 3;
    d = 4;
    if bool_to_int((a < b) == (c < d)) != 0 {
        std::process::exit(4);
    }

    a = 1;
    b = 2;
    c = 3;
    d = 4;
    if bool_to_int((a < b) == (c < d)) != 1 {
        std::process::exit(5);
    }

    std::process::exit(0);
}