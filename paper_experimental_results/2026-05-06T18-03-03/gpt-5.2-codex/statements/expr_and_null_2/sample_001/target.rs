fn main() {
    let s: &[u8] = b"12345\0";
    let mut acc = 0;
    let mut idx = 0;
    while s[idx] != 0 {
        acc += 1;
        idx += 1;
    }
    std::process::exit(if acc == 5 { 0 } else { 1 });
}