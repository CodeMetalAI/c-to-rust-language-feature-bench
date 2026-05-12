fn main() {
    let s = b"12345\0";
    let mut idx: usize = 0;
    let mut acc: i32 = 0;
    while s[idx] != 0 {
        idx += 1;
        acc += 1;
    }
    std::process::exit(if acc == 5 { 0 } else { 1 });
}