fn main() -> i32 {
    let mut x = 0;

    {
        // This block is skipped due to the goto statement
        // However, we need to declare a label to match the original code
        let _ = || {};
        x = 1;
    }

    x += 1;

    if x == 1 {
        0
    } else {
        1
    }
}