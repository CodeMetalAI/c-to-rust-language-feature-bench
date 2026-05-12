fn main() {
    let mut x = 0;

    // In Rust, we can simulate the goto behavior using a loop that breaks immediately
    // The goto skips x = 1, so we just don't execute it
    // The label 'end:;' is reached directly via goto, skipping x = 1
    
    // Since goto end skips x = 1, we don't assign x = 1
    // x remains 0

    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}