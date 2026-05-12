fn main() {
    let a = [1, 2, 3, 4, 5];
    let n = a.len();

    if n!= 5 {
        std::process::exit(1);
    }

    std::process::exit(0);
}