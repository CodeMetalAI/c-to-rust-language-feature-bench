fn main() {
    let s = "12345\0";
    let bytes = s.as_bytes();
    let mut acc = 0;
    let mut i = 0;
    while bytes[i] != 0 {
        acc += 1;
        i += 1;
    }
    std::process::exit(if acc == 5 { 0 } else { 1 });
}