fn main() -> i32 {
    let mut x = 0;

    {
        let _ = 1; // equivalent to goto, does nothing in Rust
    }

    x += 1;
    return if x == 1 { 0 } else { 1 };
}