fn main() {
    let mut x = 0;

    // In Rust, we can simulate the goto behavior using a loop that breaks immediately
    // The goto skips x = 1, so we just don't execute it
    'block: {
        break 'block;
        #[allow(unreachable_code)]
        {
            x = 1;
        }
    }

    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}