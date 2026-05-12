fn main() -> i32 {
    let mut x = 0;

    // The goto logic can be simulated with a block
    {
        x = 1; // This line will not execute due to the immediate jump
    }

    x += 1;
    return if x == 1 { 0 } else { 1 };
}