fn main() {
    let mut x = 0;

    {
        // This block will be skipped because it's unreachable
        // due to the goto statement
        x = 1;
    }

    x += 1;
    println!("{}", (x == 1) as i32);
}