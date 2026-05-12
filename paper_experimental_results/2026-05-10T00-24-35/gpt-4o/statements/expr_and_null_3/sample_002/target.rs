fn main() {
    let mut x = 0;

    {
        // Simulating a goto by using a block and an unconditional break
        'outer: loop {
            break 'outer;
            x = 1;
        }
    }

    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}