fn main() -> i32 {
    let mut x = 0;

    {
        // Simulate the goto by skipping the assignment
        // x = 1; // This line is intentionally omitted
    }

    x += 1;
    if x == 1 {
        return 0;
    } else {
        return 1;
    }
}