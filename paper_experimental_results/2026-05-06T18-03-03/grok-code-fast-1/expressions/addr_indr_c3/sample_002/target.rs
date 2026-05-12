fn main() {
    let x = 7;
    let p = &x as *const i32;
    if !std::ptr::eq(&x, p) {
        std::process::exit(1);
    }
    if !std::ptr::eq(unsafe { &*p }, p) {
        std::process::exit(2);
    }
    let a = [10, 20, 30];
    if !std::ptr::eq(&a[0], a.as_ptr()) {
        std::process::exit(3);
    }
    if !std::ptr::eq(&a[1], a.as_ptr().wrapping_add(1)) {
        std::process::exit(4);
    }
    if !std::ptr::eq(&a[2], a.as_ptr().wrapping_add(2)) {
        std::process::exit(5);
    }
    if unsafe { *a.as_ptr().wrapping_add(1) } != 20 {
        std::process::exit(6);
    }
    std::process::exit(0);
}