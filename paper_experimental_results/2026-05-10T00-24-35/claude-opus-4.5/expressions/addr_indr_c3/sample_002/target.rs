fn main() {
    let x = 7;
    let p = &x;

    if std::ptr::addr_of!(x) != p as *const i32 {
        std::process::exit(1);
    }

    if std::ptr::addr_of!(*p) != p as *const i32 {
        std::process::exit(2);
    }

    let a: [i32; 3] = [10, 20, 30];

    if &a[0] as *const i32 != a.as_ptr().wrapping_add(0) {
        std::process::exit(3);
    }
    if &a[1] as *const i32 != a.as_ptr().wrapping_add(1) {
        std::process::exit(4);
    }
    if &a[2] as *const i32 != a.as_ptr().wrapping_add(2) {
        std::process::exit(5);
    }

    if a[1] != 20 {
        std::process::exit(6);
    }

    std::process::exit(0);
}