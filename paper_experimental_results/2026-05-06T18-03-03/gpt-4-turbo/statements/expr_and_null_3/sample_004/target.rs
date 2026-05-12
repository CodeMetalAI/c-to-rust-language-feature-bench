fn main() {
    let mut x = 0;

    {
        // Simulating the `goto end;` behavior by skipping the assignment
        x = x; // This line does nothing but is here to represent the skipped code
    }

    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}