fn main() {
    let arr = [1, 2, 3, 4, 5];
    let n = arr.len();

    if n != 5 {
        std::process::exit(1);
    }

    std::process::exit(0);
}