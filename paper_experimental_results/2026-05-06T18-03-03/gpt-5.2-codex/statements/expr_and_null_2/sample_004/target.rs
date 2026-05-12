fn main() {
    let s = "12345";
    let mut acc = 0;
    for _b in s.as_bytes() {
        acc += 1;
    }
    let code = if acc == 5 { 0 } else { 1 };
    std::process::exit(code);
}