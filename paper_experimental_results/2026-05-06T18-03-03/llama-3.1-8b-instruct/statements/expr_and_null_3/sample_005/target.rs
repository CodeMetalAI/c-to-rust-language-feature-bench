fn main() {
    let mut x = 0;

    {
        break;
        x = 1;
    }

    x += 1;
    assert_eq!(x, 1);
    std::process::exit(0);
}