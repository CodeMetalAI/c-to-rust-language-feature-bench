fn main() {
    let mut x = 5;

    let y = {
        x += 1;
        x
    };

    if y != 6 {
        std::process::exit(1);
    }
    if x != 6 {
        std::process::exit(2);
    }

    x = 10;
    let y = {
        x += 1;
        x
    };
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