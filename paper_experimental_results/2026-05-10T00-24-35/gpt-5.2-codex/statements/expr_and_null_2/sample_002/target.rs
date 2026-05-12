fn main() {
    let bytes = b"12345\0";
    let mut idx = 0usize;
    let mut acc = 0;
    while bytes[idx] != 0 {
        idx += 1;
        acc += 1;
    }
    if acc == 5 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}