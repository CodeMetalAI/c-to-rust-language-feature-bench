fn main() {
    let s: &[u8] = b"12345\0";
    let mut i = 0;
    let mut acc = 0;
    while s[i] != 0 {
        acc += 1;
        i += 1;
    }
    std::process::exit(if acc == 5 { 0 } else { 1 });
}