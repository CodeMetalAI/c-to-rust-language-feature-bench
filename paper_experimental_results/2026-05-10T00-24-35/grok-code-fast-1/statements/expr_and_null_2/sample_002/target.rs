fn main() {
    let s: &[u8] = b"12345\0";
    let mut s_ptr = 0;
    let mut acc = 0;
    while s[s_ptr] != 0 {
        s_ptr += 1;
        acc += 1;
    }
    std::process::exit(if acc == 5 { 0 } else { 1 });
}