use std::process::exit;

fn main() {
    let x: i32 = 7;
    let p: &i32 = &x;

    if !std::ptr::eq(&x, p) {
        exit(1);
    }

    if !std::ptr::eq(&*p, p) {
        exit(2);
    }

    let a: [i32; 3] = [10, 20, 30];

    if !std::ptr::eq(&a[0], &a[0]) {
        exit(3);
    }
    if !std::ptr::eq(&a[1], &a[1]) {
        exit(4);
    }
    if !std::ptr::eq(&a[2], &a[2]) {
        exit(5);
    }

    if a[1] != 20 {
        exit(6);
    }

    exit(0);
}