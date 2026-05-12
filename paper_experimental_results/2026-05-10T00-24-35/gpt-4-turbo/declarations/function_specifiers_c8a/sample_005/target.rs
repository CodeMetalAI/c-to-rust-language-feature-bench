use std::process::exit;

fn die0() -> ! {
    exit(0)
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
}