fn main() {
    let mut x = 0;

    {
        x = 1;
    }

    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}