fn main() {
    let mut x = 5;

    let y = x.wrapping_sub(1);
    if y != 4 {
        std::process::exit(1);
    }
    if x != 4 {
        std::process::exit(2);
    }

    x = 10;
    let y = x.wrapping_sub(1);
    let mut z = 10;
    z -= 1;
    if y != 9 {
        std::process::exit(3);
    }
    if x != 9 {
        std::process::exit(4);
    }
    if z != 9 {
        std::process::exit(5);
    }

    std::process::exit(0);
}