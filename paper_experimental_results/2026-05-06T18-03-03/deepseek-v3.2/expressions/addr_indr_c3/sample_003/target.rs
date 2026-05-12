fn main() {
    let x = 7;
    let p = &x;

    if (&x != p) {
        std::process::exit(1);
    }

    if (&(*p) != p) {
        std::process::exit(2);
    }

    let a = [10, 20, 30];

    if (&a[0] != a.as_ptr()) {
        std::process::exit(3);
    }
    if (&a[1] != a.as_ptr().add(1)) {
        std::process::exit(4);
    }
    if (&a[2] != a.as_ptr().add(2)) {
        std::process::exit(5);
    }

    if (*a.as_ptr().add(1) != 20) {
        std::process::exit(6);
    }

    std::process::exit(0);
}