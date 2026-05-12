fn main() {
    let x = 7;
    let _p = &x;

    let a = [10, 20, 30];

    if a[1] != 20 {
        std::process::exit(6);
    }

    std::process::exit(0);
}