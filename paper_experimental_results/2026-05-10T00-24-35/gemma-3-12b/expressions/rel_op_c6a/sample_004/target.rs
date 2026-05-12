fn main() {
    let mut a = 1;
    let mut b = 2;
    let mut c = 2;

    if !((a < b) && (b < c)) {
        std::process::exit(1);
    }

    if !(((a < b) && (b < c))) {
        std::process::exit(2);
    }

    a = 3;
    b = 2;
    c = 2;

    if !((a < b) && (b < c)) {
        std::process::exit(3);
    }

    std::process::exit(0);
}