fn _exit(status: i32) -> ! {
    std::process::exit(status);
}

fn die0() -> ! {
    _exit(0);
}

fn pick(x: i32) -> i32 {
    if x != 0 {
        123
    } else {
        die0()
    }
}

fn main() {
    let _ = pick(0);
    std::process::exit(1);
}