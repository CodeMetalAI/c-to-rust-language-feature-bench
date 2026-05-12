fn main() {
    let mut x = 0;

    {
        // This block is unreachable, but Rust doesn't optimize it away
        // like C does. We can use the `unreachable!` macro to indicate
        // that this block is unreachable.
        unreachable!();
        x = 1;
    }

    x += 1;
    println!("{}", (x == 1) as i32);
}