use std::process::ExitCode;

fn die0() -> i32 {
    std::process::exit(0)
}

fn pick(x: i32) -> i32 {
    if x != 0 {
        123
    } else {
        die0()
    }
}

fn main() -> ExitCode {
    let v = pick(0);
    let _ = v;
    ExitCode::from(1)
}