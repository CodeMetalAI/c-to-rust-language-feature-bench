fn main() {
    let x = 7;
    let p = &x;
    if p != &x {
        std::process::exit(1);
    }
    if &(*p) != p {
        std::process::exit(2);
    }
    let a = [10, 20, 30];
    if a[1] != 20 {
        std::process::exit(6);
    }
    std::process::exit(0);
}