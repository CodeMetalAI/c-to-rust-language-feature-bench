fn main() {
    let mut x = 0;

    // In Rust, we can simulate the goto behavior using a loop that we break out of
    // The goto skips x = 1, so we just don't execute it
    // Since goto end jumps to end:, the x = 1 is never executed
    
    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}