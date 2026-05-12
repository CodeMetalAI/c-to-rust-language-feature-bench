fn main() {
    let mut x = 0;

    // In Rust, we can't use goto, but we can achieve the same effect
    // The goto skips x = 1, so we just don't execute it
    // Using a loop with break to simulate the goto behavior
    loop {
        break;
        #[allow(unreachable_code)]
        {
            x = 1;
        }
    }

    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}