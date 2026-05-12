use std::process::exit;

fn main() {
    let x = 7;
    let p = &x;

    if !std::ptr::eq(&x, p) {
        exit(1);
    }

    if !std::ptr::eq(&*p, p) {
        exit(2);
    }

    let a = [10, 20, 30];
    let a_ptr = a.as_ptr();

    if (&a[0] as *const i32) != a_ptr.wrapping_add(0) {
        exit(3);
    }
    if (&a[1] as *const i32) != a_ptr.wrapping_add(1) {
        exit(4);
    }
    if (&a[2] as *const i32) != a_ptr.wrapping_add(2) {
        exit(5);
    }

    if a[1] != 20 {
        exit(6);
    }

    exit(0);
}