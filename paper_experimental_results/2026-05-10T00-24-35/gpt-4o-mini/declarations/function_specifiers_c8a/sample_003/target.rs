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

fn main() -> i32 {
    let v = pick(0);
    let _ = v; // to avoid unused variable warning
    1
}