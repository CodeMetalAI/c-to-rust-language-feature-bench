fn main() -> i32 {
    let mut x = 0;

    {
        // Simulating the goto by jumping to the end of the block
        // No-op in the middle block
    }

    x += 1;
    if x == 1 {
        0
    } else {
        1
    }
}