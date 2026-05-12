fn die0() -> ! {
    std::process::exit(0);
}

fn pick(x: bool) -> i32 {
    if x {
        123
    } else {
        die0()
    }
}

fn main() {
    let v = pick(false);
    let _ = v;
    std::process::exit(1);
}