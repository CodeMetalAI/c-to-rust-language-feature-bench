fn main() {
    let mut x = 0;

    {
        // Mimicking the `goto end;` behavior by simply not executing `x = 1;`.
        // In Rust we don't have `goto`, so we skip the assignment.
    }

    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}