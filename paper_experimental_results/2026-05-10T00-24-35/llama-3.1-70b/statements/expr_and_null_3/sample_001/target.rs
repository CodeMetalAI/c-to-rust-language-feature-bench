fn main() {
    let mut x = 0;

    {
        // Rust doesn't support goto, but we can achieve similar behavior using a loop
        loop {
            break;
            x = 1;
        }
    }

    x += 1;
    if x == 1 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}