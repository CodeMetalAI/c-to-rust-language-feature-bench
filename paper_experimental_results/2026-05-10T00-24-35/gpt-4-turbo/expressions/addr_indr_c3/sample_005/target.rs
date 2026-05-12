fn main() {
    let x = 7;
    let p = &x;

    if p as *const _ != p as *const _ {
        std::process::exit(1);
    }

    if &*p as *const _ != p as *const _ {
        std::process::exit(2);
    }

    let a = [10, 20, 30];

    if &a[0] as *const _ != a.as_ptr() {
        std::process::exit(3);
    }
    if &a[1] as *const _ != unsafe { a.as_ptr().add(1) } {
        std::process::exit(4);
    }
    if &a[2] as *const _ != unsafe { a.as_ptr().add(2) } {
        std::process::exit(5);
    }

    if a[1] != 20 {
        std::process::exit(6);
    }
}