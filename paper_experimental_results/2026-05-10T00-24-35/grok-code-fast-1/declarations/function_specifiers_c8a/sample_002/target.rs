fn die0() -> ! {
    std::process::exit(0);
}

fn pick(x: i32) -> i32 {
    if x != 0 {
        123
    } else {
        die0()
    }
}

fn main() {
    let v = pick(0);
    let _ = v;
    // This line is unreachable, but matches the C code structure
    std::process::exit(1);
}