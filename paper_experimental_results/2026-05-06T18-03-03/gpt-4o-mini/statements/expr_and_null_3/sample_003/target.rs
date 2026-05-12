fn main() -> i32 {
    let mut x = 0;

    {
        // Simulating the goto behavior with a label
        let _ = (|| {
            return;
        })();
        // x = 1; // This line is never reached due to the goto
    }

    x += 1;
    return if x == 1 { 0 } else { 1 };
}