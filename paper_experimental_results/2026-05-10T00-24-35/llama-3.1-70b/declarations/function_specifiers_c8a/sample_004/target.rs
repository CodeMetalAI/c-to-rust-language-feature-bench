use std::process;
use std::panic;

fn die0() -> ! {
    process::exit(0);
}

fn pick(x: bool) -> i32 {
    if x { 123 } else { die0() }
}

fn main() {
    let v = pick(false);
    drop(v);
    std::process::exit(1);
}