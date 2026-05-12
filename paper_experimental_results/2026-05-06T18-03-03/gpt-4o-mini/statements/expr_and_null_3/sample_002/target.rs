fn main() -> i32 {
    let mut x = 0;

    {
        // Simulating the goto behavior with a label
        // The code after the label will not be executed
        // since we are jumping to `end`.
        goto_end();
    }

    x += 1;
    return if x == 1 { 0 } else { 1 };
}

fn goto_end() {
    // This function simulates the behavior of the goto by doing nothing.
}