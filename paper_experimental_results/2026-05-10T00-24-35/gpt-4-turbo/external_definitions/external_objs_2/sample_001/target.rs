fn main() {
    let mut i = [0; 1];

    if i[0] != 0 {
        std::process::exit(1);
    }
    i[0] = 7;
    if i[0] != 7 {
        std::process::exit(2);
    }
}