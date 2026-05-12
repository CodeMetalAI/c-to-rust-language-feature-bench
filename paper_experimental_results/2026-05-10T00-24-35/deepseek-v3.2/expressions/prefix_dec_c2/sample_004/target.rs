fn main() {
    let mut x = 5;
    
    x -= 1;
    let y = x;
    if y != 4 {
        return std::process::exit(1);
    }
    if x != 4 {
        return std::process::exit(2);
    }

    x = 10;
    x -= 1;
    let y = x;
    let mut z = 10;
    z -= 1;
    if y != 9 {
        return std::process::exit(3);
    }
    if x != (9) {
        return std::process::exit(4);
    }
    if z != 9 {
        return std::process::exit(5);
    }

    std::process::exit(0);
}