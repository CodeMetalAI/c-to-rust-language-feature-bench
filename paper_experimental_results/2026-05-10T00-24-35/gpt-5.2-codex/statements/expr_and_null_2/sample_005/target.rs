fn main() {
    let s = b"12345\0";
    let mut acc = 0;
    let mut i = 0;
    while s[i] != 0 {
        acc += 1;
        i += 1;
    }
    let exit_code = if acc == 5 { 0 } else { 1 };
    std::process::exit(exit_code);
}