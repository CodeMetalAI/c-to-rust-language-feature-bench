fn main() {
    let mut x: i32 = 5;
    // y = --x
    x -= 1;
    let y: i32 = x;
    if y != 4 {
        std::process::exit(1);
    }
    if x != 4 {
        std::process::exit(2);
    }
    x = 10;
    // y = --x
    x -= 1;
    let y: i32 = x;
    let mut z: i32 = 10;
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
    // return 0;
}