fn main() {
    let mut i = 0;
    if i!= 0 {
        return 1;
    }
    i = 7;
    if i!= 7 {
        return 2;
    }
    std::process::exit(0);
}