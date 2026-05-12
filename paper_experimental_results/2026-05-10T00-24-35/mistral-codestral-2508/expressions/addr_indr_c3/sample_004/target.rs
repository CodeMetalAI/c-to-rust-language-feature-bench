fn main() {
    let x = 7;
    let p = &x;

    if std::ptr::addr_of!(x) != p {
        std::process::exit(1);
    }

    if std::ptr::addr_of!(*p) != p {
        std::process::exit(2);
    }

    let a = [10, 20, 30];

    if std::ptr::addr_of!(a[0]) != a.as_ptr() {
        std::process::exit(3);
    }
    if std::ptr::addr_of!(a[1]) != unsafe { a.as_ptr().add(1) } {
        std::process::exit(4);
    }
    if std::ptr::addr_of!(a[2]) != unsafe { a.as_ptr().add(2) } {
        std::process::exit(5);
    }

    if unsafe { *a.as_ptr().add(1) } != 20 {
        std::process::exit(6);
    }
}