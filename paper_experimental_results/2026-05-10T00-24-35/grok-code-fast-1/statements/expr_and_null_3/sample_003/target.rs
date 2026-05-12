fn main() {
    let mut x = 0;

    {
        // In C, goto end; skips x = 1;
        // In Rust, we simulate by not executing the assignment
        // x = 1;  // skipped
    }
    // end:;  // label would be here

    x += 1;
    let ret = if x == 1 { 0 } else { 1 };
    std::process::exit(ret);
}