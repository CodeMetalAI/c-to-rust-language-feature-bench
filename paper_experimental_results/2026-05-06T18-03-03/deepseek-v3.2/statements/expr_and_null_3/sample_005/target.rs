fn main() {
    let mut x = 0;

    {
        goto_end();
        x = 1;
        end();
    }

    x += 1;
    let exit_code = if x == 1 { 0 } else { 1 };
    std::process::exit(exit_code);
}

fn goto_end() {
    // Rust doesn't have goto, so we simulate the jump by returning early
    return;
}

fn end() {
    // Placeholder for the label, no action needed
}