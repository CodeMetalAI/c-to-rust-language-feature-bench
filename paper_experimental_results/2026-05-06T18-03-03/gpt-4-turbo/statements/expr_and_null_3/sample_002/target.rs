fn main() {
    let mut x = 0;

    {
        // Simulating the goto by directly jumping to the label section
        // `x = 1;` is never executed due to the goto in C++
    }

    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}