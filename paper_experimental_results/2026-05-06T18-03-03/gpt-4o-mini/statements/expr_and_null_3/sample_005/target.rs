fn main() -> i32 {
    let mut x = 0;

    {
        // Simulating the goto behavior with a label
        let _ = ();
        // The line `x = 1;` is effectively skipped
    }

    x += 1;
    return if x == 1 { 0 } else { 1 };
}