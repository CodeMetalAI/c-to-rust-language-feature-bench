fn main() -> i32 {
    let mut x = 0;

    {
        // Simulating the goto behavior with a label
        // The code after the label will not execute
        // goto end; is effectively a no-op here
        // x = 1; is skipped
    }

    x += 1;
    return if x == 1 { 0 } else { 1 };
}