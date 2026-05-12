fn main() {
    let mut x = 41;

    let old = x;
    x += 1;
    if old != 41 {
        std::process::exit(1);
    }
    if x != 42 {
        std::process::exit(2);
    }

    let old = x;
    x += 1;
    if old != 42 {
        std::process::exit(3);
    }
    if x != 43 {
        std::process::exit(4);
    }

    x = 100;
    let y = x;
    x += 1;
    if y != 100 {
        std::process::exit(5);
    }
    if x != 101 {
        std::process::exit(6);
    }

    std::process::exit(0);
}