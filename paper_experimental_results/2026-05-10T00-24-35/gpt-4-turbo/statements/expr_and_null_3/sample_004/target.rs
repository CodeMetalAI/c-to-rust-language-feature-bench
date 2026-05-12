fn main() {
    let mut x = 0;

    {
        x = 1;
    }

    x += 1;

    if x == 2 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}