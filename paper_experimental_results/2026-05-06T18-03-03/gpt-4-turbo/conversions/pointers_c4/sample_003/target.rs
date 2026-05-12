fn main() {
    let p0: *const i32 = std::ptr::null();
    let v0: *const () = std::ptr::null();
    let p1: *const i32 = v0 as *const i32;

    let fp0: Option<fn(i32) -> i32> = None;
    let fp1: Option<fn(i32) -> i32> = None;
    let fp2: Option<fn(i32) -> i32> = v0 as *const ();

    if p0 != p1 {
        std::process::exit(1);
    }
    if !p0.is_null() {
        std::process::exit(2);
    }

    if !v0.is_null() {
        std::process::exit(3);
    }
    if v0 != p0 as *const () {
        std::process::exit(4);
    }

    if fp0 != fp1 {
        std::process::exit(5);
    }
    if fp1 != fp2 {
        std::process::exit(6);
    }
    if fp0.is_some() {
        std::process::exit(7);
    }

    if p0 as *const () != fp0.map(|f| f as *const ()) {
        std::process::exit(8);
    }

    // If all checks pass, return 0
}