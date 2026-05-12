use std::mem::size_of;
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

    let base = a.as_ptr() as usize;
    let sz = size_of::<i32>();

    let addr0 = &a[0] as *const i32 as usize;
    let addr1 = &a[1] as *const i32 as usize;
    let addr2 = &a[2] as *const i32 as usize;

    if addr0 != base + 0 * sz {
        exit(3);
    }
    if addr1 != base + 1 * sz {
        exit(4);
    }
    if addr2 != base + 2 * sz {
        exit(5);
    }

    if a[1] != 20 {
        exit(6);
    }

    exit(0);
}