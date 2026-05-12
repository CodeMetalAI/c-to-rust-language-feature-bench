fn main() {
    let mut x = 0;

    {
        // Simulating the goto by directly jumping to the label's position with a block.
        {
            // This block would contain the code skipped by the goto.
            // `x = 1;` is skipped due to the goto.
        }
        // Label `end:` is effectively here at the end of the block.
    }

    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}