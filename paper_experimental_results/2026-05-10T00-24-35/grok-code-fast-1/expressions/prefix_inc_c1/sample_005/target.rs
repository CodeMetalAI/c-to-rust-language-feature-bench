fn main() {
    let mut x = 5;
    x += 1;
    let y = x;
    if y != 6 {
        std::process::exit(1);
    }
    if x != 6 {
        std::process::exit(2);
    }
    x = 10;
    x += 1;
    let y = x;
    let mut z = 10;
    z += 1;
    if y != 11 {
        std::process::exit(3);
    }
    if z != 11 {
        std::process::exit(4);
    }
    if z != 11 {
        std::process::exit(5);
    }
}