fn main() {
    let mut x = 0;

    {
        // Simulate the `goto end;` by skipping over the assignment to `x`
        // The `end` label is effectively simulated by the end of this block.
    }

    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}