fn main() {
    let x = 7;
    let p = &x;

    if !std::ptr::eq(&x, p) {
        std::process::exit(1);
    }

    if !std::ptr::eq(&*p, p) {
        std::process::exit(2);
    }

    let a = [10, 20, 30];

    let p0 = a.as_ptr();
    if !std::ptr::eq(&a[0], p0) {
        std::process::exit(3);
    }

    let p1 = p0.wrapping_add(1);
    if !std::ptr::eq(&a[1], p1) {
        std::process::exit(4);
    }

    let p2 = p0.wrapping_add(2);
    if !std::ptr::eq(&a[2], p2) {
        std::process::exit(5);
    }

    if a[1] != 20 {
        std::process::exit(6);
    }

    std::process::exit(0);
}