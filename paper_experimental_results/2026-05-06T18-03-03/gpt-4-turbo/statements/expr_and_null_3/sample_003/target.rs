fn main() {
    let mut x = 0;

    {
        // Simulating the goto by directly jumping to the label section
        // `x = 1;` is never executed due to the goto in C++
        {
            // This block is where `x = 1;` would be, but it's skipped
        }
        // Label `end:` is effectively here
    }

    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}