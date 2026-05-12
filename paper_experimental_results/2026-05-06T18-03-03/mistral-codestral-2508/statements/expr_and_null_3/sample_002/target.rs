fn main() {
    let mut x = 0;

    {
        // In Rust, we can't use goto, so we use a loop with a break statement to simulate the behavior
        loop {
            break;
            x = 1;
        }
    }

    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}