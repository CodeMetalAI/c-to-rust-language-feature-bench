fn main() {
    let mut x = 0;

    // Rust doesn't have goto, but we can simulate this behavior
    // The goto skips x = 1, so we just don't execute it
    // In the original: goto end jumps over x = 1 to end label
    
    // The block with goto effectively does nothing to x
    // because the goto skips the x = 1 assignment

    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}