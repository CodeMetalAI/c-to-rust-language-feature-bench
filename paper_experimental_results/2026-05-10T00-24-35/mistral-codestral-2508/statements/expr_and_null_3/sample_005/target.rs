fn main() {
    let mut x = 0;

    {
        let mut _end = false;
        if !_end {
            x = 1;
            _end = true;
        }
    }

    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}