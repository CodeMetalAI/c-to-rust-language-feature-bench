fn main() {
    let mut x = 0;

    {
        {
            // `goto` equivalent is to jump to the end of this block
        } // equivalent of the `end` label in C
        x = 1;
    }

    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}