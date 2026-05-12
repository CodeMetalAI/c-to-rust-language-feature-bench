fn main() {
    let mut a: i32;
    let mut b: i32;
    let mut c: i32;
    let mut d: i32;

    a = 1;
    b = 2;
    c = 3;
    d = 4;
    if (a < b) != (c < d) {
        std::process::exit(1);
    }

    a = 2;
    b = 1;
    c = 4;
    d = 3;
    if (a < b) != (c < d) {
        std::process::exit(2);
    }

    a = 1;
    b = 2;
    c = 4;
    d = 3;
    if (a < b) == (c < d) {
        std::process::exit(3);
    }

    a = 2;
    b = 1;
    c = 3;
    d = 4;
    if (a < b) == (c < d) {
        std::process::exit(4);
    }

    a = 1;
    b = 2;
    c = 3;
    d = 4;
    if (a < b) != (c < d) {
        std::process::exit(5);
    }
}