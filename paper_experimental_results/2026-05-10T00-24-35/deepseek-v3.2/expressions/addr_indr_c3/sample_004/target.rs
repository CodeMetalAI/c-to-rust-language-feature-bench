fn main() {
    let x = 7;
    let p = &x;

    if (&x as *const i32) != (p as *const i32) {
        return std::process::exit(1);
    }

    if (&(*p) as *const i32) != (p as *const i32) {
        return std::process::exit(2);
    }

    let a = [10, 20, 30];

    if (&a[0] as *const i32) != (a.as_ptr()) {
        return std::process::exit(3);
    }
    if (&a[1] as *const i32) != (a.as_ptr().add(1)) {
        return std::process::exit(4);
    }
    if (&a[2] as *const i32) != (a.as_ptr().add(2)) {
        return std::process::exit(5);
    }

    if a[1] != 20 {
        return std::process::exit(6);
    }

    std::process::exit(0);
}