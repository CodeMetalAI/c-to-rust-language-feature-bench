fn main() {
    let mut x = 5i32;
    x -= 1;
    let y = x;
    if y != 4 {
        std::process::exit(1);
    }
    if x != 4 {
        std::process::exit(2);
    }
    x = 10;
    x -= 1;
    let y = x;
    let mut z = 10i32;
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
}