fn main() {
    let s = b"12345\0";
    let mut idx = 0usize;
    let mut acc = 0;
    while s[idx] != b'\0' {
        idx += 1;
        acc += 1;
    }
    std::process::exit(if acc == 5 { 0 } else { 1 });
}