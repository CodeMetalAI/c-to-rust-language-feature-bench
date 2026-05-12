fn main() {
    let x = 1 - 1;
    let y = 3;

    if x != 0 {
        std::process::exit(1);
    }
    if y != 3 {
        std::process::exit(2);
    }
}