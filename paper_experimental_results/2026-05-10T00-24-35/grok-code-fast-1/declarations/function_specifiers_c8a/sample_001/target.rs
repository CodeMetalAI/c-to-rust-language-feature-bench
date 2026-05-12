fn die0() -> ! {
    std::process::exit(0);
}

fn pick(x: i32) -> i32 {
    if x != 0 { 123 } else { die0() }
}

fn main() {
    let _v = pick(0);
    // Unreachable code follows, equivalent to C's unreachable return 1;
    // But since we exit above, this isn't reached, preserving the behavior.
}