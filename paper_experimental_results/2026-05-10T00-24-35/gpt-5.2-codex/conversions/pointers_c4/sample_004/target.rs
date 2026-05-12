#[derive(Copy, Clone, PartialEq, Eq)]
struct Ptr {
    is_null: bool,
}

impl Ptr {
    fn null() -> Self {
        Ptr { is_null: true }
    }
}

fn f(x: i32) -> i32 {
    x
}

fn main() {
    let p0: Ptr = Ptr::null();
    let v0: Ptr = Ptr::null();
    let p1: Ptr = v0;

    let fp0: Ptr = Ptr::null();
    let fp1: Ptr = Ptr::null();
    let fp2: Ptr = v0;

    if p0 != p1 {
        std::process::exit(1);
    }
    if p0 != Ptr::null() {
        std::process::exit(2);
    }

    if v0 != Ptr::null() {
        std::process::exit(3);
    }
    if v0 != p0 {
        std::process::exit(4);
    }

    if fp0 != fp1 {
        std::process::exit(5);
    }
    if fp1 != fp2 {
        std::process::exit(6);
    }
    if fp0 != Ptr::null() {
        std::process::exit(7);
    }

    if p0 != fp0 {
        std::process::exit(8);
    }

    // call f to avoid unused warning (not required for behavior)
    let _ = f(0);

    std::process::exit(0);
}