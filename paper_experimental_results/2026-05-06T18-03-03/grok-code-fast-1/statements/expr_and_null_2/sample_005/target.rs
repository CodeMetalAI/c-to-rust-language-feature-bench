fn main() {
    let s: &[u8] = b"12345\0";
    let mut ptr = 0;
    let mut acc = 0;
    while s[ptr] != 0 {
        acc += 1;
        ptr += 1;
    }
    let exit_code = if acc == 5 { 0 } else { 1 };
    std::process::exit(exit_code);
}