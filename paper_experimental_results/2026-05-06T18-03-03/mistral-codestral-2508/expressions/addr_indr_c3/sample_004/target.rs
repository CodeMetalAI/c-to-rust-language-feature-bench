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

    if std::ptr::addr_of!(a[0]) != &a[0] {
        std::process::exit(3);
    }
    if std::ptr::addr_of!(a[1]) != &a[1] {
        std::process::exit(4);
    }
    if std::ptr::addr_of!(a[2]) != &a[2] {
        std::process::exit(5);
    }

    if a[1] != 20 {
        std::process::exit(6);
    }
}