fn f(x: i32) -> i32 {
    x
}

fn main() {
    let p0: Option<&i32> = None;
    let v0: Option<()> = None;
    let p1: Option<&i32> = None;

    let fp0: Option<fn(i32) -> i32> = None;
    let fp1: Option<fn(i32) -> i32> = None;
    let fp2: Option<fn(i32) -> i32> = None;

    if p0 != p1 {
        std::process::exit(1);
    }
    if p0.is_some() {
        std::process::exit(2);
    }

    if v0.is_some() {
        std::process::exit(3);
    }
    if v0 != None {
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

    // Not directly comparable in Rust: if (p0 != (int *)fp0)
    // We use `std::ptr::null` for comparison but this is not idiomatic or recommended in safe Rust.
    let p0_raw: *const i32 = std::ptr::null();
    let fp0_raw: *const () = std::ptr::null();
    if p0_raw != fp0_raw as *const i32 {
        std::process::exit(8);
    }
}