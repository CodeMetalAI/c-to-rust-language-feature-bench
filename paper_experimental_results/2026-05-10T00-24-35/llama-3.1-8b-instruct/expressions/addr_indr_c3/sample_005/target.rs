fn main() {
    let x = 7;
    let p = &x;

    if &x != p {
        std::process::exit(1);
    }

    let y = *p;
    let q = &y;

    if &y != q {
        std::process::exit(2);
    }

    let a = [10, 20, 30];

    if &a[0] != &a[0] {
        std::process::exit(3);
    }
    if &a[1] != &a[1] {
        std::process::exit(4);
    }
    if &a[2] != &a[2] {
        std::process::exit(5);
    }

    if &a[1] != &a[1] {
        std::process::exit(6);
    }

    std::process::exit(0);
}