fn main() {
    let mut acc: i32 = 0;

    // Simulate `goto first_time;` before the loop.
    let mut i: i32 = 0;
    let mut jumped = true;

    while i < 5 {
        if !jumped {
            acc += 1;
            if acc < 15 {
                acc = -15;
            }
        }

        // first_time:
        acc = 15;
        jumped = false;
        i += 1;
        continue;
    }

    let exit_code = if acc == 20 { 0 } else { 1 };
    std::process::exit(exit_code);
}