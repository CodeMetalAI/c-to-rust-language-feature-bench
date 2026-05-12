fn main() {
    let mut x = 0;
    // Simulate the block: goto end skips x = 1;
    x += 1;
    let exit_code = if x == 1 { 0 } else { 1 };
    std::process::exit(exit_code);
}