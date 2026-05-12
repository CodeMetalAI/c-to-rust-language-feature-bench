fn main() {
    let mut x = 0;

    // In Rust, we can't use goto, so we simulate the control flow
    // The goto skips x = 1, so we just don't execute it
    // The label 'end:;' is reached directly

    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}