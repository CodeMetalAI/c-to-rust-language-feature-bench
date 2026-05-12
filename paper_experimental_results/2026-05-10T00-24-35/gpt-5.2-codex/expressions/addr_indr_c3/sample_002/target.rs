use std::ptr;

fn main() {
    let x = 7;
    let p = &x;

    if !ptr::eq(&x, p) {
        std::process::exit(1);
    }

    if !ptr::eq(&*p, p) {
        std::process::exit(2);
    }

    let a = [10, 20, 30];

    if !ptr::eq(&a[0], &a[0]) {
        std::process::exit(3);
    }
    if !ptr::eq(&a[1], &a[1]) {
        std::process::exit(4);
    }
    if !ptr::eq(&a[2], &a[2]) {
        std::process::exit(5);
    }

    if a[1] != 20 {
        std::process::exit(6);
    }

    std::process::exit(0);
}