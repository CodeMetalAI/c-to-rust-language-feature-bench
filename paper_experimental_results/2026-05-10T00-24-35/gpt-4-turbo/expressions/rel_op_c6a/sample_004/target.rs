fn main() {
    let mut a = 1;
    let b = 2;
    let c = 2;

    if !((a < b) && (b < c)) {
        std::process::exit(1);
    }

    if !((a < b) < c) {
        std::process::exit(2);
    }

    a = 3;

    if !((a < b) && (b < c)) {
        std::process::exit(3);
    }
}