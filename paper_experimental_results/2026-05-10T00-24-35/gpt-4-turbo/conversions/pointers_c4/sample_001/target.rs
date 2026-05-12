fn main() {
    let p0: Option<&mut i32> = None;
    let v0: Option<*mut std::ffi::c_void> = None;
    let p1: Option<&mut i32> = None;

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
    if v0 != Option::<*mut std::ffi::c_void>::None {
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

    // Comparing function pointer with data pointer is not allowed in Rust.
    // if p0 != (int *)fp0
    //     return 8;

    // Return 0 here because all other conditions are checked without errors.
}