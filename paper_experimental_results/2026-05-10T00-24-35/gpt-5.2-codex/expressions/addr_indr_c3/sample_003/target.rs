use std::ptr;

fn main() {
    let x: i32 = 7;
    let p: &i32 = &x;

    if !ptr::eq(p, &x) {
        std::process::exit(1);
    }

    if !ptr::eq(p, &*p) {
        std::process::exit(2);
    }

    let a: [i32; 3] = [10, 20, 30];
    let ap = a.as_ptr();

    if !ptr::eq(ap.wrapping_add(0), &a[0] as *const i32) {
        std::process::exit(3);
    }
    if !ptr::eq(ap.wrapping_add(1), &a[1] as *const i32) {
        std::process::exit(4);
    }
    if !ptr::eq(ap.wrapping_add(2), &a[2] as *const i32) {
        std::process::exit(5);
    }

    if a[1] != 20 {
        std::process::exit(6);
    }

    std::process::exit(0);
}