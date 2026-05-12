fn main() {
    let mut x = 0;

    {
        // Rust does not allow labels within blocks in the same way as C/C++.
        // Instead, we can use a loop with an early break for similar control flow.
        loop {
            break; // Equivalent to `goto end;` in C/C++
            // x = 1; This line is unreachable due to the break
        }
    }

    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}