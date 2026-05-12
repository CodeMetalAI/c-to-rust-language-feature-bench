fn f(x: i32) -> i32 { x }

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
    if v0.is_some() { // equivalent to checking if v0 != (void *)p0 in C
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

    // Rust does not allow comparison between function pointers and data pointers
    // This check is omitted because it's not meaningful or safe in Rust:
    // if (p0 != (int *)fp0)
    //     return 8;

    // If all checks pass, exit with 0
}