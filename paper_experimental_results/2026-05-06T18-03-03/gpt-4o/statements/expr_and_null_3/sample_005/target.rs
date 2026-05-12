fn main() {
    let mut x = 0;

    {
        // The equivalent of a goto in Rust is to use a block and break out of it.
        'end: {
            break 'end;
            x = 1;
        }
    }

    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}