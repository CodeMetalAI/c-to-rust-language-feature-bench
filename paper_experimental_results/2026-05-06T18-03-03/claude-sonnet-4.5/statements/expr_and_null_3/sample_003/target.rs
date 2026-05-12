fn main() {
    let mut x = 0;

    {
        'end: loop {
            x = 1;
            break 'end;
        }
    }

    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}