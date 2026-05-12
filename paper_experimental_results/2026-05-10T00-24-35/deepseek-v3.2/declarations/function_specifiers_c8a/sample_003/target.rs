use std::process;

fn die0() {
    process::exit(0);
}

fn pick(x: bool) -> i32 {
    if x {
        123
    } else {
        die0();
    }
}

fn main() {
    let v = pick(false);
    // v is unused, but we don't need to explicitly ignore it
    // Return 1 would never happen because process::exit terminates the program
}