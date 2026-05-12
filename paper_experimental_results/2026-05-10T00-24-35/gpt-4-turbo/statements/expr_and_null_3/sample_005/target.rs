fn main() {
    let mut x = 0;

    {
        // simulate the goto by simply skipping the assignment
        // Rust does not support goto so we use Rust control flow constructs
    }

    x += 1;

    // Rust's if is an expression, so it can directly control the return value
    if x == 1 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}