fn main() -> i32 {
    let mut x = 0;

    {
        // Simulate the goto by skipping the assignment
        // to x
        // x = 1; // This line is equivalent to the unreachable code after goto
    }

    x += 1;
    return if x == 1 { 0 } else { 1 };
}